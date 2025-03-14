use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum FarewellResponse {

    Status200
    (models::Farewell)
    ,

    Status500
    (models::Error)
}


/// Farewells
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Farewells<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Returns a goodbye message.
    ///
    /// Farewell - GET /goodbye
    async fn farewell(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
    ) -> Result<FarewellResponse, E>;
}
