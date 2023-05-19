use crate::helpers::{divide_i16_by, divide_u16_by, divide_u32_by, parse_string};
use macaddr::MacAddr6;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Primitive, Debug)]
pub enum MessageType {
    Heartbeat = 0x4710,
    Data = 0x4210,
    Hello = 0x4110,
    HelloCd = 0x4810,
    Unknown44 = 0x4310,
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
            sth2: 0x0078,
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
    pub one: u8,
    total_operation_time: u32,
    timer: u32,
    #[serde(skip_serializing)]
    zero: u32,
    uploading_frequency: u8,
    data_logging_frequency: u8,
    hearbeat_frequency: u8,
    max_num_of_connected_devices: u8,
    signal_quality: u8,
    sensor_type: u8,
    #[serde(deserialize_with = "parse_string::<_, 40>")]
    module_version: String,
    sta_mac_address: MacAddr6,
    #[serde(deserialize_with = "parse_string::<_, 16>")]
    local_ip_address: String,
    #[serde(skip_serializing)]
    zero2: u16,
    #[serde(skip_serializing)]
    one2: u16,
    sensor_type_list: u16,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct HelloCd {
    pub one: u8,
    total_operation_time: u32,
    timer: u32,
    timestamp: u32,
    one2: u16,
    sth: u32,
    zero: u8,
    sth2: u32,
    timestamp2: u32,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct HelloEnd {
    pub one: u8,
    total_operation_time: u32,
    timer: u32,
    timestamp: u32,
    sth: u16,
    sth2: u16,
    sth3: u16,
    sth4: u16,
    sth5: u16,
    sth6: u16,
    sth7: u16,
    sth8: u16,
    sth9: u16,
    sth10: u16,
    sth11: u16,
    sth12: u16,
    sth13: u16,
    sth14: u16,
    sth15: u16,
    sth16: u16,
    sth17: u16,
    sth18: u16,
    sth19: u16,
    sth20: u16,
    sth21: u16,
    sth22: u16,
    sth23: u16,
    sth24: u8,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct Unknown44 {
    pub _sth1: u8,
    _sth2: u8,
    _sth3: u8,
    _sth4: u8,
    _sth5: u8,
    _sth6: u8,
    _sth7: u8,
    _sth8: u8,
    _sth9: u8,
    timestamp: u32,
    _sth10: u16,
    #[serde(deserialize_with = "parse_string::<_, 16>")]
    wifi_ssid: String,
}

#[derive(Debug)]
pub enum MessageData {
    Heartbeat(Heartbeat),
    Data(Data),
    Hello(Hello),
    HelloCd(HelloCd),
    HelloEnd(HelloEnd),
    Unknown44(Unknown44),
}

#[derive(Debug)]
pub struct SofarMessage {
    pub data: MessageData,
    pub message_type: MessageType,
    pub message_number: u8,
    pub message_number_2: u8,
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
    pub request_message_number_2: u8,
    pub data_logger_sn: u32,
}
