use axum::async_trait;
use serde::{Deserialize, Serialize};

use super::auth_service::AuthService;

use chrono::{DateTime, Utc};
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct Usuario {
    pub id: i32,
    pub nome: String,
    pub cpf: String,
    pub email: String,
    pub apelido: String,
    pub foto: Option<String>,
    pub reputacao: Option<i32>,
}

impl Usuario {
    pub fn from_row(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            nome: row.get("nome"),
            cpf: row.get("cpf"),
            email: row.get("email"),
            apelido: row.get("apelido"),
            foto: row.get("foto"),
            reputacao: row.get("reputacao"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credenciais {
    pub id: i32,
    pub usuario_id: i32,
    pub email: String,
    pub email_verified: bool,
    pub phone_number: Option<String>,
    pub phone_verified: bool,
    pub oauth_provider: Option<String>,
    pub oauth_provider_id: Option<String>,
    pub password_hash: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Credenciais {
    pub fn from_row(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            usuario_id: row.get("usuario_id"),
            email: row.get("email"),
            email_verified: row.get("email_verified"),
            phone_number: row.get("phone_number"),
            phone_verified: row.get("phone_verified"),
            oauth_provider: row.get("oauth_provider"),
            oauth_provider_id: row.get("oauth_provider_id"),
            password_hash: row.get("password_hash"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}
