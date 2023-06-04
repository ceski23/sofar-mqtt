#[macro_use]
extern crate enum_primitive_derive;
extern crate dotenv;
extern crate num_traits;

mod codec;
mod config;
mod homeassistant;
mod logger;
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
use anyhow::Context;
use futures_util::{SinkExt, StreamExt};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::{
    net::{TcpListener, TcpStream},
    task,
};
use tokio_util::codec::Framed;
use tracing::{error, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().context("Couldn't load env variables")?;
    logger::init_logger()?;

    info!("Starting sofar-mqtt v{}", env!("CARGO_PKG_VERSION"));

    let config = serde_env::from_env::<Config>()?;
    let listener = TcpListener::bind(format!("0.0.0.0:{0}", config.tcp_port)).await?;
    info!("Waiting for connections");

    loop {
        let (mut socket, _connection) = listener.accept().await?;
        task::spawn(async move {
            let result = process_socket(&mut socket).await.with_context(|| {
                format!(
                    "Finished connection to {} with error",
                    &socket.peer_addr().unwrap(),
                )
            });

            if let Err(err) = result {
                error!("{err:?}")
            }
        });
    }
}

#[tracing::instrument(
    skip_all,
    fields(ip = %stream.peer_addr().unwrap()),
)]
async fn process_socket(stream: &mut TcpStream) -> anyhow::Result<()> {
    info!("Spawning connection handler");

    let mut inverter_ip: Option<String> = None;
    let mut module_version: Option<String> = None;
    let mut framed_stream = Framed::new(stream, SofarCodec::default());

    while let Some(frame) = framed_stream.next().await {
        match frame {
            Err(err) => error!("Error while reading frame ({:#?})", err),
            Ok(message) => {
                info!("Received frame of type {:?}", message.message_type);

                let response_message =
                    SofarMessage::from_incoming_message(&message, current_timestamp());

                match message.data {
                    IncomingMessageData::Data(data) => {
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

                        info!("Sending data to MQTT broker");
                        info!("Sending attributes ({:?})", attributes);

                        mqtt_publisher
                            .publish_attributes(&serde_json::to_value(&attributes)?)
                            .await?;

                        info!("Sending data ({:?})", entities);

                        for entity in entities {
                            mqtt_publisher.publish_discovery(&entity, &device).await?;
                            mqtt_publisher.publish_state(&entity).await?;
                        }

                        mqtt_publisher.event_loop.poll().await?;

                        info!("Disconnecting from MQTT broker");
                        mqtt_publisher.mqtt_client.disconnect().await?;
                    }
                    IncomingMessageData::Hello(data) => {
                        inverter_ip = Some(
                            data.local_ip_address
                                .trim_matches(char::from(0))
                                .to_string(),
                        );
                        module_version =
                            Some(data.module_version.trim_matches(char::from(0)).to_string());
                        framed_stream.send(response_message).await?;
                    }
                    _ => {
                        framed_stream.send(response_message).await?;
                    }
                }
            }
        }
    }

    info!("Finishing TCP connection");
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
