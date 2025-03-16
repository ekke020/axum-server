use super::prelude::*;
use openapi::apis::tags::{CreateTagResponse, Tags};

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
            models::CreateTag201Response::new(),
        ))
    }
}
