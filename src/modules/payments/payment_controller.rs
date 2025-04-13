use std::sync::Arc;

use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use tokio::sync::Mutex;

use crate::AppState;

use super::{
    payment_model::{CreateTransactionInput, UpdateTransactionStatusInput},
    payment_service::{process_payment, update_payment_status},
};

#[debug_handler]
pub async fn create_payment_controller(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<CreateTransactionInput>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut mut_service = app_state.lock().await;
    let db = &mut mut_service.db;

    match process_payment(db, payload).await {
        Ok((transaction, payment_record)) => {
            let response = json!({
                "transaction": transaction,
                "payment_record": payment_record
            });
            Ok((StatusCode::CREATED, Json(response)).into_response())
        }
        Err(err) => {
            let response = json!({
                "error": "Unable to process payment",
                "message": err
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(response)))
        }
    }
}

#[debug_handler]
pub async fn update_payment_status_controller(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Path(transaction_id): Path<i32>,
    Json(payload): Json<UpdateTransactionStatusInput>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut mut_service = app_state.lock().await;
    let db = &mut mut_service.db;

    match update_payment_status(db, transaction_id, &payload.new_status).await {
        Ok(_) => Ok((StatusCode::OK, Json(json!({ "success": true }))).into_response()),
        Err(err) => {
            let response = json!({
                "error": "Failed to update payment status",
                "message": err
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(response)))
        }
    }
} 