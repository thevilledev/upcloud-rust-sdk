use reqwest::Client as ReqwestClient;

use crate::constants::{API_BASE_URL, API_VERSION, VERSION};
use crate::config::Config;
use crate::error::Error;

pub struct Client {
    config: Config,
    client: ReqwestClient,
}

impl Client {
    pub fn new() -> Result<Self, Error> {
        let config = Config::new(
            std::env::var("UPCLOUD_USERNAME")
                .map_err(|_| Error::ConfigError("UPCLOUD_USERNAME environment variable not set".to_string()))?,
            std::env::var("UPCLOUD_PASSWORD")
                .map_err(|_| Error::ConfigError("UPCLOUD_PASSWORD environment variable not set".to_string()))?
        );
        Self::with_config(config)
    }

    pub fn with_config(config: Config) -> Result<Self, Error> {
        let mut client_builder = ReqwestClient::builder()
            .user_agent(format!("upcloud-rust-sdk/{}", VERSION));

        if let Some(timeout) = config.timeout {
            client_builder = client_builder.timeout(timeout);
        }

        if let Some(hook) = config.http_client_hook.as_ref() {
            client_builder = hook(client_builder);
        }

        let client = client_builder.build()?;

        Ok(Self { config, client })
    }

    pub(crate) async fn get(&self, path: &str) -> Result<String, Error> {
        self.request(reqwest::Method::GET, path, Option::<&()>::None).await
    }

    pub(crate) async fn post<T: serde::Serialize + std::fmt::Debug>(
        &self,
        path: &str,
        body: Option<&T>,
    ) -> Result<String, Error> {
        println!("POST: {:?}", body);
        self.request(reqwest::Method::POST, path, body).await
    }

    pub(crate) async fn put<T: serde::Serialize + std::fmt::Debug>(
        &self,
        path: &str,
        body: Option<&T>,
    ) -> Result<String, Error> {
        self.request(reqwest::Method::PUT, path, body).await
    }

    pub(crate) async fn delete(&self, path: &str) -> Result<String, Error> {
        self.request(reqwest::Method::DELETE, path, Option::<&()>::None).await
    }

    async fn request<T: serde::Serialize + std::fmt::Debug>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<&T>,
    ) -> Result<String, Error> {
        let url = format!(
            "{}/{}/{}",
            self.config.base_url.as_deref().unwrap_or(API_BASE_URL),
            API_VERSION,
            path.trim_start_matches('/')
        );

        let mut builder = self.client.request(method, &url);
        
        builder = builder.basic_auth(&self.config.username, Some(&self.config.password));

        if let Some(body) = body {
            builder = builder.json(body);
        }

        let response = builder.send().await?;
        
        if !response.status().is_success() {
            return Err(Error::ApiError {
                status: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        Ok(response.text().await?)
    }
}