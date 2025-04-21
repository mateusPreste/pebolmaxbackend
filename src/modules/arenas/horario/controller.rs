use crate::{
    errors::default_errors::DefaultError,
    modules::arenas::{
        arenas_model::{ Horario, InputValidation },
        horario::service::{
            delete_all_horarios_service, // Adicionado delete_all_horarios_service
            delete_single_horario_service,
            find_horarios_by_quadra_id_service,
            update_horarios_service,
            update_single_horario_service,
        },
    },
};
use axum::{ extract::{ Path, State }, http::StatusCode, response::IntoResponse, Json };
use serde_json::json;
use std::sync::Arc;
use crate::AppState;
use tokio::sync::Mutex;

pub async fn find_horarios_by_quadra_id_controller(
    Path(id): Path<i32>,
    State(app_state): State<Arc<Mutex<AppState>>>
) -> Result<impl IntoResponse, DefaultError> {
    println!("Buscando a horario com a quadra de id {}", id);

    let mut state = app_state.lock().await;
    let db = &mut state.db;

    match find_horarios_by_quadra_id_service(db, id).await {
        Ok(quadras) => Ok((StatusCode::OK, Json(quadras))),
        Err(e) => Err(DefaultError::NotFound(e)),
    }
}

pub async fn update_horarios_controller(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Path(quadra_id): Path<i32>,
    Json(mut payload): Json<Vec<Horario>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    for horario in payload.iter_mut() {
        if let Err(err) = horario.validate() {
            let response = json!({"error": "Invalid input", "message": err});
            return Err((StatusCode::BAD_REQUEST, Json(response)));
        }
    }

    let mut state = app_state.lock().await;
    let db = &mut state.db;

    match update_horarios_service(db, quadra_id, &payload).await {
        Ok(_) => Ok((StatusCode::OK, Json(json!({"message": "Horários atualizados com sucesso"})))),
        Err(err) => {
            let status_code = if err.contains("não informado") {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            let response = json!({"error": "Falha ao atualizar horários", "message": err});
            Err((status_code, Json(response)))
        }
    }
}

// Controlador para atualizar o horário de um dia específico (PATCH ou PUT)
pub async fn update_single_horario_controller(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Path((quadra_id, dia_semana)): Path<(i32, String)>, // Extrai quadra_id e dia_semana
    Json(payload): Json<Horario> // Recebe o Horario completo no corpo
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Verifica se o dia_semana do path corresponde ao do payload
    // (Opcional, mas bom para consistência)
    if payload.dia_semana != dia_semana {
        let response =
            json!({
            "error": "Inconsistent data",
            "message": format!("Dia da semana no path ('{}') não corresponde ao do payload ('{}')", dia_semana, payload.dia_semana)
        });
        return Err((StatusCode::BAD_REQUEST, Json(response)));
    }

    // A validação do Horario (incluindo tempos) será feita no serviço

    let state = app_state.lock().await; // Não precisa ser mutável para este serviço
    let db = &state.db;

    match update_single_horario_service(db, quadra_id, payload).await {
        Ok(_) => Ok((StatusCode::OK, Json(json!({"message": "Horário atualizado com sucesso"})))),
        Err(err) => {
            let status_code = if
                err.contains("inválido") ||
                err.contains("Nenhum horário encontrado")
            {
                StatusCode::BAD_REQUEST // Erro de validação ou dado não encontrado
            } else {
                StatusCode::INTERNAL_SERVER_ERROR // Erro interno/banco
            };
            let response = json!({"error": "Falha ao atualizar horário", "message": err});
            Err((status_code, Json(response)))
        }
    }
}

// Controlador para deletar o horário de um dia específico
pub async fn delete_single_horario_controller(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Path((quadra_id, dia_semana)): Path<(i32, String)> // Extrai quadra_id e dia_semana
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let state = app_state.lock().await; // Não precisa ser mutável
    let db = &state.db;

    match delete_single_horario_service(db, quadra_id, &dia_semana).await {
        Ok(_) => Ok((StatusCode::NO_CONTENT, Json(json!({})))), // 204 No Content é apropriado para DELETE bem-sucedido
        Err(err) => {
            let status_code = if
                err.contains("inválido") ||
                err.contains("Nenhum horário encontrado")
            {
                StatusCode::BAD_REQUEST // Erro de validação ou dado não encontrado
            } else {
                StatusCode::INTERNAL_SERVER_ERROR // Erro interno/banco
            };
            let response = json!({"error": "Falha ao deletar horário", "message": err});
            Err((status_code, Json(response)))
        }
    }
}

// Controlador para deletar TODOS os horários de uma quadra
pub async fn delete_all_horarios_controller(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Path(quadra_id): Path<i32> // Extrai quadra_id
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let state = app_state.lock().await; // Não precisa ser mutável
    let db = &state.db;

    match delete_all_horarios_service(db, quadra_id).await {
        Ok(_) => Ok((StatusCode::NO_CONTENT, Json(json!({})))), // 204 No Content
        Err(err) => {
            // Aqui, um erro provavelmente é interno, pois não há validação complexa
            let response = json!({"error": "Falha ao deletar horários", "message": err});
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(response)))
        }
    }
}
