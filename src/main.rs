#![allow(dead_code)]
mod db;
mod errors;
mod handlers;
mod server;
mod services;

use google_cloud_spanner::client::{ChannelConfig, Client, ClientConfig, google_cloud_auth};

const DATABASE: &str =
    "projects/evo-core-dev-001/instances/core-spanner-instance/databases/tag-database";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let path = std::env::var("GOOGLE_APPLICATION_CREDENTIALS").unwrap();
    let config = ClientConfig::default().with_auth().await.unwrap();
    println!("{:?}", &config);
    match Client::new(DATABASE, config).await {
        Ok(client) => {
            // Use the client
            println!("Spanner Client Created");
            Ok(())
        }
        Err(e) => {
            eprintln!("Error creating Spanner client: {:?}", e);
            Err(e.into())
        }
    }
    // Ok(())
}
