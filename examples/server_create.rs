use upcloud_rust_sdk::{config::Config, client::Client};
use upcloud_rust_sdk::types::server::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(
        &std::env::var("UPCLOUD_USERNAME").expect("UPCLOUD_USERNAME must be set"),
        &std::env::var("UPCLOUD_PASSWORD").expect("UPCLOUD_PASSWORD must be set")
    );
    let client = Client::new(config)?;

    // Create a new server
    let create_request = CreateServerRequest::new()
        .with_title("test-server".to_string())
        .with_hostname("test-server.example.com".to_string())
        .with_zone("fi-hel1".to_string())
        .with_plan("1xCPU-2GB".to_string())
        .with_storage_device(CreateServerStorageDevice::new(
            CREATE_SERVER_STORAGE_DEVICE_ACTION_CLONE.to_string(),
            "01000000-0000-4000-8000-000020070100".to_string())
                .with_size(20)
                .with_tier("maxiops")
                .with_title("System Disk")
                .with_encrypted(true)
        )
        .with_networking(CreateServerNetworking {
            interfaces: InterfaceWrapper {
                interface: vec![CreateServerInterface {
                    ip_addresses: IPAddressWrapper {
                        ip_address: vec![CreateServerIPAddress {
                            family: Some("IPv4".to_string()),
                            address: None,
                        }]
                    },
                    interface_type: "public".to_string(),
                    network: None,
                    source_ip_filtering: Some("yes".to_string()),
                    bootable: Some("no".to_string()),
                    index: Some(1),
                }]
            }
        })
        .with_login_user(LoginUser {
            username: "admin".to_string(),
            ssh_keys: None,
            create_password: Some("yes".to_string()),
        })
        .with_password_delivery(PASSWORD_DELIVERY_SMS.to_string())
        .with_metadata("yes".to_string())
        .build();

    let res = client.create_server(&create_request).await?;

    #[cfg(debug_assertions)]
    println!("Created server: {:?}", res.server);

    // Wait for server to start
    let _ = client.wait_for_server_state(
        &res.server.uuid,
        Some(SERVER_STATE_STARTED),
        None,
        Duration::from_secs(300)
    ).await?;

    #[cfg(debug_assertions)]
    println!("Server started: {:?}", res.server);

    Ok(())
}