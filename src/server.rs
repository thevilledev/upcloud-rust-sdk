use serde::{Deserialize, Serialize};

use crate::{
    client::Client,
    error::Error,
    types::server::{ServerConfigurations, ServerDetails, Servers},
};

impl Client {

    pub async fn get_servers(&self) -> Result<Servers, Error> {
        let response = self.get("/server").await?;
        Ok(serde_json::from_str(&response)?)
    }

    pub async fn get_server_details(&self, uuid: &str) -> Result<ServerDetails, Error> {
        let response = self.get(&format!("/server/{}", uuid)).await?;
        Ok(serde_json::from_str(&response)?)
    }

    // Add other server-related methods here
}