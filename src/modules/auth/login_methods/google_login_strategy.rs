use std::cmp::Ordering;

use axum::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::modules::auth::auth_service::{get_user_and_cred_by_oauth, AuthService};
use reqwest::Client;

use super::login_strategy::{LoginParams, LoginResponse, LoginStrategy};

pub struct GoogleLoginStrategy {
    pub(crate) auth_service: AuthService,
}

#[async_trait]
impl LoginStrategy for GoogleLoginStrategy {
    async fn authenticate(&self, params: &LoginParams) -> Result<LoginResponse, String> {
        
        if params.id_token.is_some() {
            // check if the id_token is valid
            let user_data = verify_google_id_token(params)
                .await
                .map_err(|e| format!("Google token validation failed: {}", e))?;

            return Ok(LoginResponse { user_id: user_data });
        } else if params.access_token.is_some() {
            // check if the oauth token is valid
            let user_data = check_oauth_acess_token(params)
                .await
                .map_err(|e| format!("Google token validation failed: {}", e))?;

            return Ok(LoginResponse { user_id: user_data });
        }

        Err("Invalid login method".to_string())
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
    //extract access_token and user_id from params
    let access_token = params
        .access_token
        .as_ref()
        .ok_or("access_token is required for Google login")?;

    let user_id = params
        .user_id
        .as_ref()
        .ok_or("user_id is required for Google login")?;

    // Build the URL to verify the access_token with Google.
    let url = format!(
        "https://www.googleapis.com/oauth2/v1/tokeninfo?access_token={}",
        access_token
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



#[derive(Debug, Serialize, Deserialize)]
pub struct Headers {
    // Use #[serde(rename = "...")] for keys with hyphens or other differences
    pub date: String,
    pub pragma: String,
    #[serde(rename = "cache-control")]
    pub cache_control: String,
    pub expires: String,
    #[serde(rename = "content-type")]
    pub content_type: String,
    // This field will capture the *last* "vary" value from the JSON
    pub vary: String,
    pub server: String,
    #[serde(rename = "x-xss-protection")]
    pub x_xss_protection: String,
    #[serde(rename = "x-frame-options")]
    pub x_frame_options: String,
    #[serde(rename = "x-content-type-options")]
    pub x_content_type_options: String,
    #[serde(rename = "alt-svc")]
    pub alt_svc: String,
    #[serde(rename = "accept-ranges")]
    pub accept_ranges: String,
    #[serde(rename = "transfer-encoding")]
    pub transfer_encoding: String,
    // Note: If a header might be missing, use Option<String>
    // e.g., #[serde(rename = "optional-header")] pub optional_header: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponseTypedHeaders {
    pub url: String,
    pub status: u16,
    pub headers: Headers, // Use the dedicated Headers struct
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleIdTokenInfo {
    pub iss: String,
    pub azp: String,
    pub aud: String,
    // Subject ID - often large, best kept as String
    pub sub: String,
    pub email: String,
    // This is the string "true" in the JSON, not a boolean true
    pub email_verified: String,
    // This is the string "null" in the JSON, not a JSON null
    pub nonce: String,
    // Timestamps are strings in JSON but can often be parsed to u64 by serde
    pub nbf: String, // Not Before timestamp
    pub name: String,
    pub picture: String, // URL
    pub given_name: String,
    pub family_name: String,
    pub iat: String, // Issued At timestamp
    pub exp: String, // Expiration timestamp
    pub jti: String, // JWT ID
    // Optional header claims sometimes included in the payload
    pub alg: Option<String>, // Algorithm (e.g., "RS256")
    pub kid: Option<String>, // Key ID
    pub typ: Option<String>, // Type (e.g., "JWT")
}



pub async fn verify_google_id_token(params: &LoginParams) -> Result<String, String> {
    // Build the URL to verify the id_token with Google
    let url = format!(
        "https://oauth2.googleapis.com/tokeninfo?id_token={}",
        params.id_token.as_ref().ok_or("id_token is required for Google login")?
    );

    // Create HTTP client and send request
    let client = Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    // Check if the request was successful
    if !response.status().is_success() {
        let error_msg = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(error_msg);
    }

    // Parse the token info
    let token_info: GoogleIdTokenInfo = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse token info"))?;

    // Verify the token hasn't expired
    let now = Utc::now().timestamp();
    if now >= token_info.exp.parse::<i64>().unwrap() {
        return Err(
            "Token has expired".to_string()
        );
    }

    // Verify the issuer
    if token_info.iss != "https://accounts.google.com" && token_info.iss != "accounts.google.com" {
        return Err(
            "Invalid token issuer".to_string()
        );
    }

    // Verify the audience matches your client ID
    let expected_client_id = std::env::var("GOOGLE_WEB_CLIENT_ID")
        .map_err(|_| "GOOGLE_CLIENT_ID not found in environment".to_string())?;

    if token_info.aud != expected_client_id {
        return Err(
            "Invalid token audience".to_string()
        );
    }

    // If email verification is required, check it
    if token_info.email_verified != "true" {
        return Err(
            "Email not verified".to_string()
        );
    }

    // Return the user ID (sub claim)
    Ok(token_info.sub)
}

fn compare_i64_u64(i: i64, u: u64) -> Ordering {
    if i < 0 {
        // Negative i64 is always less than any u64
        Ordering::Less
    } else {
        // i is non-negative, cast it to u64 for comparison.
        // This cast is safe because we know i >= 0.
        let i_as_u64 = i as u64;
        // Now compare the two u64 values
        i_as_u64.cmp(&u)
        // Equivalent to:
        // if i_as_u64 < u { Ordering::Less }
        // else if i_as_u64 > u { Ordering::Greater }
        // else { Ordering::Equal }
    }
}