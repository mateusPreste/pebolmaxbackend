use chrono::Utc;
use tokio_postgres::Client;
use serde_json;

use super::payment_model::{PaymentRecord, Transaction, TransactionStatus};

pub async fn create_transaction(
    client: &mut tokio_postgres::Transaction<'_>,
    service_id: i32,
    amount: f64,
    conta_id: i32,
) -> Result<Transaction, String> {
    let now = Utc::now().naive_utc();
    let stmt = "INSERT INTO transactions (service_id, amount, status, created_at, updated_at) \
                VALUES ($1, $2, $3, $4, $5) RETURNING transaction_id";
    
    let row = client
        .query_one(
            stmt,
            &[&service_id, &amount, &TransactionStatus::Pendente.as_str(), &now, &now],
        )
        .await
        .map_err(|e| e.to_string())?;
    
    let id: i32 = row.get("transaction_id");
    Ok(Transaction {
        id: Some(id),
        service_id,
        amount,
        conta_id,
        status: TransactionStatus::Pendente,
        created_at: now,
        updated_at: now,
    })
}

pub async fn create_payment_record(
    client: &mut tokio_postgres::Transaction<'_>,
    transaction_id: i32,
) -> Result<PaymentRecord, String> {
    let stmt = "INSERT INTO payment_records (transaction_id) \
                VALUES ($1) RETURNING payment_id";
    
    let row = client
        .query_one(
            stmt,
            &[&transaction_id],
        )
        .await
        .map_err(|e| e.to_string())?;
    
    let id: i32 = row.get("payment_id");
    Ok(PaymentRecord {
        id: Some(id),
        transaction_id,
        finalized_at: None,
    })
}

pub async fn update_transaction_status(
    client: &mut Client,
    transaction_id: i32,
    new_status: &TransactionStatus,
) -> Result<(), String> {
    let now = Utc::now().naive_utc();
    let stmt = "UPDATE transactions SET status = $1, updated_at = $2 WHERE transaction_id = $3";
    client
        .execute(stmt, &[&new_status.as_str(), &now, &transaction_id])
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn finalize_payment(
    client: &mut Client,
    payment_id: i32,
) -> Result<(), String> {
    let now = Utc::now().naive_utc();
    let stmt = "UPDATE payment_records SET finalized_at = $1 WHERE payment_id = $2";
    client
        .execute(stmt, &[&now, &payment_id])
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
} 