use crate::modules::arenas::arenas_model::Estabelecimento;
use crate::modules::arenas::estabelecimento::service::{
    get_estabelecimento,
    register_estabelecimento,
    update_estabelecimento_service,
};
use crate::{ AppState, InputValidation };
use axum::{ extract::{ Path, State }, http::StatusCode, response::IntoResponse, Json };
use serde::de::DeserializeOwned;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::service::{ delete_estabelecimento, get_all_estabelecimentos_service };

pub async fn register_estabelecimento_controller<T>(
    State(service): State<Arc<Mutex<AppState>>>,
    Json(mut payload): Json<T>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
    where
        T: InputValidation + DeserializeOwned + Into<Estabelecimento> // Exige que T implemente InputValidation e seja conversível para Estabelecimento
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

    match register_estabelecimento(db, payload.into()).await {
        Ok(estabelecimento) => Ok((StatusCode::CREATED, Json(estabelecimento))),
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

pub async fn get_all_estabelecimentos_handler(State(
    app_state,
): State<Arc<Mutex<AppState>>>) -> Result<
    impl IntoResponse,
    (StatusCode, Json<serde_json::Value>)
> {
    // Lock no estado compartilhado para acessar o cliente do banco de dados
    let mut state = app_state.lock().await;
    let db = &mut state.db;

    // Chamar o serviço para buscar todos os estabelecimentos
    match get_all_estabelecimentos_service(db).await {
        Ok(estabelecimentos) => {
            // Retornar os estabelecimentos encontrados
            Ok((StatusCode::OK, Json(estabelecimentos)))
        }
        Err(err) => {
            // Log de erro e retorno de erro interno
            println!("Erro ao buscar estabelecimentos: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    json!({
                    "error": "Internal server error",
                    "message": err
                })
                ),
            ))
        }
    }
}

pub async fn delete_estabelecimento_controller(
    Path(id): Path<i32>,
    State(app_state): State<Arc<Mutex<AppState>>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut state = app_state.lock().await;
    let db = &mut state.db;

    match delete_estabelecimento(db, id).await {
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

pub async fn update_estabelecimento_controller(
    Path(id): Path<i32>,
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<Estabelecimento>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("Atualizando estabelecimento com ID: {}", id);

    let mut state = app_state.lock().await;
    let db = &mut state.db;

    match update_estabelecimento_service(db, id, payload).await {
        Ok(_) =>
            Ok((
                StatusCode::OK,
                Json(
                    json!({
            "message": "Estabelecimento atualizado com sucesso"
        })
                ),
            )),
        Err(err) => {
            println!("Erro ao atualizar estabelecimento: {}", err);
            Err((
                StatusCode::NOT_FOUND,
                Json(
                    json!({
                    "error": "Not Found",
                    "message": err
                })
                ),
            ))
        }
    }
}

//get estabelecimento controller
pub async fn get_estabelecimento_controller(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Path(id): Path<i32>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("id recebido no controlador: {}", id);

    let mut state = app_state.lock().await;
    let db = &mut state.db;

    match get_estabelecimento(db, id).await {
        Ok(Some(estabelecimento)) => {
            println!("Estabelecimento encontrado: {:?}", estabelecimento);
            Ok((StatusCode::OK, Json(estabelecimento)))
        }
        Ok(None) => {
            println!("Estabelecimento não encontrado para o id: {}", id);
            Err((
                StatusCode::NOT_FOUND,
                Json(
                    json!({
                    "erro": "Not found",
                    "message": "Estabelecimento não encontrado"
                })
                ),
            ))
        }
        Err(err) => {
            println!("Erro ao buscar estabelecimento: {}", err);
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
