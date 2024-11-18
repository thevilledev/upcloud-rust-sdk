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

#[tokio::test]
async fn test_list_servers() {
    use crate::config;

    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let _mock = server.mock("GET", "/1.3/server")
        .with_status(200)
        .with_body(r#"{"servers":{"server":[{"core_number":"1","hostname":"fi.example.com","labels":{"label":[{"key":"env","value":"prod"}]},"license":0,"memory_amount":"2048","plan":"1xCPU-2GB","plan_ivp4_bytes":"34253332","plan_ipv6_bytes":"0","state":"started","tags":{"tag":["PROD","CentOS"]},"title":"Helsinki server","uuid":"00798b85-efdc-41ca-8021-f6ef457b8531","zone":"fi-hel1"},{"core_number":"1","hostname":"uk.example.com","labels":{"label":[]},"license":0,"memory_amount":"512","plan":"custom","state":"stopped","tags":{"tag":["DEV","Ubuntu"]},"title":"London server","uuid":"009d64ef-31d1-4684-a26b-c86c955cbf46","zone":"uk-lon1"}]}}"#)
        .create();

    let client = Client::with_config(
        config::Config::new("foo", "bar")
            .with_base_url(url)
    ).unwrap();

    
    let result = client.list_servers().await.unwrap();
    assert_eq!(result.server.len(), 2);
    assert_eq!(result.server[0].uuid, "00798b85-efdc-41ca-8021-f6ef457b8531");
    assert_eq!(result.server[0].title, "Helsinki server");
}

// TODO: Add test for get_server
#[tokio::test]
async fn test_get_server() {}

// TODO: Add test for create_server
#[tokio::test]
async fn test_create_server() {}