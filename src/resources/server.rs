use async_trait::async_trait;

use crate::{
    error::Error,
    types::server::*,
    client::Client,
};

use tokio::time::{sleep, Duration};

#[async_trait]
pub trait ServerOperations {
    async fn list_servers(&self) -> Result<ServerList, Error>;
    async fn get_server(&self, uuid: &str) -> Result<ServerDetails, Error>;
    async fn create_server(&self, request: &CreateServerRequest) -> Result<ServerDetails, Error>;
    async fn start_server(&self, uuid: &str, request: &StartServerRequest) -> Result<StartServerResponse, Error>;
    async fn stop_server(&self, uuid: &str, request: &StopServerRequest) -> Result<StopServerResponse, Error>;
    async fn restart_server(&self, uuid: &str, request: &RestartServerRequest) -> Result<RestartServerResponse, Error>;
    async fn modify_server(&self, uuid: &str, request: &ModifyServerRequest) -> Result<ModifyServerResponse, Error>;
    async fn delete_server(&self, uuid: &str) -> Result<(), Error>;
    async fn delete_server_and_storages(&self, uuid: &str, delete_backups: bool) -> Result<(), Error>;
    async fn wait_for_server_state(
        &self,
        uuid: &str,
        desired_state: Option<&str>,
        undesired_state: Option<&str>,
        timeout: Duration,
    ) -> Result<ServerDetails, Error>;
}

#[async_trait]
impl ServerOperations for Client {
    async fn list_servers(&self) -> Result<ServerList, Error> {
        let response = self.get("/server").await?;
        let details: GetServerResponse = serde_json::from_str(&response)?;
        Ok(details.servers)
    }

    async fn get_server(&self, uuid: &str) -> Result<ServerDetails, Error> {
        let response = self.get(&format!("/server/{}", uuid)).await?;
        let details: GetServerDetailsResponse = serde_json::from_str(&response)?;
        Ok(details.server)
    }

    async fn create_server(&self, request: &CreateServerRequest) -> Result<ServerDetails, Error> {
        let response = self.post("/server", Some(request)).await?;
        let create_response: CreateServerResponse = serde_json::from_str(&response)?;
        Ok(create_response.server)
    }

    async fn start_server(&self, uuid: &str, request: &StartServerRequest) -> Result<StartServerResponse, Error> {
        let response = self.post(&format!("/server/{}/start", uuid), Some(request)).await?;
        Ok(serde_json::from_str(&response)?)
    }

    async fn stop_server(&self, uuid: &str, request: &StopServerRequest) -> Result<StopServerResponse, Error> {
        let response = self.post(&format!("/server/{}/stop", uuid), Some(request)).await?;
        Ok(serde_json::from_str(&response)?)
    }

    async fn restart_server(&self, uuid: &str, request: &RestartServerRequest) -> Result<RestartServerResponse, Error> {
        let response = self.post(&format!("/server/{}/restart", uuid), Some(request)).await?;
        Ok(serde_json::from_str(&response)?)
    }

    async fn modify_server(&self, uuid: &str, request: &ModifyServerRequest) -> Result<ModifyServerResponse, Error> {
        let response = self.put(&format!("/server/{}", uuid), Some(request)).await?;
        Ok(serde_json::from_str(&response)?)
    }

    async fn delete_server(&self, uuid: &str) -> Result<(), Error> {
        self.delete(&format!("/server/{}", uuid)).await?;
        Ok(())
    }

    async fn delete_server_and_storages(&self, uuid: &str, delete_backups: bool) -> Result<(), Error> {
        let path = if delete_backups {
            format!("/server/{}/?storages=1&backups=delete", uuid)
        } else {
            format!("/server/{}/?storages=1", uuid)
        };
        self.delete(&path).await?;
        Ok(())
    }

    async fn wait_for_server_state(
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

            let res = self.get_server(uuid).await?;

            match (desired_state, undesired_state) {
                (Some(desired), _) if res.server.state == desired => return Ok(res),
                (_, Some(undesired)) if res.server.state != undesired => return Ok(res),
                _ => {
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }
            }
        }
    }
}