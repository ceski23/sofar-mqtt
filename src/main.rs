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
mod publisher;

use crate::{codec::SofarCodec, config::Config, publisher::MqttPublisher};
use futures_util::StreamExt;
use rumqttc::MqttOptions;
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

                let mut publisher = MqttPublisher::new("sofar".to_string());

                log::info!("Sending data to MQTT broker");

                for (key, value) in map.iter() {
                    match publisher.publish_state(key, value).await {
                        Ok(_) => {}
                        Err(err) => {
                            log::error!("Error sending data to MQTT broker ({:?})", err);
                            return Err(err);
                        }
                    }

                    match publisher.pubish_discovery(key).await {
                        Ok(_) => {}
                        Err(err) => {
                            log::error!("Error sending data to MQTT broker ({:?})", err);
                            return Err(err);
                        }
                    }
                }

                log::info!("Disconnecting from MQTT broker");
                publisher.mqtt_client.disconnect().await?;
            }
            _ => {}
        }
    }

    log::info!("Finishing TCP connection");
    Ok(())
}
