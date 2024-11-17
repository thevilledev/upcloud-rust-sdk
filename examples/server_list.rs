use upcloud_rust_sdk::{config::Config, client::Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(
        &std::env::var("UPCLOUD_USERNAME").expect("UPCLOUD_USERNAME must be set"),
        &std::env::var("UPCLOUD_PASSWORD").expect("UPCLOUD_PASSWORD must be set")
    );
    let client = Client::new(config)?;

    // Get servers
    let servers = client.get_servers().await?;

    #[cfg(debug_assertions)]
    println!("{:?}", servers);

    Ok(())
}