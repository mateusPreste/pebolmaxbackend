use std::sync::Arc;

use crate::{modules::arenas::arenas_model::Estabelecimento, AppState, InputValidation};
use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::NaiveDate;
use serde::de::DeserializeOwned;
use serde_json::json;
use tokio::sync::Mutex;

use super::{
    arenas_model::RegisterQuadraInput,
    arenas_repository::list_free_times,
    arenas_service::{get_available_hours, register_estabelecimento, register_quadra},
};

pub async fn register_estabelecimento_controller<T>(
    State(service): State<Arc<Mutex<AppState>>>,
    Json(mut payload): Json<T>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
where
    T: InputValidation + DeserializeOwned + Into<Estabelecimento>, // Exige que T implemente InputValidation e seja conversível para Estabelecimento
{
    //validar a entrada
    payload.validate().map_err(|err| {
        let response = json!({
            "error": "Invalid input",
            "message": err,
        });
        (StatusCode::BAD_REQUEST, Json(response))
    })?;

    let mut mut_service = service.lock().await;
    let db = &mut mut_service.db;

    match register_estabelecimento(db, payload.into()).await {
        Ok(estabelecimento) => Ok((StatusCode::CREATED, Json(estabelecimento))),
        Err(err) => {
            let response = json!({
                "error": "Internal server error",
                "message": err.to_string(),
            });
            Err((StatusCode::CONFLICT, Json(response)))
        }
    }
}

pub async fn register_quadras_controller<T>(
    State(service): State<Arc<Mutex<AppState>>>,
    Json(mut payload): Json<T>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
where
    T: InputValidation + DeserializeOwned + Into<RegisterQuadraInput>, // Exige que T implemente InputValidation
{
    //validar a entrada
    payload.validate().map_err(|err| {
        let response = json!({
            "error": "Invalid input",
            "message": err,
        });
        (StatusCode::BAD_REQUEST, Json(response))
    })?;

    let mut mut_service = service.lock().await;
    let db = &mut mut_service.db;

    match register_quadra(db, payload.into()).await {
        Ok(quadras) => Ok((StatusCode::CREATED, Json(quadras))),
        Err(err) => {
            let response = json!({
                "error": "Internal server error",
                "message": err.to_string(),
            });
            Err((StatusCode::CONFLICT, Json(response)))
        }
    }
}

pub async fn list_free_times_controller(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Path((date_str, quadra_id)): Path<(String, i32)>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Parse the date
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Invalid date format",
                "message": "Use YYYY-MM-DD"
            })),
        )
    })?;

    // Lock the shared state to access the database client.
    let mut state = app_state.lock().await;
    let free_intervals = get_available_hours(&mut state.db, quadra_id, date)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Could not fetch free times",
                    "message": err
                })),
            )
        })?;

    Ok((StatusCode::OK, Json(free_intervals)).into_response())
}
