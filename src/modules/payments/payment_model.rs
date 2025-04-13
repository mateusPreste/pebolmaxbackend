use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::InputValidation;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransactionStatus {
    Pendente,
    Rejeitado,
    Aceito,
    Cancelado,
    Expirado,
}

impl PartialEq for TransactionStatus {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl TransactionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionStatus::Pendente => "pendente",
            TransactionStatus::Rejeitado => "rejeitado",
            TransactionStatus::Aceito => "aceito",
            TransactionStatus::Cancelado => "cancelado",
            TransactionStatus::Expirado => "expirado",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: Option<i32>,
    pub service_id: i32,
    pub amount: f64,
    pub conta_id: i32,
    pub status: TransactionStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentRecord {
    pub id: Option<i32>,
    pub transaction_id: i32,
    pub finalized_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTransactionInput {
    pub service_id: i32,
    pub amount: f64,
    pub conta_id: i32,
}

impl InputValidation for CreateTransactionInput {
    fn validate(&mut self) -> Result<(), String> {
        if self.amount <= 0.0 {
            return Err("Amount must be greater than 0".into());
        }
        
        
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateTransactionStatusInput {
    pub transaction_id: i32,
    pub new_status: TransactionStatus,
}

impl InputValidation for UpdateTransactionStatusInput {
    fn validate(&mut self) -> Result<(), String> {
        Ok(())
    }
} 