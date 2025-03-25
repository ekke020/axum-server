use std::os::macos::raw::stat;

use crate::handlers::ApiImp;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::Local;
use openapi::apis::ErrorHandler;
use serde::Serialize;
use serde_json::json;
use google_cloud_spanner::client::google_cloud_auth;

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
        Ok(Error::new("", 400).into_response())
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

impl Default for AppError {
    fn default() -> Self {
        AppError::Empty
    }
}
// TODO: Implement the From trait for the following types
impl From<google_cloud_spanner::client::Error> for AppError {
    fn from(e: google_cloud_spanner::client::Error) -> Self {
        let error = match e {
            google_cloud_spanner::client::Error::GRPC(status) => {
                Error::new(status.message(), 500)
            },
            google_cloud_spanner::client::Error::InvalidSession(session_error) => todo!(),
            google_cloud_spanner::client::Error::ParseError(error) => todo!(),
            google_cloud_spanner::client::Error::Connection(error) => todo!(),
            google_cloud_spanner::client::Error::InvalidConfig(_) => todo!(),
        };

        AppError::DatabaseError(error)
    }
}

impl From<google_cloud_auth::error::Error> for AppError {
    fn from(_: google_cloud_spanner::client::google_cloud_auth::error::Error) -> Self {
        AppError::default()
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct Error {
    message: String,
    pub status: u16,
    time: String,
    pub path: Option<String>,
    pub details: ErrorDetails,
    pub suggestion: Option<String>,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, axum::Json(json!(self))).into_response()
    }
}

impl Error {
    fn new(message: &str, status: u16) -> Self {
        Self {
            message: message.to_string(),
            status,
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
pub struct ErrorDetails {
    pub field: String,
    pub value: String,
    pub reason: String,
}
