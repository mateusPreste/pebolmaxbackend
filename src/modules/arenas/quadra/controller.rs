use crate::modules::arenas::arenas_model::Quadra;
use crate::modules::arenas::quadra::service::update_quadra_service;
use crate::modules::arenas::{ arenas_model::RegisterQuadraInput, quadra::service::register_quadra };
use crate::{ AppState, InputValidation };
use axum::extract::Path;
use axum::{ extract::State, http::StatusCode, response::IntoResponse, Json };
use serde::de::DeserializeOwned;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn register_quadras_controller<T>(
    State(service): State<Arc<Mutex<AppState>>>,
    Json(mut payload): Json<T>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
    where
        T: InputValidation + DeserializeOwned + Into<RegisterQuadraInput> 
{
    //validar a entrada
    payload.validate().map_err(|err| {
        let response =
            json!({
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
            let response =
                json!({
                "error": "Internal server error",
                "message": err.to_string(),
            });
            Err((StatusCode::CONFLICT, Json(response)))
        }
    }
}

pub async fn update_quadras_controller(
    Path(id): Path<i32>,
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<Quadra>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("Atualizando Quadra com ID: {}", id);

    let mut state = app_state.lock().await;
    let db = &mut state.db;

    match update_quadra_service(db, id, payload).await {
        Ok(updated_quadra) =>
            Ok((
                StatusCode::OK,
                Json(
                    json!({
                    "message": "Quadra atualizada com sucesso",
                    "quadra": updated_quadra
                })
                ),
            )),
        Err(err) => {
            println!("Erro ao atualizar quadra: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR, // Or NOT_FOUND depending on the service logic
                Json(
                    json!({
                    "error": "Update failed",
                    "message": err
                })
                ),
            ))
        }
    }
}
