use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Container {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Config")]
    pub config: Config,
    #[serde(rename = "NetworkSettings")]
    pub network_settings: NetworkSettings,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "Labels")]
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct NetworkSettings {
    #[serde(rename = "Ports")]
    pub ports: HashMap<String, Vec<Port>>,
}

#[derive(Debug, Deserialize)]
pub struct Port {
    #[serde(rename = "HostIp")]
    pub host_ip: String,
    #[serde(rename = "HostPort")]
    pub host_port: String,
}
