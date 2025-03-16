use crate::handlers::ApiImp;
use crate::routes;
use listenfd::ListenFd;
use std::sync::Arc;
use tokio::net::TcpListener;

pub async fn run() {
    let swagger = routes::routes();
    let app = openapi::server::new(Arc::new(ApiImp)).merge(swagger);

    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        // otherwise fall back to local listening
        None => TcpListener::bind("127.0.0.1:8080").await.unwrap(),
    };

    // run it
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
