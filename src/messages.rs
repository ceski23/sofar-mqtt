use crate::serde_helpers::{divide_i16_by, divide_u16_by, divide_u32_by, parse_string};
use macaddr::MacAddr6;

#[derive(Primitive, Debug, Clone, Copy)]
pub enum SofarMessageType {
    Heartbeat = 0x4710,
    Data = 0x4210,
    Hello = 0x4110,
    HelloCd = 0x4810,
    Unknown44 = 0x4310,
}

#[allow(dead_code)]
#[derive(serde::Serialize, Debug)]
pub struct ServerResponse {
    pub message_id: u8,
    _unknown1: u8,
    pub timestamp: u32,
    _unknown2: u16,
    _unknown3: u16,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct Heartbeat {
    pub _unknown: u8,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct Data {
    #[serde(skip_serializing)]
    pub _unknown1: u8,
    sensor_type_list: u16,
    total_operation_time: u32,
    timer: u32,
    pub timestamp: u32,
    #[serde(skip_serializing)]
    _unknown2: u16,
    counter: u32,
    #[serde(deserialize_with = "parse_string::<_, 16>")]
    pub inverter_serial_number: String,
    #[serde(deserialize_with = "divide_i16_by::<_, 10>")]
    pub inverter_temperature: f32,
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
    pub current_power: u32,
    #[serde(deserialize_with = "divide_u32_by::<_, 100>")]
    pub daily_energy: f64,
    #[serde(deserialize_with = "divide_u32_by::<_, 10>")]
    pub total_energy: f64,
    pub total_time: u32,
    pub inverter_status: u16,
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
    pub inverter_firmware: String,
    #[serde(deserialize_with = "parse_string::<_, 4>")]
    pub hardware_version: String,
    pub logger_temperature: i16,
    #[serde(deserialize_with = "divide_u16_by::<_, 10>")]
    bus_voltage: f32,
    #[serde(deserialize_with = "divide_u16_by::<_, 10>")]
    vice_cpu_input_voltage: f32,
    #[serde(skip_serializing)]
    _unknown3: u16,
    countdown_time: u16,
    #[serde(skip_serializing)]
    _unknown4: u16,
    pv1_insulation_resistance: u16,
    pv2_insulation_resistance: u16,
    insulation_impedance: u16,
    pub country_code: u16,
    #[serde(skip_serializing)]
    _unknown5: u32,
    leaking_current: u16,
    a_phase_dc_distribution: u16,
    b_phase_dc_distribution: u16,
    c_phase_dc_distribution: u16,
    #[serde(deserialize_with = "parse_string::<_, 4>")]
    pub main_inverter_firmware: String,
    #[serde(deserialize_with = "parse_string::<_, 4>")]
    pub slave_inverter_firmware: String,
    pub year: u8,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    #[serde(skip_serializing)]
    _unknown6: u32,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct Hello {
    #[serde(skip_serializing)]
    pub one: u8,
    total_operation_time: u32,
    timer: u32,
    #[serde(skip_serializing)]
    _unknown1: u32,
    uploading_frequency: u8,
    data_logging_frequency: u8,
    hearbeat_frequency: u8,
    max_num_of_connected_devices: u8,
    signal_quality: u8,
    sensor_type: u8,
    #[serde(deserialize_with = "parse_string::<_, 40>")]
    pub module_version: String,
    sta_mac_address: MacAddr6,
    #[serde(deserialize_with = "parse_string::<_, 16>")]
    pub local_ip_address: String,
    #[serde(skip_serializing)]
    _unknown2: u16,
    #[serde(skip_serializing)]
    _unknown3: u16,
    sensor_type_list: u16,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct HelloCd {
    pub one: u8,
    total_operation_time: u32,
    timer: u32,
    timestamp: u32,
    _unknown1: u16,
    _unknown2: u32,
    _unknown3: u8,
    _unknown4: u32,
    _unknown5: u32,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct HelloEnd {
    pub one: u8,
    total_operation_time: u32,
    timer: u32,
    timestamp: u32,
    _unknown1: u16,
    _unknown2: u16,
    _unknown3: u16,
    _unknown4: u16,
    _unknown5: u16,
    _unknown6: u16,
    _unknown7: u16,
    _unknown8: u16,
    _unknown9: u16,
    _unknown10: u16,
    _unknown11: u16,
    _unknown12: u16,
    _unknown13: u16,
    _unknown14: u16,
    _unknown15: u16,
    _unknown16: u16,
    _unknown17: u16,
    _unknown18: u16,
    _unknown19: u16,
    _unknown20: u16,
    _unknown21: u16,
    _unknown22: u16,
    _unknown23: u16,
    _unknown24: u8,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct Unknown44 {
    pub _unknown1: u8,
    _unknown2: u8,
    _unknown3: u8,
    _unknown4: u8,
    _unknown5: u8,
    _unknown6: u8,
    _unknown7: u8,
    _unknown8: u8,
    _unknown9: u8,
    timestamp: u32,
    _unknown10: u16,
    #[serde(deserialize_with = "parse_string::<_, 16>")]
    wifi_ssid: String,
}

#[derive(Debug)]
pub enum IncomingMessageData {
    Heartbeat(Heartbeat),
    Data(Data),
    Hello(Hello),
    HelloCd(HelloCd),
    #[allow(dead_code)]
    HelloEnd(HelloEnd),
    Unknown44(Unknown44),
}

#[derive(Debug)]
pub enum OutgoingMessageData {
    ServerResponse(ServerResponse),
}

#[derive(Debug)]
pub struct SofarMessage<T> {
    pub data: T,
    pub message_type: SofarMessageType,
    pub message_number: u8,
    pub message_number_2: u8,
    pub data_logger_sn: u32,
}

impl SofarMessage<OutgoingMessageData> {
    pub fn from_incoming_message(
        request: &SofarMessage<IncomingMessageData>,
        timestamp: u32,
    ) -> Self {
        let message_id = match &request.data {
            IncomingMessageData::Heartbeat(data) => data._unknown,
            IncomingMessageData::Data(data) => data._unknown1,
            IncomingMessageData::Hello(data) => data.one,
            IncomingMessageData::HelloCd(data) => data.one,
            IncomingMessageData::HelloEnd(data) => data.one,
            IncomingMessageData::Unknown44(data) => data._unknown1,
        };

        SofarMessage {
            data: OutgoingMessageData::ServerResponse(ServerResponse {
                message_id,
                _unknown1: 1,
                timestamp,
                _unknown2: 0x0078,
                _unknown3: 0,
            }),
            message_type: request.message_type,
            message_number: request.message_number + 1,
            message_number_2: request.message_number_2,
            data_logger_sn: request.data_logger_sn,
        }
    }
}
