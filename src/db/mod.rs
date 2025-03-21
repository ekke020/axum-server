mod queries;
// use google_cloud_spanner::client::Client;
// use google_cloud_spanner::client::ClientConfig;
// use google_cloud_spanner::statement::Statement;

const SPANNER_DATABASE: &str = "projects/evo-core-dev-001/instances/core-spanner-instance/databases/tag-database";
pub async fn run() {
    // let config = ClientConfig::default().with_auth().await.unwrap();
    // let client = Client::new(
    //     "projects/evo-core-dev-001/instances/core-spanner-instance/databases/tag-database",
    //     config,
    // )
    // .await
    // .unwrap();
    // let mut tx = client.read_only_transaction().await.unwrap();
    // let mut result = tx
    //     .query(Statement::new("SELECT * FROM tags LIMIT 10"))
    //     .await
    //     .unwrap();
    // while let Some(row) = result.next().await.unwrap() {
    //     let tag: String = row.column_by_name("tag").unwrap();
    //     println!("tag: {}", tag);
    // }
    // client.close().await;
}
