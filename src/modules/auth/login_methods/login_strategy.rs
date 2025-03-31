use axum::async_trait;
use serde::{Deserialize, Serialize};

use crate::modules::auth::auth_service::AuthService;

use super::{
    direct_login_strategy::DirectLoginStrategy, google_login_strategy::GoogleLoginStrategy,
};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct LoginParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginRequest {
    pub login_method: String,
    pub params: LoginParams,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct LoginResponse {
    pub user_id: String,
}

// Strategy trait
#[async_trait]
pub trait LoginStrategy: Send {
    async fn authenticate(&self, params: &LoginParams) -> Result<LoginResponse, String>;
}

// Factory for creating strategies
pub struct LoginStrategyFactory {
    pub(crate) auth_service: AuthService,
}

impl LoginStrategyFactory {
    pub fn new(auth_service: AuthService) -> Self {
        Self { auth_service }
    }

    pub fn create_strategy(&self, method: &str) -> Option<Box<dyn LoginStrategy>> {
        match method {
            "direct" => Some(Box::new(DirectLoginStrategy {
                auth_service: self.auth_service.clone(),
            })),
            "oauth_google" => Some(Box::new(GoogleLoginStrategy {
                auth_service: self.auth_service.clone(),
            })),
            _ => None,
        }
    }
}
