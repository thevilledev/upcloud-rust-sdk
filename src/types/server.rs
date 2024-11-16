use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfigurations {
    #[serde(rename = "server_sizes")]
    pub server_configurations: Vec<ServerConfiguration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfiguration {
    #[serde(rename = "core_number")]
    pub core_number: String,
    #[serde(rename = "memory_amount")]
    pub memory_amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub core_number: String,
    pub hostname: String,
    pub license: f64,
    pub memory_amount: String,
    pub plan: String,
    pub progress: String,
    pub state: String,
    pub tags: Vec<String>,
    pub title: String,
    pub uuid: String,
    pub zone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Servers {
    pub servers: Vec<Server>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerDetails {
    #[serde(flatten)]
    pub server: Server,
    pub boot_order: String,
    pub firewall: String,
    pub host: i32,
    // Add other fields as needed
}