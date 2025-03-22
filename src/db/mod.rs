mod queries;
use google_cloud_spanner::client::Client;
use google_cloud_spanner::client::ClientConfig;
use google_cloud_spanner::statement::Statement;
use google_cloud_spanner::client::google_cloud_auth;
const DATABASE: &str =
    "projects/evo-core-dev-001/instances/core-spanner-instance/databases/tag-database";

#[derive(Debug)]
enum Error {
    DatabaseError,
    NotFound,
}

impl From<google_cloud_spanner::client::Error> for Error {
    fn from(_: google_cloud_spanner::client::Error) -> Self {
        Error::DatabaseError
    }
}
impl From<google_cloud_auth::error::Error> for Error {
    fn from(_: google_cloud_spanner::client::google_cloud_auth::error::Error) -> Self {
        Error::DatabaseError
    }
}
pub async fn run() {
    let config = ClientConfig::default().with_auth().await.unwrap();
    let client = Client::new(DATABASE, config).await.unwrap();
    let mut tx = client.read_only_transaction().await.unwrap();
    let mut result = tx
        .query(Statement::new("SELECT * FROM tags LIMIT 10"))
        .await
        .unwrap();
    while let Some(row) = result.next().await.unwrap() {
        let tag: String = row.column_by_name("tag").unwrap();
        println!("tag: {}", tag);
    }
    client.close().await;
}

pub async fn test() {
    let res = queries::tag::fetch_tag_by_id("001a9b8a-d47a-4cae-9e1e-e6e52dfd8aa4").await;
    match res {
        Ok(tag) => {
            println!("tag: {:?}", tag);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    let res = queries::tag::fetch_tags().await;
    match res {
        Ok(tags) => {
            println!("tags: {:?}", tags);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

async fn get_client() -> Result<Client, Error> {
    let config = ClientConfig::default().with_auth().await?;
    let client = Client::new(DATABASE, config).await?;
    Ok(client)
}
