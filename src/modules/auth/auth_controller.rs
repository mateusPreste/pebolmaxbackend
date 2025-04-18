use std::sync::Arc;

use axum::debug_handler;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Mutex;

use crate::modules::auth::auth_model::{Credenciais, Usuario};
use crate::modules::auth::auth_service::DbUserResult;
use crate::AppState;

use super::auth_service::{create_new_user_and_credentials, get_user_and_cred_by_oauth};
use super::login_methods::login_strategy::{
    LoginParams, LoginRequest, LoginResponse, LoginStrategyFactory,
};

#[debug_handler]
pub async fn login_controller(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut_state = state.lock().await;
    let auth = &mut_state.auth;
    let db = &mut_state.db;

    match auth
        .process_login(&db, &payload.login_method, &payload.params)
        .await
    {
        DbUserResult::Ok((login_data, usuario)) => {
            let token = match auth
                .generate_token(&login_data.user_id)
                .map_err(|e| e.to_string())
            {
                Ok(token) => token,
                Err(e) => {
                    return Ok((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({
                            "error": "Internal server error",
                            "message": e,
                        })),
                    )
                        .into_response())
                }
            };

            Ok((
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "user": usuario,
                    "token": token,
                    "user_id": login_data.user_id,
                })),
            )
                .into_response())
        }

        DbUserResult::NotFound => Ok((
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "User not found",
            })),
        )
            .into_response()),
        DbUserResult::Err(e) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Internal server error",
                "message": e,
            })),
        )
            .into_response()),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct UserData {
    pub nome: String,
    pub cpf: String,
    pub email: String,
    pub apelido: String,
    pub foto: String,
    pub oauth_provider: String,
    pub oauth_provider_id: String,
    pub login_method: String,
    pub access_token: String,
    pub id_token: String,
    pub user_id: String,
}
#[debug_handler]
pub async fn register_user_controller(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<UserData>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut_state = state.lock().await;
    let auth = &mut_state.auth;
    let db = &mut_state.db;

    let login_result = auth
        .process_login(
            db,
            &payload.login_method,
            &LoginParams {
                access_token: Some(payload.access_token.clone()),
                user_id: Some(payload.user_id.clone()),
                id_token: Some(payload.id_token.clone()),
            },
        )
        .await;

    match login_result {
        // When login is successful, return token and found user.
        DbUserResult::Ok((login_data, usuario)) => {
            let token = match auth
                .generate_token(&login_data.user_id)
                .map_err(|e| e.to_string())
            {
                Ok(token) => token,
                Err(e) => {
                    let response = json!({
                        "error": "Internal server error",
                        "message": e,
                    });
                    return Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response());
                }
            };

            let response = json!({
                "success": true,
                "user": usuario,
                "token": token,
                "user_id": login_data.user_id,
            });
            Ok((StatusCode::OK, Json(response)).into_response())
        }
        // When no user is found, create a new user and credentials record.
        DbUserResult::NotFound => match create_new_user_and_credentials(&db, &payload).await {
            Ok((usuario, credenciais)) => {
                let token = match auth
                    .generate_token(&payload.user_id)
                    .map_err(|e| e.to_string())
                {
                    Ok(token) => token,
                    Err(e) => {
                        let response = json!({
                            "error": "Internal server error",
                            "message": e,
                        });
                        return Ok(
                            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
                        );
                    }
                };

                let response = json!({
                    "success": true,
                    "user": usuario,
                    "token": token,
                    "user_id": payload.user_id,
                });
                Ok((StatusCode::OK, Json(response)).into_response())
            }
            Err(e) => {
                let response = json!({
                    "error": "Internal server error",
                    "message": e,
                });
                Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response())
            }
        },
        // Catch-all error.
        DbUserResult::Err(err_msg) => {
            let response = json!({
                "error": "Internal server error",
                "message": err_msg,
            });
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response())
        }
    }
}
