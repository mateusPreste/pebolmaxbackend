use std::sync::Arc;

use crate::AppState;
use axum::{ extract::{ Path, State }, http::StatusCode, response::IntoResponse, Json };
use chrono::NaiveDate;
use serde_json::json;
use tokio::sync::Mutex;

use super::arenas_service::get_available_hours;

pub async fn list_free_times_controller(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Path((date_str, quadra_id)): Path<(String, i32)>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Parse the date
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(
                json!({
                "error": "Invalid date format",
                "message": "Use YYYY-MM-DD"
            })
            ),
        )
    })?;

    // Lock the shared state to access the database client.
    let mut state = app_state.lock().await;
    let free_intervals = get_available_hours(&mut state.db, quadra_id, date).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(
                json!({
                    "error": "Could not fetch free times",
                    "message": err
                })
            ),
        )
    })?;

    Ok((StatusCode::OK, Json(free_intervals)).into_response())
}
