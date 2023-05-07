#[derive(serde::Serialize)]
pub struct Device {
    pub configuration_url: String,
    pub identifiers: String,
    pub manufacturer: String,
    pub model: String,
    pub name: String,
    pub sw_version: String,
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
}

// TODO: Delete this
impl Default for Device {
    fn default() -> Self {
        Device {
            configuration_url: "http://10.0.0.64/index_cn.html".to_owned(),
            identifiers: "sofar".to_owned(),
            manufacturer: "Sofar".to_owned(),
            model: "SF4ES003".to_owned(),
            name: "Sofar SF4ES003".to_owned(),
            sw_version: "LSW3_14_FFFF_1.0.34".to_owned(),
        }
    }
}

impl Entity {
    pub fn power_sensor(name: String, prefix: String, device: Device) -> Self {
        Entity {
            device,
            name: name.to_string(),
            unique_id: format!("{name}_{prefix}"),
            object_id: format!("{name}_{prefix}"),
            qos: 0,
            unit_of_measurement: Some("W".to_string()),
            state_topic: format!("{prefix}/{name}"),
            state_class: Some("measurement".to_string()),
            device_class: Some("power".to_string()),
        }
    }

    pub fn new_temperature_entity(name: String, prefix: String, device: Device) -> Self {
        Entity {
            device,
            name: name.to_string(),
            unique_id: format!("{name}_{prefix}"),
            object_id: format!("{name}_{prefix}"),
            qos: 0,
            unit_of_measurement: Some("°C".to_string()),
            state_topic: format!("{prefix}/{name}"),
            state_class: Some("measurement".to_string()),
            device_class: Some("temperature".to_string()),
        }
    }

    pub fn energy_sensor(name: String, prefix: String, device: Device) -> Self {
        Entity {
            device,
            name: name.to_string(),
            unique_id: format!("{name}_{prefix}"),
            object_id: format!("{name}_{prefix}"),
            qos: 0,
            unit_of_measurement: Some("kWh".to_string()),
            state_topic: format!("{prefix}/{name}"),
            state_class: Some("total_increasing".to_string()),
            device_class: Some("energy".to_string()),
        }
    }

    pub fn generic_sensor(name: String, prefix: String, device: Device) -> Self {
        Entity {
            device,
            name: name.to_string(),
            unique_id: format!("{name}_{prefix}"),
            object_id: format!("{name}_{prefix}"),
            qos: 0,
            state_topic: format!("{prefix}/{name}"),
            device_class: None,
            state_class: None,
            unit_of_measurement: None,
        }
    }
}
