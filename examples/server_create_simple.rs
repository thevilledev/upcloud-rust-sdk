use upcloud_sdk::{client::Client, resources::server::ServerOperations};
use upcloud_sdk::types::server::*;

// Remember to define UPCLOUD_USERNAME and UPCLOUD_PASSWORD environment variables

pub const TEMPLATE_UUID: &str = "01000000-0000-4000-8000-000020070100"; // Debian 12 (Bookworm)
pub const SSH_USER: &str = "admin";
pub const SSH_KEY: &str = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAINVn5Alm7dObCxo7Z03jyOIZWbcTms7VX3KxastNZHm8 foo@example.tld";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()?;

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
        )
        .with_metadata("yes") // Enable metadata for cloud-init templates
        .build();

    #[cfg(debug_assertions)]
    println!("Create request: {:?}", create_request);

    let res = client.create_server(&create_request).await?;

    #[cfg(debug_assertions)]
    println!("Created server: {:?}", res.server);

    Ok(())
}