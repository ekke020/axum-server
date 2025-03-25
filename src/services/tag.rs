use openapi::models::Tag;

use crate::errors::AppError;
use crate::db::queries;

pub fn create_tag(request_body: &Option<Tag>) -> Result<Tag, AppError> {
    let tag = request_body.as_ref().ok_or(AppError::Empty)?;
    Ok(request_body.as_ref().unwrap().clone())
    // Tag::new(
    //     tag.tag.clone(),
    //     tag.display_name.clone(),
    //     tag.is_store_tag,
    //     tag.locked,
    // )
}

pub async fn list_tags() -> Result<Vec<Tag>, AppError> {
    let tags = queries::tag::fetch_tags().await?;
    Ok(vec![Tag::new(
        "tag".to_string(),
        false,
        false,
    )])
}