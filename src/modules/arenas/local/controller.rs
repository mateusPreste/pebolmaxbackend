use crate::AppState;
use crate::modules::arenas::arenas_model::Local;
use crate::modules::arenas::local::service::{
    delete_local_service,
    get_local_by_id_service,
    update_local_service,
};
use axum::{ extract::{ Path, State }, http::StatusCode, response::IntoResponse, Json };
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn get_locais_controller(
    Path(id): Path<i32>,
    State(app_state): State<Arc<Mutex<AppState>>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut state = app_state.lock().await;
    let db = &mut state.db;

    match get_local_by_id_service(db, id).await {
        Ok(Some(local)) => {
            println!("Local Encontrado: {:?}", local);
            Ok((StatusCode::OK, Json(local)))
        }
        Ok(None) => {
            println!("Local não encontrado para o id : {}", id);
            Err((
                StatusCode::NOT_FOUND,
                Json(
                    json!({
                        "error": "Not Found",
                        "message": "Local não encontrado"
                    })
                ),
            ))
        }
        Err(err) => {
            println!("Erro ao buscar local: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    json!({
                    "error": "Internal Server Error",
                    "message": err
                })
                ),
            ))
        }
    }
}

pub async fn update_locais_controller(
    Path(id): Path<i32>,
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<Local>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("Atualizando Local com ID: {}", id);

    let mut state = app_state.lock().await;
    let db = &mut state.db;

    match update_local_service(db, id, payload).await {
        Ok(updated_local) =>
            Ok((
                StatusCode::OK,
                Json(
                    json!({
                    "message": "Local atualizado com sucesso",
                    "local": updated_local
                })
                ),
            )),
        Err(err) =>
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    json!({
                    "error": "Internal Server Error",
                    "message": err
                })
                ),
            )),
    }
}

pub async fn delete_local_controller(
    Path(id): Path<i32>,
    State(app_state): State<Arc<Mutex<AppState>>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut state = app_state.lock().await;
    let db = &mut state.db;

    match delete_local_service(db, id).await {
        Ok(_) => Ok((StatusCode::NO_CONTENT, ())),
        Err(err) =>
            Err((
                StatusCode::NOT_FOUND,
                Json(
                    json!({
                "error": "Not Found",
                "message": err
            })
                ),
            )),
    }
}
