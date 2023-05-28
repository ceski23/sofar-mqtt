#[macro_use]
extern crate enum_primitive_derive;
extern crate dotenv;
extern crate log;
extern crate num_traits;

mod codec;
mod config;
mod helpers;
mod homeassistant;
mod models;
mod publisher;

use crate::{
    codec::SofarCodec,
    config::Config,
    homeassistant::{Attributes, Device},
    models::{MessageData, SensorsData, SofarResponseMessage},
    publisher::MqttPublisher,
};
use futures_util::{SinkExt, StreamExt};
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
    let mut inverter_ip: Option<String> = None;
    let mut module_version: Option<String> = None;
    let mut framed_stream = Framed::new(stream, SofarCodec::default());

    while let Some(frame) = framed_stream.next().await {
        match frame {
            Err(err) => log::error!("Error while reading frame ({:#?})", err),
            Ok(message) => {
                log::info!("Received frame of type {:?}", message.message_type);
                let response_message = SofarResponseMessage::new(&message);

                match message.data {
                    MessageData::Data(data) => {
                        log::info!("{:?}", data);
                        log::info!("Responding with {:?}", response_message);
                        framed_stream.send(response_message).await?;

                        let device = Device {
                            configuration_url: inverter_ip
                                .clone()
                                .map(|ip| format!("http://{}/index_cn.html", ip)),
                            identifiers: format!(
                                "sofar_{}",
                                &data.inverter_serial_number.trim().to_lowercase()
                            ),
                            manufacturer: String::from("Sofar"),
                            model: format!("{}", &data.inverter_serial_number.trim()),
                            name: format!("Sofar {}", &data.inverter_serial_number.trim()),
                            sw_version: module_version.to_owned(),
                        };

                        let attributes = Attributes {
                            country_code: data.country_code,
                            day: data.day,
                            hardware_version: format!("{}", data.hardware_version),
                            hour: data.hour,
                            inverter_firmware: format!("{}", data.inverter_firmware),
                            main_inverter_firmware: format!("{}", data.main_inverter_firmware),
                            minute: data.minute,
                            month: data.month,
                            second: data.second,
                            slave_inverter_firmware: format!("{}", data.slave_inverter_firmware),
                            timestamp: data.timestamp,
                            total_time: data.total_time,
                            year: data.year,
                        };

                        let mut publisher = MqttPublisher::new(format!(
                            "sofar_{}",
                            &data.inverter_serial_number.trim().to_lowercase()
                        ));

                        log::info!("Sending data to MQTT broker");
                        log::info!("Sending attributes ({:?})", attributes);

                        match publisher
                            .publish_state(
                                format!("attributes"),
                                &serde_json::to_value(&attributes)?,
                            )
                            .await
                        {
                            Ok(_) => {}
                            Err(err) => {
                                log::error!("Error sending data to MQTT broker ({:?})", err);
                                break;
                            }
                        }

                        let sensors_data = SensorsData {
                            current_power: data.current_power,
                            daily_energy: data.daily_energy,
                            inventer_temperature: data.inventer_temperature,
                            inverter_status: data.inverter_status,
                            total_energy: data.total_energy,
                        };

                        log::info!("Sending data ({:?})", sensors_data);

                        for (key, value) in serde_json::from_value::<Map<String, Value>>(
                            serde_json::to_value(&sensors_data)?,
                        )? {
                            match publisher
                                .publish_state(format!("state/{key}"), &value)
                                .await
                            {
                                Ok(_) => {}
                                Err(err) => {
                                    log::error!("Error sending data to MQTT broker ({:?})", err);
                                    break;
                                }
                            }

                            match publisher.pubish_discovery(key.to_owned(), &device).await {
                                Ok(_) => {}
                                Err(err) => {
                                    log::error!("Error sending data to MQTT broker ({:?})", err);
                                    break;
                                }
                            }
                        }

                        log::info!("Disconnecting from MQTT broker");
                        publisher.mqtt_client.disconnect().await?;
                    }
                    MessageData::Hello(data) => {
                        log::info!("Decoded: {:?}", data);

                        inverter_ip = Some(
                            data.local_ip_address
                                .trim_matches(char::from(0))
                                .to_string(),
                        );
                        module_version =
                            Some(data.module_version.trim_matches(char::from(0)).to_string());

                        log::info!("Responding with {:?}", response_message);
                        framed_stream.send(response_message).await?;
                    }
                    data => {
                        log::info!("Decoded: {:?}", data);
                        log::info!("Responding with {:?}", response_message);
                        framed_stream.send(response_message).await?;
                    }
                }
            }
        }
    }

    log::info!("Finishing TCP connection");
    Ok(())
}
