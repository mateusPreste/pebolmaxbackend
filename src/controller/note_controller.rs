use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    modules::notes::note_service::{self, get_notes},
    schema::{CreateNoteSchema, FilterOptions, UpdateNoteSchema},
    AppState,
};

pub async fn create_note_controller(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateNoteSchema>,
) -> impl IntoResponse {
    match note_service::create_note(&state, payload).await {
        Ok(note) => {
            let response = json!({ "status": "success", "data": { "note": note } });
            (StatusCode::CREATED, Json(response))
        }
        Err(err_msg) => {
            let response = json!({ "status": "error", "message": err_msg });
            (StatusCode::CONFLICT, Json(response))
        }
    }
}

pub async fn get_note_controller(
    Path(id): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match note_service::get_note(&state, id).await {
        Ok(note) => {
            let response = json!({ "status": "success", "data": { "note": note } });
            (StatusCode::OK, Json(response))
        }
        Err(err_msg) => {
            let response = json!({ "status": "fail", "message": err_msg });
            (StatusCode::NOT_FOUND, Json(response))
        }
    }
}

pub async fn edit_note_controller(
    Path(id): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateNoteSchema>,
) -> impl IntoResponse {
    match note_service::edit_note(&state, id, payload).await {
        Ok(note) => {
            let response = json!({ "status": "success", "data": { "note": note } });
            (StatusCode::OK, Json(response))
        }
        Err(err_msg) => {
            let response = json!({ "status": "error", "message": err_msg });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn delete_note_controller(
    Path(id): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match note_service::delete_note(&state, id).await {
        Ok(_) => (StatusCode::NO_CONTENT, Json(json!({ "status": "success" }))),
        Err(err_msg) => {
            let response = json!({ "status": "fail", "message": err_msg });
            (StatusCode::NOT_FOUND, Json(response))
        }
    }
}

pub async fn note_list_controller(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> impl IntoResponse {
    let Query(opts) = opts.unwrap_or_default();
    let limit = opts.limit.unwrap_or(10) as i64;
    let offset = ((opts.page.unwrap_or(1) - 1) * opts.limit.unwrap_or(10)) as i64;

    // Use the entire application state directly for get_notes
    let notes = match get_notes(&data, limit, offset).await {
        Ok(notes) => notes,
        Err(e) => {
            let error_response = json!({
                "status": "fail",
                "message": format!("Something bad happened while fetching all note items: {}", e)
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    };

    // Build the final JSON response
    let json_response = json!({
        "status": "success",
        "results": notes.len(),
        "notes": notes
    });
    Ok(Json(json_response))
}

pub async fn healthchecker_controller(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> impl IntoResponse {
    let response = json!({ "status": "success", "message": "Server is running" });
    (StatusCode::OK, Json(response))
}
