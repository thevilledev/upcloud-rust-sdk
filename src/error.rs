use std::fmt;

#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    ApiError {
        status: u16,
        message: String,
    },
    SerdeError(serde_json::Error),
    Timeout,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::RequestError(e) => write!(f, "Request error: {}", e),
            Error::ApiError { status, message } => {
                write!(f, "API error ({}): {}", status, message)
            }
            Error::SerdeError(e) => write!(f, "Serialization error: {}", e),
            Error::Timeout => write!(f, "Request timed out"),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::RequestError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeError(err)
    }
}