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

    pub fn new_temperature_entity(name: String, prefix: String, device: &Device) -> Self {
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

    pub fn generic_sensor(name: String, prefix: String, device: &Device) -> Self {
        Entity {
            device: device.to_owned(),
            name: name.to_string(),
            unique_id: format!("{name}_{prefix}"),
            object_id: format!("{name}_{prefix}"),
            qos: 0,
            state_topic: format!("{prefix}/state/{name}"),
            device_class: None,
            state_class: Some("measurement".to_string()),
            unit_of_measurement: None,
            json_attributes_topic: format!("{prefix}/attributes"),
        }
    }
}
