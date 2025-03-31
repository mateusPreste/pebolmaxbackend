use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use tokio::sync::Mutex;
use tokio_postgres::Row;

use crate::{
    model::NoteModel,
    schema::{CreateNoteSchema, FilterOptions, UpdateNoteSchema},
    AppState,
};

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

// Helper function to convert a Row to NoteModel
fn row_to_note(row: &Row) -> NoteModel {
    NoteModel {
        id: row.get("id"),
        title: row.get("title"),
        content: row.get("content"),
        category: row.get("category"),
        published: row.get("published"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

pub async fn note_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<Mutex<AppState>>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let data = data.lock().await;
    let client = &data.db;

    let query_result = client
        .query(
            "SELECT * FROM notes ORDER by id LIMIT $1 OFFSET $2",
            &[&(limit as i64), &(offset as i64)],
        )
        .await;

    if let Err(e) = query_result {
        let error_response = json!({
            "status": "fail",
            "message": format!("Something bad happened while fetching all note items: {}", e),
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let rows = query_result.unwrap();
    let notes: Vec<NoteModel> = rows.iter().map(|row| row_to_note(row)).collect();

    let json_response = json!({
        "status": "success",
        "results": notes.len(),
        "notes": notes
    });
    Ok(Json(json_response))
}

pub async fn create_note_handler(
    State(data): State<Arc<Mutex<AppState>>>,
    Json(body): Json<CreateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let data = data.lock().await;
    let client = &data.db;

    let category = body.category.unwrap_or_else(|| "".to_string());

    let query_result = client
        .query_one(
            "INSERT INTO notes (title, content, category) VALUES ($1, $2, $3) RETURNING *",
            &[&body.title, &body.content, &category],
        )
        .await;

    match query_result {
        Ok(row) => {
            let note = row_to_note(&row);
            let note_response = json!({"status": "success", "data": json!({
                "note": note
            })});

            return Ok((StatusCode::CREATED, Json(note_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = json!({
                    "status": "fail",
                    "message": "Note with that title already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": format!("{:?}", e)})),
            ));
        }
    }
}

pub async fn get_note_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let client = &data.db;

    let query_result = client
        .query_one("SELECT * FROM notes WHERE id = $1", &[&id])
        .await;

    match query_result {
        Ok(row) => {
            let note = row_to_note(&row);
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "note": note
            })});

            return Ok(Json(note_response));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Note with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}

pub async fn edit_note_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let client = &data.db;

    let query_result = client
        .query_one("SELECT * FROM notes WHERE id = $1", &[&id])
        .await;

    if let Err(_) = query_result {
        let error_response = json!({
            "status": "fail",
            "message": format!("Note with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let note = row_to_note(&query_result.unwrap());
    let now = chrono::Utc::now();

    let title = body.title.unwrap_or(note.title);
    let content = body.content.unwrap_or(note.content);
    let category = body.category.unwrap_or(note.category.unwrap_or_default());
    let published = body.published.unwrap_or(note.published.unwrap_or(false));

    let update_result = client
        .query_one(
            "UPDATE notes SET title = $1, content = $2, category = $3, published = $4, updated_at = $5 WHERE id = $6 RETURNING *",
            &[&title, &content, &category, &published.to_string(), &now.to_string(), &id.to_string()],
        )
        .await;

    match update_result {
        Ok(row) => {
            let updated_note = row_to_note(&row);
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "note": updated_note
            })});

            return Ok(Json(note_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}

pub async fn delete_note_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let client = &data.db;

    let result = client
        .execute("DELETE FROM notes WHERE id = $1", &[&id])
        .await;

    match result {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("Note with ID: {} not found", id)
                });
                return Err((StatusCode::NOT_FOUND, Json(error_response)));
            }
            Ok(StatusCode::NO_CONTENT)
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )),
    }
}
