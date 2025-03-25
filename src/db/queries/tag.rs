use super::prelude::*;
use google_cloud_spanner::{row::Row, statement::ToKind};
use std::vec;

const MISSING_TAG_SUGGESTION: &str = "Make sure the tag ID is correct. Use the list tags endpoint to find available tag IDs.";
pub async fn fetch_tag_by_id<T: Into<String> + ToKind>(id: T) -> Result<Tag, AppError> {
    let client = get_client().await?;
    let mut tx = client.single().await?;
    let mut query = Statement::new("SELECT * FROM tags WHERE id = @id");
    query.add_param("id", &id);
    let mut result = tx.query(query).await?; // Handle query error
                              
    let row = result
        .next()
        .await?
        .ok_or(AppErrorBuilder::not_found(&format!(
            "No tag with ID: {} found.",
            id.into()
        )).suggestion(MISSING_TAG_SUGGESTION).build())?;
    let tag = TagRow::from(row).into();
    Ok(tag)
}

pub async fn fetch_tags() -> Result<Vec<Tag>, AppError> {
    let client = get_client().await?;
    let mut tx = client.single().await?;
    let query = Statement::new("SELECT * FROM tags Limit 10");
    let mut result = tx.query(query).await?;

    let mut tags = vec![];
    while let Some(row) = result.next().await? {
        let tag: Tag = TagRow::from(row).into();
        tags.push(tag);
    }
    Ok(tags)
}

#[derive(Debug)]
struct TagRow {
    id: String,
    locked: bool,
    store_id: Option<String>,
    tag: String,
    tenant_id: String,
}

impl From<Row> for TagRow {
    fn from(row: Row) -> Self {
        let id = row.column_by_name("id").unwrap();
        let locked = row.column_by_name("locked").unwrap();
        let store_id = row.column_by_name("store_id").unwrap();
        let tag = row.column_by_name("tag").unwrap();
        let tenant_id = row.column_by_name("tenant_id").unwrap();
        TagRow {
            id,
            locked,
            store_id,
            tag,
            tenant_id,
        }
    }
}
impl Into<Tag> for TagRow {
    fn into(self) -> Tag {
        Tag {
            id: Some(self.id),
            locked: self.locked,
            tag: self.tag,
            entities: None,
            is_store_tag: self.store_id.is_some(),
            store_id: self.store_id,
        }
    }
}
