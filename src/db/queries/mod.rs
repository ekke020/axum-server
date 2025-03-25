pub mod tag;

mod prelude {
    pub use super::get_client;
    pub use crate::errors::AppError;
    pub use async_trait::async_trait;
    pub use google_cloud_spanner::client::Client;
    pub use google_cloud_spanner::client::ClientConfig;
    pub use google_cloud_spanner::statement::Statement;
    pub use openapi::models::Tag;
}

use google_cloud_spanner::client::Client;
use google_cloud_spanner::client::ClientConfig;

use crate::errors::AppError;

const DATABASE: &str =
    "projects/evo-core-dev-001/instances/core-spanner-instance/databases/tag-database";

pub async fn get_client() -> Result<Client, AppError> {
    let config = ClientConfig::default().with_auth().await?;
    let client = Client::new(DATABASE, config).await?;
    Ok(client)
}
