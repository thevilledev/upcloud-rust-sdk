use upcloud_rust_sdk::{
    client::Client,
    resources::server::ServerOperations,
    types::common::LabelFilter
};

// Remember to define UPCLOUD_USERNAME and UPCLOUD_PASSWORD environment variables

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()?;

    // List servers
    let servers = client.list_servers().await?;

    #[cfg(debug_assertions)]
    println!("{:?}", servers);

    // List servers by labels
    let label_filter = LabelFilter::new().with("env", "prod");
    let servers_by_labels = client.list_servers_by_labels(&label_filter).await?;

    #[cfg(debug_assertions)]
    println!("{:?}", servers_by_labels);

    // Get specific server
    let server = client.get_server(&servers.server[0].uuid).await?;

    #[cfg(debug_assertions)]
    println!("{:?}", server);

    Ok(())
}