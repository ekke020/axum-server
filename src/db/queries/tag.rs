use std::vec;

use google_cloud_spanner::{
    row::Row,
    statement::{Statement, ToKind},
};
use openapi::models::Tag;

use super::super::{get_client, Error};

pub async fn fetch_tag_by_id<T: Into<String> + ToKind>(id: T) -> Result<Tag, Error> {
    let client = get_client().await?;
    let mut tx = client.single().await?;
    let mut query = Statement::new("SELECT * FROM tags WHERE id = @id");
    query.add_param("id", &id);
    let mut result = tx.query(query).await.map_err(|_| Error::NotFound)?; // Handle query error

    let row = result.next().await.map_err(|_| Error::DatabaseError)?; // Handle query error
    row.ok_or(Error::NotFound).and_then(|row| {
        let tag = TagRow::from(row).into();
        Ok(tag)
    })
}

pub async fn fetch_tags() -> Result<Vec<Tag>, Error> {
    let client = get_client().await?;
    let mut tx = client.single().await?;
    let query = Statement::new("SELECT * FROM tags Limit 10");
    let mut result = tx.query(query).await.map_err(|_| Error::NotFound)?; // Handle query error

    let mut tags = vec![];
    while let Some(row) = result.next().await.map_err(|_| Error::DatabaseError)? {
        let tag: Tag = TagRow::from(row).into();
        tags.push(tag);
    };
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
