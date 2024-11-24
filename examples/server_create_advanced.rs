use upcloud_sdk::{
    client::Client,
    config::Config,
    error::Error,
    resources::server::ServerOperations,
    types::{common::Labels, server::*}
};

use std::sync::Arc;
use reqwest::ClientBuilder;

use std::time::Duration;

// Remember to define UPCLOUD_USERNAME and UPCLOUD_PASSWORD environment variables

pub const TEMPLATE_UUID: &str = "01000000-0000-4000-8000-000020070100"; // Debian 12 (Bookworm)
pub const SSH_USER: &str = "admin";
pub const SSH_KEY: &str = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAINVn5Alm7dObCxo7Z03jyOIZWbcTms7VX3KxastNZHm8 foo@example.tld";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(
        std::env::var("UPCLOUD_USERNAME")
            .map_err(|_| Error::ConfigError("UPCLOUD_USERNAME environment variable not set".to_string()))?,
        std::env::var("UPCLOUD_PASSWORD")
            .map_err(|_| Error::ConfigError("UPCLOUD_PASSWORD environment variable not set".to_string()))?
    )
        .with_timeout(Duration::from_secs(10))
        .with_http_client_hook(Arc::new(|builder: ClientBuilder| {
            builder
                .tcp_keepalive(Duration::from_secs(60))
                .pool_max_idle_per_host(10)
                .https_only(true)
        }));

    let client = Client::with_config(config)?;

    // Create a new server
    let create_request = CreateServerRequest::new()
        .with_title("test-server")
        .with_hostname("test-server.example.com")
        .with_zone("fi-hel1")
        .with_plan("1xCPU-2GB")
        .with_login_user(
            LoginUser::new(SSH_USER)
                .with_create_password(false)
                .with_ssh_key(SSH_KEY)
        )
        .with_storage_device(
            CreateServerStorageDevice::from_template(TEMPLATE_UUID)
                .with_size(20)
                .with_tier("maxiops")
                .with_title("System Disk")
                .with_encrypted(true)
        )
        .with_networking(
            CreateServerNetworking::new()
                .with_interface(
                    CreateServerInterface::new("public")
                        .with_ip_address("IPv4", None)
                        .with_index(1)
                )
                .with_interface(
                    CreateServerInterface::new("utility")
                        .with_ip_address("IPv4", None)
                        .with_index(2)
                )
        )
        .with_labels(
            Labels::new()
                .with("env", "prod")
        )
        .with_metadata("yes")
        .build();

    #[cfg(debug_assertions)]
    println!("Create request: {:?}", create_request);

    let res = client.create_server(&create_request).await?;

    #[cfg(debug_assertions)]
    println!("Created server: {:?}", res.server);

    // Wait for server to start
    let _ = client.wait_for_server_state(
        &res.server.uuid,
        Some(&ServerState::Started),
        None,
        Duration::from_secs(300)
    ).await?;

    #[cfg(debug_assertions)]
    println!("Server started: {:?}", res.server);

    Ok(())
}