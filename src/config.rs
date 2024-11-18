use std::time::Duration;
use reqwest::ClientBuilder;
use std::sync::Arc;

#[derive(Clone)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub base_url: Option<String>,
    pub timeout: Option<Duration>,
    pub http_client_hook: Option<Arc<dyn Fn(ClientBuilder) -> ClientBuilder + Send + Sync>>,

}

impl Config {
    pub fn new<S: Into<String>>(username: S, password: S) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
            base_url: None,
            timeout: None,
            http_client_hook: None,
        }
    }

    pub fn with_base_url<S: Into<String>>(mut self, base_url: S) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn with_http_client_hook(mut self, hook: Arc<dyn Fn(ClientBuilder) -> ClientBuilder + Send + Sync>) -> Self {
        self.http_client_hook = Some(hook);
        self
    }
}
