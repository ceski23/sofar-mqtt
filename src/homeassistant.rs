use crate::messages::Data;

#[derive(serde::Serialize, Clone)]
pub struct Device {
    pub configuration_url: Option<String>,
    pub identifiers: String,
    pub manufacturer: String,
    pub model: String,
    pub name: String,
    pub sw_version: Option<String>,
}

#[derive(serde::Serialize, Debug)]
pub struct Attributes {
    pub timestamp: u32,
    pub total_time: u32,
    pub inverter_firmware: String,
    pub hardware_version: String,
    pub country_code: u16,
    pub main_inverter_firmware: String,
    pub slave_inverter_firmware: String,
    pub year: u8,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Attributes {
    pub fn from_data(data: &Data) -> Self {
        Attributes {
            country_code: data.country_code,
            day: data.day,
            hardware_version: data.hardware_version.to_string(),
            hour: data.hour,
            inverter_firmware: data.inverter_firmware.to_string(),
            main_inverter_firmware: data.main_inverter_firmware.to_string(),
            minute: data.minute,
            month: data.month,
            second: data.second,
            slave_inverter_firmware: data.slave_inverter_firmware.to_string(),
            timestamp: data.timestamp,
            total_time: data.total_time,
            year: data.year,
        }
    }
}

#[derive(serde::Serialize)]
pub struct Entity {
    pub name: String,
    pub unique_id: String,
    pub object_id: String,
    pub qos: u8,
    pub unit_of_measurement: Option<String>,
    pub state_topic: String,
    pub state_class: Option<String>,
    pub device_class: Option<String>,
    pub device: Device,
    pub json_attributes_topic: String,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum EntityType {
    PowerSensor {
        name: String,
        value: u32,
    },
    TemperatureSensor {
        name: String,
        value: f32,
    },
    EnergySensor {
        name: String,
        value: f64,
    },
    #[allow(dead_code)]
    GenericSensor {
        name: String,
        value: f32,
    },
    GenericDiscreteSensor {
        name: String,
        value: String,
    },
}

impl Entity {
    pub fn power_sensor(name: String, prefix: String, device: &Device) -> Self {
        Entity {
            device: device.to_owned(),
            name: name.to_string(),
            unique_id: format!("{name}_{prefix}"),
            object_id: format!("{name}_{prefix}"),
            qos: 0,
            unit_of_measurement: Some("W".to_string()),
            state_topic: format!("{prefix}/state/{name}"),
            state_class: Some("measurement".to_string()),
            device_class: Some("power".to_string()),
            json_attributes_topic: format!("{prefix}/attributes"),
        }
    }

    pub fn temperature_entity(name: String, prefix: String, device: &Device) -> Self {
        Entity {
            device: device.to_owned(),
            name: name.to_string(),
            unique_id: format!("{name}_{prefix}"),
            object_id: format!("{name}_{prefix}"),
            qos: 0,
            unit_of_measurement: Some("°C".to_string()),
            state_topic: format!("{prefix}/state/{name}"),
            state_class: Some("measurement".to_string()),
            device_class: Some("temperature".to_string()),
            json_attributes_topic: format!("{prefix}/attributes"),
        }
    }

    pub fn energy_sensor(name: String, prefix: String, device: &Device) -> Self {
        Entity {
            device: device.to_owned(),
            name: name.to_string(),
            unique_id: format!("{name}_{prefix}"),
            object_id: format!("{name}_{prefix}"),
            qos: 0,
            unit_of_measurement: Some("kWh".to_string()),
            state_topic: format!("{prefix}/state/{name}"),
            state_class: Some("total_increasing".to_string()),
            device_class: Some("energy".to_string()),
            json_attributes_topic: format!("{prefix}/attributes"),
        }
    }

    pub fn generic_sensor(name: String, prefix: String, device: &Device, discrete: bool) -> Self {
        Entity {
            device: device.to_owned(),
            name: name.to_string(),
            unique_id: format!("{name}_{prefix}"),
            object_id: format!("{name}_{prefix}"),
            qos: 0,
            state_topic: format!("{prefix}/state/{name}"),
            device_class: None,
            state_class: if discrete {
                None
            } else {
                Some("measurement".to_string())
            },
            unit_of_measurement: None,
            json_attributes_topic: format!("{prefix}/attributes"),
        }
    }
}

pub fn entities_from_data(data: &Data) -> Vec<EntityType> {
    vec![
        EntityType::PowerSensor {
            name: "current_power".to_string(),
            value: data.current_power,
        },
        EntityType::EnergySensor {
            name: "daily_energy".to_string(),
            value: data.daily_energy,
        },
        EntityType::TemperatureSensor {
            name: "inverter_temperature".to_string(),
            value: data.inverter_temperature,
        },
        EntityType::GenericDiscreteSensor {
            name: "inverter_status".to_string(),
            value: data.inverter_status.to_string(),
        },
        EntityType::EnergySensor {
            name: "total_energy".to_string(),
            value: data.total_energy,
        },
    ]
}
