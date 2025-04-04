use std::sync::Arc;

use crate::modules::rent::rent_model::ReservaInput;
use crate::modules::rent::rent_service::register_reserva;
use crate::AppState;
use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Mutex;

use super::rent_service;

#[debug_handler]
pub async fn register_reserva_controller(
    // Here we assume that AppState holds a mutable database connection (or Arc<Mutex<...>> etc.)
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<ReservaInput>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut mut_service = app_state.lock().await;
    let db = &mut mut_service.db;

    match register_reserva(db, payload).await {
        Ok(reserva) => Ok((StatusCode::CREATED, Json(reserva)).into_response()),
        Err(err) => {
            let response = json!({
                "error": "Unable to register reservation",
                "message": err
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(response)))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStatusInput {
    pub reserva_id: i32,
    pub new_status: String,
}

pub async fn update_reserva_status_controller(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Path(reserva_id): Path<i32>,
    Json(payload): Json<UpdateStatusInput>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Lock the shared state (which holds, for example, the database client).
    let mut app_state = app_state.lock().await;
    match rent_service::update_reserva_status_service(
        &mut app_state.db,
        reserva_id,
        payload.new_status,
    )
    .await
    {
        Ok(_) => Ok((StatusCode::OK, Json(json!({ "success": true }))).into_response()),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to update reservation status",
                "message": err
            })),
        )),
    }
}
