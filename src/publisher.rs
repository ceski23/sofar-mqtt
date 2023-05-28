use crate::{
    config::Config,
    homeassistant::{Device, Entity},
};
use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};
use serde_json::Value;
use std::error::Error;

pub struct MqttPublisher {
    pub mqtt_client: AsyncClient,
    pub event_loop: EventLoop,
    prefix: String,
}

impl MqttPublisher {
    pub(crate) fn new(prefix: String) -> Self {
        let config = serde_env::from_env::<Config>().unwrap();
        let mut mqttoptions = MqttOptions::new("sofar-mqtt", config.mqtt_host, config.mqtt_port);

        if config.mqtt_user.is_some() && config.mqtt_password.is_some() {
            mqttoptions.set_credentials(config.mqtt_user.unwrap(), config.mqtt_password.unwrap());
        }

        let (mqtt_client, event_loop) = AsyncClient::new(mqttoptions.to_owned(), 10);

        MqttPublisher {
            mqtt_client,
            event_loop,
            prefix,
        }
    }

    pub async fn publish_state(
        &mut self,
        key: String,
        value: &Value,
    ) -> Result<(), Box<dyn Error>> {
        let state_payload = match value {
            Value::String(a) => a.trim().to_owned(),
            a => a.to_string(),
        };

        self.mqtt_client
            .publish(
                format!("{}/{}", self.prefix, key),
                QoS::AtMostOnce,
                false,
                state_payload,
            )
            .await?;

        self.event_loop.poll().await?;
        Ok(())
    }

    pub async fn pubish_discovery(
        &mut self,
        key: String,
        device: &Device,
    ) -> Result<(), Box<dyn Error>> {
        let payload = self.prepare_discovery_payload(key.as_str(), device);

        self.mqtt_client
            .publish(
                format!("homeassistant/sensor/{}/{key}/config", device.identifiers),
                QoS::AtMostOnce,
                false,
                serde_json::to_string(&payload)?,
            )
            .await?;

        self.event_loop.poll().await?;
        Ok(())
    }

    pub(crate) fn prepare_discovery_payload(&mut self, key: &str, device: &Device) -> Entity {
        match key {
            "current_power" => {
                Entity::power_sensor(key.to_string(), self.prefix.to_owned(), device)
            }
            "inverter_temperature" | "logger_temperature" => {
                Entity::new_temperature_entity(key.to_string(), self.prefix.to_owned(), device)
            }
            "daily_energy" | "total_energy" => {
                Entity::energy_sensor(key.to_string(), self.prefix.to_owned(), device)
            }
            _ => Entity::generic_sensor(key.to_string(), self.prefix.to_owned(), device),
        }
    }
}
