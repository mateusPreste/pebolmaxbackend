use crate::modules::rent::rent_model::Reserva;
use tokio_postgres::Client;

pub async fn create_reserva(client: &mut Client, mut reserva: Reserva) -> Result<Reserva, String> {
    let stmt =
        "INSERT INTO reservas (usuario_id, quadra_id, inicio, fim, status_id, modalidade, min_pagantes) \
                VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id";
    let row = client
        .query_one(
            stmt,
            &[
                &reserva.usuario_id,
                &reserva.quadra_id,
                &reserva.inicio,
                &reserva.fim,
                &reserva.status_id,
                &reserva.modalidade,
                &reserva.min_pagantes,
            ],
        )
        .await
        .map_err(|e| e.to_string())?;
    let id: i32 = row.get("id");
    reserva.id = Some(id);
    Ok(reserva)
}

pub async fn update_reserva_status(
    client: &mut Client,
    reserva_id: i32,
    new_status: &str,
) -> Result<(), String> {
    let stmt = "UPDATE reservas SET status_id = $1 WHERE id = $2";
    client
        .execute(stmt, &[&new_status, &reserva_id])
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
