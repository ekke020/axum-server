use openapi::models::Tag;

use crate::errors::AppError;
use crate::db::queries;

pub fn create_tag(request_tag: &Tag) -> Result<Tag, AppError> {
    // let tag = request_body.as_ref()?;
    Ok(request_tag.clone())
    // Tag::new(
    //     tag.tag.clone(),
    //     tag.display_name.clone(),
    //     tag.is_store_tag,
    //     tag.locked,
    // )
}

pub async fn list_tags() -> Result<Vec<Tag>, AppError> {
    let tags = queries::tag::fetch_tags().await?;
    Ok(tags)
}

pub async fn get_tag_by_id(id: &uuid::Uuid) -> Result<Tag, AppError> {
    let tag = queries::tag::fetch_tag_by_id(id.to_string()).await?;
    Ok(tag)
}