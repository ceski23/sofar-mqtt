use serde::Deserialize;

fn default_tcp_port() -> u16 {
    8080
}

fn default_mqtt_host() -> String {
    String::from("localhost")
}

fn default_mqtt_port() -> u16 {
    1883
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_mqtt_host")]
    pub mqtt_host: String,
    #[serde(default = "default_mqtt_port")]
    pub mqtt_port: u16,
    pub mqtt_user: Option<String>,
    pub mqtt_password: Option<String>,
    #[serde(default = "default_tcp_port")]
    pub tcp_port: u16,
}
