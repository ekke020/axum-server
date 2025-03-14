use std::sync::Arc;
mod server;
use async_trait::async_trait;
use axum::routing::get;
use axum::Router;
use axum_extra::extract::{CookieJar, Host};
use http::Method;
use listenfd::ListenFd;
use openapi::apis::farewells::{FarewellResponse, Farewells};
use openapi::apis::greetings::{GreetingResponse, Greetings};
use openapi::apis::tags::{CreateTagResponse, Tags};
use openapi::apis::ErrorHandler;
use openapi::models::{self, CreateTag201Response};
use swagger_ui_dist::{ApiDefinition, OpenApiSource};
use tokio::net::TcpListener;

#[derive(Debug, Clone)]
struct ApiImp;
impl ErrorHandler<models::Error> for ApiImp {}

#[async_trait]
impl Tags<models::Error> for ApiImp {
    async fn create_tag(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _body: &Option<models::CreateTagRequest>,
    ) -> Result<CreateTagResponse, models::Error> {
        Ok(CreateTagResponse::Status201_SingleTag(
            CreateTag201Response::new(),
        ))
    }
}
#[async_trait]
impl Farewells<models::Error> for ApiImp {
    async fn farewell(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
    ) -> Result<FarewellResponse, models::Error> {
        Ok(FarewellResponse::Status200(models::Farewell::new(
            "Bye bye".to_string(),
        )))
    }
}
#[async_trait]
impl Greetings<models::Error> for ApiImp {
    async fn greeting(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
    ) -> Result<GreetingResponse, models::Error> {
        Ok(GreetingResponse::Status200(models::Greeting::new(
            "Hello".to_string(),
        )))
    }
}

#[tokio::main]
async fn main() {
    let api_def = ApiDefinition {
        uri_prefix: "/swagger-ui",
        api_definition: OpenApiSource::Uri("/openapi.yaml"),
        title: Some("Test API"),
    };
    let swagger_app = Router::new()
        .route(
            "/openapi.yaml",
            get(|| async move { include_str!("../api/openapi.yaml") }),
        )
        .route(
            "/tags-api.yaml",
            get(|| async move { include_str!("../api/tags-api.yaml") }),
        )
        .route("/tag-models.yaml", 
            get(|| async move { include_str!("../api/tag-models.yaml") }),
        )
        .merge(swagger_ui_dist::generate_routes(api_def));

    let app = openapi::server::new(Arc::new(ApiImp)).merge(swagger_app);

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
    server::run();
}
