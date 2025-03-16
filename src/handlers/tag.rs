use super::prelude::*;
use openapi::apis::tags::{CreateTagResponse, GetTagByIdResponse, GetTagsResponse, Tags, UpdateTagByIdResponse};

#[async_trait]
impl Tags<models::Error> for ApiImp {
    async fn create_tag(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _body: &Option<models::Tag>,
    ) -> Result<CreateTagResponse, models::Error> {
        Ok(CreateTagResponse::Status201(
            models::Tag::new("tag".to_string(), "display_name".to_string(), false, false),
        ))
    }
    
    async fn get_tag_by_id(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _path_params: &models::GetTagByIdPathParams,
    ) -> Result<GetTagByIdResponse, models::Error> {
        Ok(GetTagByIdResponse::Status200(
            models::Tag::new("tag".to_string(), "display_name".to_string(), false, false),
        ))
    }

    async fn get_tags(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
    ) -> Result<GetTagsResponse, models::Error> {
        Ok(GetTagsResponse::Status200(
            vec![models::Tag::new("tag".to_string(), "display_name".to_string(), false, false),],
        ))
    }

    async fn update_tag_by_id(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _path_params: &models::UpdateTagByIdPathParams,
        _body: &Option<models::Tag>,
    ) -> Result<UpdateTagByIdResponse, models::Error> {
        Ok(UpdateTagByIdResponse::Status200(
            models::Tag::new("tag".to_string(), "display_name".to_string(), false, false),
        ))
    }
}
