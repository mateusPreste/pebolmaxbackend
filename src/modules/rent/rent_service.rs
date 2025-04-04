use crate::modules::rent::rent_model::{Reserva, ReservaInput};
use crate::modules::rent::rent_repository::create_reserva;
use crate::modules::rent::rent_repository::update_reserva_status;
use chrono::NaiveDate;
use tokio_postgres::Client;

pub async fn register_reserva(client: &mut Client, input: ReservaInput) -> Result<Reserva, String> {
    let reserva = Reserva {
        id: None,
        usuario_id: input.usuario_id,
        quadra_id: input.quadra_id,
        inicio: input.inicio,
        fim: input.fim,
        status_id: "Aberto".to_string(), // default status
        modalidade: Some(input.modalidade.unwrap_or_else(|| "usuarios".to_string())),
        min_pagantes: input.min_pagantes.unwrap_or(0),
    };
    create_reserva(client, reserva).await
}

pub async fn update_reserva_status_service(
    client: &mut Client,
    reserva_id: i32,
    new_status: String,
) -> Result<(), String> {
    update_reserva_status(client, reserva_id, &new_status).await
}
