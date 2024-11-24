//! # upcloud-sdk
//!
//! Async Rust SDK for the UpCloud API.
//!
//! ## Example
//! ```rust
//! use upcloud_sdk::{client::Client, resources::server::ServerOperations};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new()?;
//! let servers = client.list_servers().await?;
//! # Ok(())
//! # }
//! ```

pub mod client;
pub mod error;
pub mod config;
pub mod types;
pub mod resources;

mod constants;

