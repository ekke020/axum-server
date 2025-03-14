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
pub enum CreateTagResponse {
    /// Single tag
    Status201_SingleTag
    (models::CreateTag201Response)
}


/// Tags
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Tags<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Create a tag.
    ///
    /// CreateTag - POST /api/v1/tags
    async fn create_tag(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
            body: &Option<models::CreateTagRequest>,
    ) -> Result<CreateTagResponse, E>;
}
