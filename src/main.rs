mod handlers;
mod routes;
mod server;

#[tokio::main]
async fn main() {
    server::run().await;
}
