use crate::handlers::ApiImp;
use std::sync::Arc;
mod handlers;
mod routes;
mod server;


#[tokio::main]
async fn main() {
    let swagger = routes::routes();
    let app = openapi::server::new(Arc::new(ApiImp)).merge(swagger);
    server::run(app).await;
}
