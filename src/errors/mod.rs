use std::os::macos::raw::stat;

use crate::handlers::ApiImp;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::Local;
use google_cloud_spanner::client::google_cloud_auth;
use openapi::apis::ErrorHandler;
use serde::Serialize;
use serde_json::json;
use tonic::Status;

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
        Ok(error.to_response())
    }
}

#[derive(Debug, Clone)]
pub enum AppError {
    NotFound(Error),
    DatabaseError(Error),
    ValidationError(Error),
    InternalServerError(Error),
}
impl AppError {
    // pub fn new_not_found_error(message: &str) -> Self {
    //     AppError::NotFound(Error::new(message, 404))
    // }

    // pub fn new_database_error(message: &str) -> Self {
    //     AppError::DatabaseError(Error::new(message, 500))
    // }

    // pub fn new_validation_error(message: &str) -> Self {
    //     AppError::ValidationError(Error::new(message, 400))
    // }

    // pub fn new_internal_server_error(message: &str) -> Self {
    //     AppError::InternalServerError(Error::new(message, 500))
    // }
    pub fn to_response(self) -> Response {
        match self {
            AppError::NotFound(e) => e.into_response(),
            AppError::DatabaseError(e) => e.into_response(),
            AppError::ValidationError(e) => e.into_response(),
            AppError::InternalServerError(e) => e.into_response(),
        }
    }

    pub fn set_path(&mut self, path: &str) {
        match self {
            AppError::NotFound(e) => e.path = Some(path.to_string()),
            AppError::DatabaseError(e) => e.path = Some(path.to_string()),
            AppError::ValidationError(e) => e.path = Some(path.to_string()),
            AppError::InternalServerError(e) => e.path = Some(path.to_string()),
        }
    }

    pub fn set_method(&mut self, method: &str) {
        match self {
            AppError::NotFound(e) => e.method = Some(method.to_string()),
            AppError::DatabaseError(e) => e.method = Some(method.to_string()),
            AppError::ValidationError(e) => e.method = Some(method.to_string()),
            AppError::InternalServerError(e) => e.method = Some(method.to_string()),
        }
    }
}

pub struct AppErrorBuilder {
    app_error: AppError,
}
impl AppErrorBuilder {
    fn new(app_error: AppError) -> Self {
        Self { app_error }
    }

    pub fn not_found(message: &str) -> Self {
        Self::new(AppError::NotFound(Error::new(message, 404)))
    }

    pub fn suggestion(mut self, suggestion: &str) -> Self {
        match &mut self.app_error {
            AppError::NotFound(e) => e.suggestion = Some(suggestion.to_string()),
            AppError::DatabaseError(e) => e.suggestion = Some(suggestion.to_string()),
            AppError::ValidationError(e) => e.suggestion = Some(suggestion.to_string()),
            AppError::InternalServerError(e) => e.suggestion = Some(suggestion.to_string()),
        }
        self
    }
    // fn database_error(message: &str) -> &mut Self {
    //     self.error = AppError::new_database_error(message);
    //     self
    // }

    // fn validation_error(message: &str) -> &mut Self {
    //     self.error = AppError::new_validation_error(message);
    //     self
    // }

    // fn internal_server_error(message: &str) -> &mut Self {
    //     self.error = AppError::new_internal_server_error(message);
    //     self
    // }

    pub fn build(self) -> AppError {
        self.app_error
    }
}
// TODO: This is a temp solution. default is probably not suitable here.
impl Default for AppError {
    fn default() -> Self {
        AppError::DatabaseError(Error::new("message", 404))
    }
}
// TODO: Implement the From trait for the following types
impl From<google_cloud_spanner::client::Error> for AppError {
    fn from(e: google_cloud_spanner::client::Error) -> Self {
        let error = match e {
            google_cloud_spanner::client::Error::GRPC(status) => Error::new(status.message(), 500),
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
// TODO: Map this to correct error.
impl From<Status> for AppError {
    fn from(status: Status) -> Self {
        let error = Error::new(status.message(), 503);
        AppError::DatabaseError(error)
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct Error {
    message: String,
    pub status: u16,
    time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ErrorDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
            method: None,
            details: None,
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
