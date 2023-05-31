#[macro_use]
extern crate enum_primitive_derive;
extern crate dotenv;
extern crate log;
extern crate num_traits;

mod codec;
mod config;
mod homeassistant;
mod messages;
mod mqtt;
mod serde_helpers;

use crate::{
    codec::SofarCodec,
    config::Config,
    homeassistant::{entities_from_data, Attributes, Device},
    messages::{IncomingMessageData, SofarMessage},
    mqtt::MqttPublisher,
};
use futures_util::{SinkExt, StreamExt};
use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};
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

                let response_message =
                    SofarMessage::from_incoming_message(&message, current_timestamp());

                match message.data {
                    IncomingMessageData::Data(data) => {
                        log::info!("{:?}", data);
                        log::info!("Responding with {:?}", response_message);
                        framed_stream.send(response_message).await?;

                        let device = Device {
                            configuration_url: inverter_ip
                                .clone()
                                .map(|ip| format!("http://{}/index_cn.html", ip)),
                            identifiers: format!(
                                "sofar_{}",
                                data.inverter_serial_number.trim().to_lowercase()
                            ),
                            manufacturer: String::from("Sofar"),
                            model: data.inverter_serial_number.trim().to_string(),
                            name: format!("Sofar {}", data.inverter_serial_number.trim()),
                            sw_version: module_version.to_owned(),
                        };
                        let attributes = Attributes::from_data(&data);
                        let entities = entities_from_data(&data);

                        let mut mqtt_publisher = MqttPublisher::new(format!(
                            "sofar_{}",
                            data.inverter_serial_number.trim().to_lowercase()
                        ));

                        log::info!("Sending data to MQTT broker");
                        log::info!("Sending attributes ({:?})", attributes);

                        mqtt_publisher
                            .publish_attributes(&serde_json::to_value(&attributes)?)
                            .await
                            .unwrap_or_else(|err| {
                                log::error!("Error sending data to MQTT broker ({:?})", err)
                            });

                        log::info!("Sending data ({:?})", entities);

                        for entity in entities {
                            match mqtt_publisher.publish_discovery(&entity, &device).await {
                                Ok(_) => {}
                                Err(err) => {
                                    log::error!("Error sending data to MQTT broker ({:?})", err);
                                    break;
                                }
                            }

                            match mqtt_publisher.publish_state(&entity).await {
                                Ok(_) => {}
                                Err(err) => {
                                    log::error!("Error sending data to MQTT broker ({:?})", err);
                                    break;
                                }
                            }
                        }

                        mqtt_publisher.event_loop.poll().await?;

                        log::info!("Disconnecting from MQTT broker");
                        mqtt_publisher.mqtt_client.disconnect().await?;
                    }
                    IncomingMessageData::Hello(data) => {
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

fn current_timestamp() -> u32 {
    u32::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    )
    .unwrap_or_default()
}
