use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct NoteModel {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub category: Option<String>,
    pub published: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl NoteModel {
    pub fn from_row(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            category: row.get("category"),
            published: row.get("published"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}
