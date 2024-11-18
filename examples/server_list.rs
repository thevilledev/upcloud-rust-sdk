use upcloud_rust_sdk::{client::Client, resources::server::ServerOperations};

// Remember to define UPCLOUD_USERNAME and UPCLOUD_PASSWORD environment variables

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()?;

    // List servers
    let servers = client.list_servers().await?;

    #[cfg(debug_assertions)]
    println!("{:?}", servers);

    // Get specific server
    let server = client.get_server(&servers.server[0].uuid).await?;

    #[cfg(debug_assertions)]
    println!("{:?}", server);

    Ok(())
}