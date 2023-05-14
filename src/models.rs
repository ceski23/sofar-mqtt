use std::time::{SystemTime, UNIX_EPOCH};

use crate::helpers::{divide_i16_by, divide_u16_by, divide_u32_by, parse_string};

#[derive(Primitive, Debug)]
pub enum MessageType {
    Heartbeat = 0x4710,
    Data = 0x4210,
}

#[allow(dead_code)]
#[derive(serde::Serialize, Debug)]
pub struct ServerResponse {
    // always matches request
    message_id: u8,
    one: u8,
    pub timestamp: u32,
    sth2: u16,
    zero: u16,
}

impl ServerResponse {
    pub(crate) fn new(message_id: u8) -> Self {
        let timestamp = u32::try_from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        )
        .unwrap_or_default();

        ServerResponse {
            message_id,
            one: 1,
            timestamp,
            sth2: 0x7800,
            zero: 0,
        }
    }
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct Heartbeat {
    pub zero: u8,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct Data {
    #[serde(skip_serializing)]
    pub _sth0: u8,
    sensor_type_list: u16,
    total_operation_time: u32,
    timer: u32,
    timestamp: u32,
    #[serde(skip_serializing)]
    _sth: u16,
    counter: u32,
    #[serde(deserialize_with = "parse_string::<_, 16>")]
    inventer_serial_number: String,
    #[serde(deserialize_with = "divide_i16_by::<_, 10>")]
    inventer_temperature: f32,
    #[serde(deserialize_with = "divide_u16_by::<_, 10>")]
    vdc_1: f32,
    #[serde(deserialize_with = "divide_u16_by::<_, 10>")]
    vdc_2: f32,
    #[serde(deserialize_with = "divide_u16_by::<_, 10>")]
    idc_1: f32,
    #[serde(deserialize_with = "divide_u16_by::<_, 10>")]
    idc_2: f32,
    #[serde(deserialize_with = "divide_u16_by::<_, 10>")]
    iac_1: f32,
    #[serde(deserialize_with = "divide_u16_by::<_, 10>")]
    iac_2: f32,
    #[serde(deserialize_with = "divide_u16_by::<_, 10>")]
    iac_3: f32,
    #[serde(deserialize_with = "divide_u16_by::<_, 10>")]
    vac_1: f32,
    #[serde(deserialize_with = "divide_u16_by::<_, 10>")]
    vac_2: f32,
    #[serde(deserialize_with = "divide_u16_by::<_, 10>")]
    vac_3: f32,
    #[serde(deserialize_with = "divide_u16_by::<_, 100>")]
    fac: f32,
    current_power: u32,
    #[serde(deserialize_with = "divide_u32_by::<_, 100>")]
    daily_energy: f64,
    #[serde(deserialize_with = "divide_u32_by::<_, 10>")]
    total_energy: f64,
    total_time: u32,
    inverter_status: u16,
    fault_code_1: u8,
    fault_code_2: u8,
    fault_code_3: u8,
    fault_code_4: u8,
    fault_code_5: u8,
    fault_code_6: u8,
    fault_code_7: u8,
    fault_code_8: u8,
    fault_code_9: u8,
    fault_code_10: u8,
    alert_message_code: u16,
    inner_board_message_code: u16,
    #[serde(deserialize_with = "parse_string::<_, 4>")]
    inverter_firmware: String,
    #[serde(deserialize_with = "parse_string::<_, 4>")]
    hardware_version: String,
    logger_temperature: i16,
    #[serde(deserialize_with = "divide_u16_by::<_, 10>")]
    bus_voltage: f32,
    #[serde(deserialize_with = "divide_u16_by::<_, 10>")]
    vice_cpu_input_voltage: f32,
    #[serde(skip_serializing)]
    _sth2: u16,
    countdown_time: u16,
    #[serde(skip_serializing)]
    _sth3: u16,
    pv1_insulation_resistance: u16,
    pv2_insulation_resistance: u16,
    insulation_impedance: u16,
    country_code: u16,
    #[serde(skip_serializing)]
    _sth4: u32,
    leaking_current: u16,
    a_phase_dc_distribution: u16,
    b_phase_dc_distribution: u16,
    c_phase_dc_distribution: u16,
    #[serde(deserialize_with = "parse_string::<_, 4>")]
    main_inverter_firmware: String,
    #[serde(deserialize_with = "parse_string::<_, 4>")]
    slave_inverter_firmware: String,
    year: u8,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    #[serde(skip_serializing)]
    _sth5: u32,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct Hello {
    #[serde(skip_serializing)]
    one: u8,
    total_operation_time: u32,
    timer: u32,
    #[serde(skip_serializing)]
    zero: u16,
    uploading_frequency: u8,
    data_logging_frequency: u8,
    hearbeat_frequency: u8,
    max_num_of_connected_devices: u8,
    signal_quality: u8,
    sensor_type: u8,
    #[serde(deserialize_with = "parse_string::<_, 39>")]
    module_version: String,
    #[serde(deserialize_with = "parse_string::<_, 6>")]
    sta_mac_address: String,
    #[serde(deserialize_with = "parse_string::<_, 15>")]
    local_ip_address: String,
    #[serde(skip_serializing)]
    zero2: u16,
    #[serde(skip_serializing)]
    one2: u16,
    sensor_type_list: u16,
}

#[derive(Debug)]
pub enum MessageData {
    Heartbeat(Heartbeat),
    Data(Data),
    // Hello(Hello),
}

#[derive(Debug)]
pub struct SofarMessage {
    pub data: MessageData,
    pub message_type: MessageType,
    pub message_number: u8,
    pub data_logger_sn: u32,
}

#[derive(Debug)]
pub enum ResponseData {
    ServerResponse(ServerResponse),
}

#[derive(Debug)]
pub struct SofarResponseMessage {
    pub data: ResponseData,
    pub request_type: MessageType,
    pub request_message_number: u8,
    pub data_logger_sn: u32,
}
