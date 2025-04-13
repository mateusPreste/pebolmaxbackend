use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub service_id: Option<i32>,
    pub service_type_id: i32,
    pub service_name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceInput {
    pub service_type_id: i32,
    pub service_name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceType {
    pub service_type_id: i32,
    pub service_type: String,
    pub service_name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
} 