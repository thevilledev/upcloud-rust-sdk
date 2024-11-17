use upcloud_rust_sdk::client::Client;

// Remember to define UPCLOUD_USERNAME and UPCLOUD_PASSWORD environment variables

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()?;

    // Get servers
    let servers = client.get_servers().await?;

    #[cfg(debug_assertions)]
    println!("{:?}", servers);

    Ok(())
}