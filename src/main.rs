#[macro_use]
extern crate enum_primitive_derive;
extern crate dotenv;
extern crate log;
extern crate num_traits;

mod codec;
mod config;
mod discovery;
mod helpers;
mod models;

use crate::{
    codec::SofarCodec,
    config::Config,
    discovery::{Device, Entity},
};
use futures_util::StreamExt;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use serde_json::{Map, Value};
use std::error::Error;
use tokio::{
    io,
    net::{TcpListener, TcpStream},
    task,
};
use tokio_util::codec::Framed;

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    log::info!("Starting sofar-mqtt!");

    let config = serde_env::from_env::<Config>().unwrap();
    let listener = TcpListener::bind(format!("0.0.0.0:{0}", config.tcp_port)).await?;
    log::info!("Waiting for connections");

    loop {
        let (mut socket, connection) = listener.accept().await?;
        task::spawn(async move {
            log::info!(
                "Spawning connection handler #{0} for {1}:{2}",
                tokio::task::id(),
                connection.ip(),
                connection.port()
            );
            match process_socket(&mut socket).await {
                Ok(_) => log::info!("Finished connection #{0}", tokio::task::id()),
                Err(_) => log::error!("Closing connection #{0} with error", tokio::task::id()),
            }
        });
    }
}

async fn process_socket(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let config = serde_env::from_env::<Config>()?;
    let mut framed_stream = Framed::new(stream, SofarCodec::default());

    let mut mqttoptions = MqttOptions::new("sofar-mqtt", config.mqtt_host, config.mqtt_port);
    if config.mqtt_user.is_some() && config.mqtt_password.is_some() {
        mqttoptions.set_credentials(config.mqtt_user.unwrap(), config.mqtt_password.unwrap());
    }

    while let Some(frame) = framed_stream.next().await {
        log::info!("Received frame");
        match frame {
            Err(err) => log::error!("Error while reading frame ({:#?})", err),
            Ok(models::SofarMessage::Data(data)) => {
                log::info!("Preparing data for MQTT broker");
                let map =
                    serde_json::from_value::<Map<String, Value>>(serde_json::to_value(&data)?)?;

                let (mqtt_client, mut eventloop) = AsyncClient::new(mqttoptions.to_owned(), 10);

                log::info!("Sending data to MQTT broker");

                for (key, value) in map.iter() {
                    let prefix = "sofar".to_string();

                    let state_payload = match value {
                        Value::String(a) => a.trim().to_owned(),
                        a => a.to_string(),
                    };

                    mqtt_client
                        .publish(
                            format!("{prefix}/{key}"),
                            QoS::AtMostOnce,
                            false,
                            state_payload,
                        )
                        .await?;

                    match eventloop.poll().await {
                        Ok(_) => {}
                        Err(err) => {
                            log::error!("Error sending data to MQTT broker ({:?})", err);
                            return Err(Box::new(err));
                        }
                    }

                    mqtt_client
                        .publish(
                            format!("homeassistant/sensor/sofar/{key}/config"),
                            QoS::AtMostOnce,
                            false,
                            serde_json::to_string(&prepare_discovery_payload(key, prefix))?,
                        )
                        .await?;

                    match eventloop.poll().await {
                        Ok(_) => {}
                        Err(err) => {
                            log::error!("Error sending data to MQTT broker ({:?})", err);
                            return Err(Box::new(err));
                        }
                    }
                }

                log::info!("Disconnecting from MQTT broker");
                mqtt_client.disconnect().await?
            }
            _ => {}
        }
    }

    log::info!("Finishing TCP connection");
    Ok(())
}

fn prepare_discovery_payload(key: &str, prefix: String) -> Entity {
    let device = Device::default();

    match key {
        "current_power" => Entity::power_sensor(key.to_string(), prefix, device),
        "inverter_temperature" => Entity::new_temperature_entity(key.to_string(), prefix, device),
        "daily_energy" | "total_energy" => Entity::energy_sensor(key.to_string(), prefix, device),
        _ => Entity::generic_sensor(key.to_string(), prefix, device),
    }
}
