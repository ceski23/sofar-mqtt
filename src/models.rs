use crate::helpers::{divide_i16_by, divide_u16_by, divide_u32_by, parse_string};

#[derive(Primitive, Debug)]
pub enum MessageType {
    Heartbeat = 0x4710,
    Data = 0x4210,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct ServerResponse {
    sth: u8,
    one: u8,
    timestamp: u32,
    sth2: u16,
    zero: u16,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct Heartbeat {
    zero: u8,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct Data {
    #[serde(skip_serializing)]
    _sth0: u8,
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

#[derive(Debug)]
pub enum SofarMessage {
    Heartbeat(Heartbeat),
    Data(Data),
}
