use super::prelude::*;
use crate::services::tag::{self, list_tags};
use openapi::apis::tags::{
    CreateTagResponse, DeleteTagByIdResponse, GetTagByIdResponse, GetTagsResponse, Tags,
    UpdateTagByIdResponse,
};

#[async_trait]
impl Tags<AppError> for ApiImp {
    async fn create_tag(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        body: &Option<models::Tag>,
    ) -> Result<CreateTagResponse, AppError> {
        let created_tag = tag::create_tag(&Option::None)?;

        Ok(CreateTagResponse::Status409(models::Error::new(
            409,
            "Conflict occurred".to_string(),
        )))
        // Err(AppError::DatabaseError("".to_string()))
        // Ok(CreateTagResponse::Status201(body.as_ref().unwrap().clone()))
    }

    async fn get_tag_by_id(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _path_params: &models::GetTagByIdPathParams,
    ) -> Result<GetTagByIdResponse, AppError> {
        Ok(GetTagByIdResponse::Status200(models::Tag::new(
            "tag".to_string(),
            false,
            false,
        )))
    }

    async fn get_tags(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _query_params: &models::GetTagsQueryParams,
    ) -> Result<GetTagsResponse, AppError> {
        let tags = list_tags()?;
        Ok(GetTagsResponse::Status200(tags))
    }

    async fn update_tag_by_id(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _path_params: &models::UpdateTagByIdPathParams,
        _body: &Option<models::Tag>,
    ) -> Result<UpdateTagByIdResponse, AppError> {
        Ok(UpdateTagByIdResponse::Status200(models::Tag::new(
            "tag".to_string(),
            false,
            false,
        )))
    }

    async fn delete_tag_by_id(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _path_params: &models::DeleteTagByIdPathParams,
    ) -> Result<DeleteTagByIdResponse, AppError> {
        Ok(DeleteTagByIdResponse::Status204(models::Message::new(
            "tag deleted".to_string(),
            001,
        )))
    }
}
