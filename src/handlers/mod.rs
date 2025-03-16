use openapi::{apis::ErrorHandler, models};

mod farewell;
mod greeting;
mod tag;

mod prelude {
    pub use async_trait::async_trait;
    pub use axum_extra::extract::{CookieJar, Host};
    pub use openapi::models;
    pub use http::Method;
    pub use super::ApiImp;
}

#[derive(Debug, Clone)]
pub struct ApiImp;
impl ErrorHandler<models::Error> for ApiImp {}
