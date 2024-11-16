use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub base_url: Option<String>,
    pub timeout: Option<Duration>,
}

impl Config {
    pub fn new<S: Into<String>>(username: S, password: S) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
            base_url: None,
            timeout: None,
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
}