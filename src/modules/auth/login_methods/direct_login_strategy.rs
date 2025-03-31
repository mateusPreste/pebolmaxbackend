use axum::async_trait;

use crate::modules::auth::auth_service::AuthService;

use super::login_strategy::{LoginParams, LoginResponse, LoginStrategy};

// auth_service.rs
pub struct DirectLoginStrategy {
    pub(crate) auth_service: AuthService,
}

#[async_trait]
impl LoginStrategy for DirectLoginStrategy {
    async fn authenticate(&self, params: &LoginParams) -> Result<LoginResponse, String> {
        let user_id = params
            .user_id
            .clone()
            .ok_or("user_id is required for direct login")?;

        //TODO: check if the credentials is ok

        let token = self
            .auth_service
            .generate_token(&user_id)
            .map_err(|e| e.to_string())?;

        Ok(LoginResponse {
            user_id: user_id,
        })
    }
}
