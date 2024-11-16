use crate::{
    client::Client,
    error::Error,
    types::server::*,
};

use tokio::time::sleep;
use std::time::Duration;
impl Client {

    pub async fn get_servers(&self) -> Result<ServerResponse, Error> {
        let response = self.get("/server").await?;
        Ok(serde_json::from_str(&response)?)
    }

    pub async fn get_server_details(&self, uuid: &str) -> Result<ServerDetails, Error> {
        let response = self.get(&format!("/server/{}", uuid)).await?;
        Ok(serde_json::from_str(&response)?)
    }

    // Add other server-related methods here

    pub async fn create_server(&self, request: &CreateServerRequestWrapper) -> Result<String, Error> {
        let response = self.post("/server", Some(request)).await?;
        let details: ServerDetails = serde_json::from_str(&response)?;
        Ok(details.uuid)
    }

    pub async fn start_server(&self, uuid: &str, request: &StartServerRequest) -> Result<ServerDetails, Error> {
        let response = self.post(&format!("/server/{}/start", uuid), Some(request)).await?;
        Ok(serde_json::from_str(&response)?)
    }

    pub async fn stop_server(&self, uuid: &str, request: &StopServerRequest) -> Result<ServerDetails, Error> {
        let response = self.post(&format!("/server/{}/stop", uuid), Some(request)).await?;
        Ok(serde_json::from_str(&response)?)
    }

    pub async fn restart_server(&self, uuid: &str, request: &RestartServerRequest) -> Result<ServerDetails, Error> {
        let response = self.post(&format!("/server/{}/restart", uuid), Some(request)).await?;
        Ok(serde_json::from_str(&response)?)
    }

    pub async fn modify_server(&self, uuid: &str, request: &ModifyServerRequest) -> Result<ServerDetails, Error> {
        let response = self.put(&format!("/server/{}", uuid), Some(request)).await?;
        Ok(serde_json::from_str(&response)?)
    }

    pub async fn delete_server(&self, uuid: &str) -> Result<(), Error> {
        self.delete(&format!("/server/{}", uuid)).await?;
        Ok(())
    }

    pub async fn delete_server_and_storages(&self, uuid: &str, delete_backups: bool) -> Result<(), Error> {
        let path = if delete_backups {
            format!("/server/{}/?storages=1&backups=delete", uuid)
        } else {
            format!("/server/{}/?storages=1", uuid)
        };
        self.delete(&path).await?;
        Ok(())
    }

    pub async fn wait_for_server_state(
        &self,
        uuid: &str,
        desired_state: Option<&str>,
        undesired_state: Option<&str>,
        timeout: Duration,
    ) -> Result<ServerDetails, Error> {
        let start = std::time::Instant::now();
        
        loop {
            if start.elapsed() > timeout {
                return Err(Error::Timeout);
            }

            let details = self.get_server_details(uuid).await?;

            match (desired_state, undesired_state) {
                (Some(desired), _) if details.state.as_deref() == Some(desired) => return Ok(details),
                (_, Some(undesired)) if details.state.as_deref() != Some(undesired) => return Ok(details),
                _ => {
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }
            }
        }
    }
}
