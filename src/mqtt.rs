use crate::{
    config::{Config, MQTT_CLIENT_ID},
    homeassistant::{Device, Entity, EntityType},
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
        let mut mqttoptions = MqttOptions::new(MQTT_CLIENT_ID, config.mqtt_host, config.mqtt_port);

        if let (Some(mqtt_user), Some(mqtt_password)) = (config.mqtt_user, config.mqtt_password) {
            mqttoptions.set_credentials(mqtt_user, mqtt_password);
        }

        let (mqtt_client, event_loop) = AsyncClient::new(mqttoptions.to_owned(), 10);

        MqttPublisher {
            mqtt_client,
            event_loop,
            prefix,
        }
    }

    pub async fn publish_state(&mut self, entity: &EntityType) -> Result<(), Box<dyn Error>> {
        let (payload, name) = match entity {
            EntityType::EnergySensor { name, value } => (value.to_string(), name.to_string()),
            EntityType::PowerSensor { name, value } => (value.to_string(), name.to_string()),
            EntityType::TemperatureSensor { name, value } => (value.to_string(), name.to_string()),
            EntityType::GenericSensor { name, value } => (value.to_string(), name.to_string()),
            EntityType::GenericDiscreteSensor { name, value } => {
                (value.to_string(), name.to_string())
            }
        };

        self.mqtt_client
            .publish(
                format!("{}/state/{}", self.prefix, name),
                QoS::AtMostOnce,
                true,
                payload,
            )
            .await?;

        self.event_loop.poll().await?;
        Ok(())
    }

    pub async fn publish_discovery(
        &mut self,
        entity: &EntityType,
        device: &Device,
    ) -> Result<(), Box<dyn Error>> {
        let (payload, name) = match entity {
            EntityType::EnergySensor { name, .. } => (
                Entity::energy_sensor(name.to_string(), self.prefix.to_owned(), device),
                name.to_string(),
            ),
            EntityType::PowerSensor { name, .. } => (
                Entity::power_sensor(name.to_string(), self.prefix.to_owned(), device),
                name.to_string(),
            ),
            EntityType::TemperatureSensor { name, .. } => (
                Entity::temperature_entity(name.to_string(), self.prefix.to_owned(), device),
                name.to_string(),
            ),
            EntityType::GenericSensor { name, .. } => (
                Entity::generic_sensor(name.to_string(), self.prefix.to_owned(), device, false),
                name.to_string(),
            ),
            EntityType::GenericDiscreteSensor { name, .. } => (
                Entity::generic_sensor(name.to_string(), self.prefix.to_owned(), device, true),
                name.to_string(),
            ),
        };

        self.mqtt_client
            .publish(
                format!("homeassistant/sensor/{}/{name}/config", device.identifiers),
                QoS::AtMostOnce,
                true,
                serde_json::to_string(&payload)?,
            )
            .await?;

        self.event_loop.poll().await?;
        Ok(())
    }

    pub async fn publish_attributes(&mut self, value: &Value) -> Result<(), Box<dyn Error>> {
        let payload = match value {
            Value::String(a) => a.trim().to_owned(),
            a => a.to_string(),
        };

        self.mqtt_client
            .publish(
                format!("{}/attributes", self.prefix),
                QoS::AtMostOnce,
                true,
                payload,
            )
            .await?;

        self.event_loop.poll().await?;
        Ok(())
    }
}
