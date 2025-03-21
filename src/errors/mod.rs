use crate::handlers::ApiImp;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::Local;
use openapi::apis::ErrorHandler;
use serde::Serialize;
use serde_json::json;

#[async_trait::async_trait]
impl ErrorHandler<AppError> for ApiImp {
    async fn handle_error(
        &self,
        method: &::http::Method,
        host: &axum_extra::extract::Host,
        cookies: &axum_extra::extract::CookieJar,
        error: AppError,
    ) -> Result<Response, http::StatusCode> {
        println!("Error handler: {:?}", error);
        println!("Error handler: {:?}", method);
        println!("Error handler: {:?}", host);
        println!("Error handler: {:?}", cookies);
        Ok(Error::new("").into_response())
        // let json_bytes = serde_json::to_vec(&Error::new()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // handle serde errors.
        // let body = axum::body::Body::from(json_bytes);
        // axum::response::Response::builder()
        //     // .header("content/type", "application/json")
        //     .status(status)
        //     .body(body)
        //     .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)
        // // Ok(response)
    }
}

#[derive(Debug, Clone)]
pub enum AppError {
    Empty,
    NotFound(Error),
    DatabaseError(Error),
    ValidationError(Error),
    InternalServerError(Error),
}

#[derive(Serialize, Debug, Clone)]
pub struct Error {
    message: String,
    status: u16,
    time: String,
    path: Option<String>,
    details: ErrorDetails,
    suggestion: Option<String>,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, axum::Json(json!(self))).into_response()
    }
}
impl Error {
    fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            status: 400,
            time: Local::now().to_string(),
            path: None,
            details: ErrorDetails {
                field: format!(""),
                value: format!(""),
                reason: format!(""),
            },
            suggestion: None,
        }
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
#[derive(Serialize, Debug, Clone)]
struct ErrorDetails {
    field: String,
    value: String,
    reason: String,
}
