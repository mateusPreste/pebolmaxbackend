use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors::Error, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tokio_postgres::Client;

use super::{
    auth_controller::UserData,
    auth_model::{Credenciais, Usuario},
    auth_repository::{self, find_user_and_credentials_by_oauth},
    login_methods::login_strategy::{LoginParams, LoginResponse, LoginStrategyFactory},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String, // Subject (user id)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}

#[derive(Clone)]
pub struct AuthService {
    jwt_secret: String,
}

impl AuthService {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }

    // there's 3 permissions: user, admin and referee
    pub fn generate_token(&self, user_id: &String) -> Result<String, Error> {
        let now = Utc::now();
        let expires_at = now + Duration::hours(24);

        let claims = TokenClaims {
            sub: user_id.clone(),
            exp: expires_at.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
    }

    pub fn validate_token(&self, token: &str) -> Result<TokenClaims, Error> {
        decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
    }
}

impl AuthService {
    pub async fn login(
        &self,
        login_method: &str,
        params: &LoginParams,
    ) -> Result<LoginResponse, String> {
        let factory = LoginStrategyFactory::new(self.clone());

        let strategy = factory
            .create_strategy(login_method)
            .ok_or_else(|| "Invalid login method".to_string())?;

        strategy.authenticate(params).await
    }

    pub async fn process_login(
        &self,
        db_client: &Client,
        login_method: &str,
        params: &LoginParams,
    ) -> DbUserResult<(LoginResponse, Usuario)> {
        // Authenticate using the appropriate login strategy.
        let login_data = match self.login(login_method, params).await {
            Ok(response) => response,
            Err(e) => {
                // Log the error
                return DbUserResult::Err(e.to_string());
            }
        };

        // Look up the user in the database using the provided user id from login_data.
        let result = get_user_and_cred_by_oauth(db_client, &login_data.user_id).await;

        println!("login_data: {:?}", login_data);
        let usuario = match result {
            DbUserResult::Ok(user_data) => user_data,
            DbUserResult::NotFound => {
                // Here you might decide to create a new user/credentials record if not found.
                return DbUserResult::NotFound;
            }
            DbUserResult::Err(e) => return DbUserResult::Err(e),
        };

        DbUserResult::Ok((login_data, usuario))
    }
}

pub enum DbUserResult<T> {
    Ok(T),
    NotFound,
    Err(String),
}

pub async fn get_user_and_cred_by_oauth(
    client: &tokio_postgres::Client,
    oauth_provider_id: &str,
) -> DbUserResult<Usuario> {
    let result = auth_repository::find_user_and_credentials_by_oauth(client, oauth_provider_id)
        .await
        .map_err(|e| e.to_string());

    match result {
        Ok(Some(query)) => DbUserResult::Ok(query.0),
        Ok(None) => DbUserResult::NotFound,
        Err(e) => DbUserResult::Err(e),
    }
}

pub async fn create_new_user_and_credentials(
    client: &Client,
    payload: &UserData,
) -> Result<(Usuario, Credenciais), String> {
    // Build new user and credentials from payload.
    let user = Usuario {
        id: 0,
        nome: payload.nome.clone(),
        cpf: payload.cpf.clone(),
        email: payload.email.clone(),
        apelido: payload.apelido.clone(),
        foto: Some(payload.foto.clone()),
        reputacao: None,
    };

    let credentials = Credenciais {
        id: 0,
        usuario_id: 0,
        email: payload.email.clone(),
        email_verified: false,
        phone_number: None,
        phone_verified: false,
        oauth_provider: Some(payload.oauth_provider.clone()),
        oauth_provider_id: Some(payload.oauth_provider_id.clone()),
        password_hash: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    auth_repository::create_new_user_and_credentials(
        client,
        &user.nome,
        &user.cpf,
        &user.apelido,
        user.foto.as_deref(),
        user.reputacao.unwrap_or(0), // Provide a default value of 0 if None
        &credentials.email,
        credentials.phone_number.as_deref(),
        credentials.oauth_provider.as_deref(),
        credentials.oauth_provider_id.as_deref(),
        credentials.password_hash.as_deref(),
    )
    .await
}
