use axum::async_trait;
use serde::Deserialize;

use crate::modules::auth::auth_service::{get_user_and_cred_by_oauth, AuthService};
use reqwest::Client;

use super::login_strategy::{LoginParams, LoginResponse, LoginStrategy};

pub struct GoogleLoginStrategy {
    pub(crate) auth_service: AuthService,
}

#[async_trait]
impl LoginStrategy for GoogleLoginStrategy {
    async fn authenticate(&self, params: &LoginParams) -> Result<LoginResponse, String> {
        // check if the oauth token is valid
        let user_data = check_oauth_acess_token(params)
            .await
            .map_err(|e| format!("Google token validation failed: {}", e))?;

        Ok(LoginResponse { user_id: user_data })
    }
}

#[derive(Debug, Deserialize)]
struct GoogleTokenInfo {
    issued_to: String, // client ID
    audience: String,  // client ID
    user_id: String,
    scope: String,
    expires_in: i64,
    email: String,
    verified_email: bool,
    access_type: String,
    // add other fields as needed
}

async fn check_oauth_acess_token(params: &LoginParams) -> Result<String, String> {
    //extract id_token and user_id from params
    let id_token = params
        .id_token
        .as_ref()
        .ok_or("id_token is required for Google login")?;

    let user_id = params
        .user_id
        .as_ref()
        .ok_or("user_id is required for Google login")?;

    // Build the URL to verify the id_token with Google.
    let url = format!(
        "https://www.googleapis.com/oauth2/v1/tokeninfo?access_token={}",
        id_token
    );
    let client = Client::new();
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err("Google token validation failed".to_string());
    }

    let token_info: GoogleTokenInfo = response.json().await.map_err(|e| e.to_string())?;

    if token_info.user_id != *user_id {
        return Err("Token user_id does not match the provided user_id".to_string());
    }

    // Check expected audience if needed:
    let expected_audience = std::env::var("GOOGLE_CLIENT_ID")
        .map_err(|_| "GOOGLE_CLIENT_ID not found in environment variables".to_string())?;
    if token_info.audience != expected_audience {
        return Err("Token audience does not match".into());
    }

    // Return the user ID on success
    Ok(token_info.user_id)
}

// load database
