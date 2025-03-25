#![allow(dead_code)]
mod db;
mod errors;
mod handlers;
mod server;
mod services;

#[tokio::main]
async fn main() {
    server::run().await;
}