use std::sync::Arc;

use crate::{modules::arenas::arenas_model::Estabelecimento, AppState, InputValidation};
use axum::{debug_handler, extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::de::DeserializeOwned;
use serde_json::json;
use tokio::sync::Mutex;

use super::{
    arenas_model::RegisterQuadraInput,
    arenas_service::{register_estabelecimento, register_quadra},
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
