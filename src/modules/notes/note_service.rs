use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde_json::json;
use tokio_postgres::Error;
use uuid::Uuid;

use crate::{
    model::NoteModel,
    schema::{CreateNoteSchema, FilterOptions, UpdateNoteSchema},
    AppState,
};

// Create a new note in the database.
pub async fn create_note(
    state: &Arc<AppState>,
    payload: CreateNoteSchema,
) -> Result<NoteModel, String> {
    let client = &state.db;
    let category = payload.category.unwrap_or_else(|| "".to_string());
    let query = "INSERT INTO notes (title, content, category) VALUES ($1, $2, $3) RETURNING *";
    let row_result = client
        .query_one(query, &[&payload.title, &payload.content, &category])
        .await;
    match row_result {
        Ok(row) => Ok(NoteModel::from_row(&row)),
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                Err("Note with that title already exists".into())
            } else {
                Err(format!("{:?}", e))
            }
        }
    }
}

// Fetch a note using its UUID.
pub async fn get_note(state: &Arc<AppState>, id: uuid::Uuid) -> Result<NoteModel, String> {
    let client = &state.db;
    let query = "SELECT * FROM notes WHERE id = $1";
    match client.query_one(query, &[&id]).await {
        Ok(row) => Ok(NoteModel::from_row(&row)),
        Err(_) => Err(format!("Note with ID: {} not found", id)),
    }
}

// Update an existing note.
pub async fn edit_note(
    state: &Arc<AppState>,
    id: uuid::Uuid,
    payload: UpdateNoteSchema,
) -> Result<NoteModel, String> {
    let client = &state.db;

    // Fetch the existing note, or return an error if not found.
    let existing_row = client
        .query_one("SELECT * FROM notes WHERE id = $1", &[&id])
        .await
        .map_err(|_| format!("Note with ID: {} not found", id))?;
    let note = NoteModel::from_row(&existing_row);

    // Overwrite with provided updated values.
    let title = payload.title.unwrap_or(note.title);
    let content = payload.content.unwrap_or(note.content);
    let category = payload
        .category
        .unwrap_or(note.category.unwrap_or_default());
    let published = payload.published.unwrap_or(note.published.unwrap_or(false));
    let now = Utc::now();

    // Execute update and return the updated note.
    client
      .query_one(
          "UPDATE notes SET title = $1, content = $2, category = $3, published = $4, updated_at = $5 WHERE id = $6 RETURNING *",
          &[
              &title,
              &content,
              &category,
              &published.to_string(),
              &now.to_string(),
              &id.to_string(),
          ],
      )
      .await
      .map(|row| NoteModel::from_row(&row))
      .map_err(|e| format!("{:?}", e))
}

// Delete a note.
pub async fn delete_note(state: &Arc<AppState>, id: uuid::Uuid) -> Result<(), String> {
    let client = &state.db;
    let result = client
        .execute("DELETE FROM notes WHERE id = $1", &[&id])
        .await;
    match result {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                Err(format!("Note with ID: {} not found", id))
            } else {
                Ok(())
            }
        }
        Err(e) => Err(format!("{:?}", e)),
    }
}

pub async fn get_notes(
    state: &Arc<AppState>,
    limit: i64,
    offset: i64,
) -> Result<Vec<NoteModel>, tokio_postgres::Error> {
    let client = &state.db;
    let query_result = client
        .query(
            "SELECT * FROM notes ORDER by id LIMIT $1 OFFSET $2",
            &[&(limit as i64), &(offset as i64)],
        )
        .await;

    if let Err(e) = query_result {
        return Err(e);
    }

    let rows = query_result.unwrap();

    let notes: Vec<NoteModel> = rows.iter().map(|row| NoteModel::from_row(row)).collect();
    Ok(notes)
}
