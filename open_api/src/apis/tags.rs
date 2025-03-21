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

    Status201
    (models::Tag)
    ,

    Status409
    (models::Error)
    ,

    Status500
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeleteTagByIdResponse {

    Status204
    (models::Message)
    ,

    Status500
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetTagByIdResponse {

    Status200
    (models::Tag)
    ,

    Status500
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetTagsResponse {

    Status200
    (Vec<models::Tag>)
    ,

    Status500
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateTagByIdResponse {

    Status200
    (models::Tag)
    ,

    Status500
    (models::Error)
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
            body: &Option<models::Tag>,
    ) -> Result<CreateTagResponse, E>;

    /// Delete a tag by id.
    ///
    /// DeleteTagById - DELETE /api/v1/tags/{id}
    async fn delete_tag_by_id(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::DeleteTagByIdPathParams,
    ) -> Result<DeleteTagByIdResponse, E>;

    /// Get a tag by id.
    ///
    /// GetTagById - GET /api/v1/tags/{id}
    async fn get_tag_by_id(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::GetTagByIdPathParams,
    ) -> Result<GetTagByIdResponse, E>;

    /// Get all tags.
    ///
    /// GetTags - GET /api/v1/tags
    async fn get_tags(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::GetTagsQueryParams,
    ) -> Result<GetTagsResponse, E>;

    /// Update a tag by id.
    ///
    /// UpdateTagById - PATCH /api/v1/tags/{id}
    async fn update_tag_by_id(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::UpdateTagByIdPathParams,
            body: &Option<models::Tag>,
    ) -> Result<UpdateTagByIdResponse, E>;
}
