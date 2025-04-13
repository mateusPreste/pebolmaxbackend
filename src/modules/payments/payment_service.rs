use tokio_postgres::Client;

use super::{
    payment_model::{CreateTransactionInput, PaymentRecord, Transaction, TransactionStatus},
    payment_repository::{
        create_payment_record, create_transaction, finalize_payment, update_transaction_status,
    },
};

pub async fn process_payment(
    client: &mut Client,
    input: CreateTransactionInput,
) -> Result<(Transaction, PaymentRecord), String> {
    // Start a transaction
    let mut transaction = client.transaction().await.map_err(|e| e.to_string())?;

    // Attempt to perform the first operation
    let result1: Result<Transaction, String> = create_transaction(&mut transaction, input.service_id, input.amount, input.conta_id).await;
    
    // Check if the first operation succeeded before proceeding
    let transaction_object = match result1 {
        Ok(get_transaction) => get_transaction,
        Err(e) => return Err(e), // Return the error from the first operation
    };

    let result2: Result<PaymentRecord, String> = create_payment_record(&mut transaction, transaction_object.id.unwrap()).await;

    transaction.commit().await.map_err(|e| e.to_string())?;

    Ok((transaction_object, result2.unwrap()))
}

pub async fn update_payment_status(
    client: &mut Client,
    transaction_id: i32,
    new_status: &TransactionStatus,
) -> Result<(), String> {
    update_transaction_status(client, transaction_id, new_status).await?;
    
    // If status is completed, finalize the payment
    if *new_status == TransactionStatus::Aceito {
        let stmt = "SELECT payment_id FROM payment_records WHERE transaction_id = $1";
        let row = client
            .query_one(stmt, &[&transaction_id])
            .await
            .map_err(|e| e.to_string())?;
        let payment_id: i32 = row.get("payment_id");
        finalize_payment(client, payment_id).await?;
    }
    
    Ok(())
} 