use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use tokio::sync::Mutex;

use std::sync::Arc;

use crate::AppState;

pub async fn auth_middleware(
    State(state): State<Arc<Mutex<AppState>>>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = auth_header.trim_start_matches("Bearer ").trim();

    let app_state = state.lock().await;
    match app_state.auth.validate_token(token) {
        Ok(_claims) => Ok(next.run(req).await),
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}
