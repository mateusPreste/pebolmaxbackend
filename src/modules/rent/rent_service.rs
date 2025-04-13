use crate::modules::rent::rent_model::{Reserva, ReservaInput, StatusReserva, ReservaDetalhesResponse};
use crate::modules::rent::rent_repository::create_reserva;
use crate::modules::rent::rent_repository::update_reserva_status;
use crate::modules::rent::rent_repository::get_reserva_details;
use crate::modules::service::service_service::create_service_for_reserva;
use crate::modules::auth::auth_service::get_user_data;
use crate::modules::arenas::arenas_service::{get_estabelecimento_data, get_local_data, get_quadra_data};
use chrono::NaiveDate;
use tokio_postgres::Client;

pub async fn register_reserva(client: &mut Client, input: ReservaInput) -> Result<Reserva, String> {
    let user_name = get_user_data(client, input.usuario_id).await?;

    let quadra_data = get_quadra_data(client, input.quadra_id).await?;
    let local_data = get_local_data(client, quadra_data.local_id).await?;
    let estabelecimento_id = local_data.estabelecimento_id.ok_or("Estabelecimento não encontrado")?;
    let estabelecimento_name = get_estabelecimento_data(client, estabelecimento_id).await?;

    // Create a service for this reserva
    let service = create_service_for_reserva(
        client,
        &estabelecimento_name,
        &user_name,
        &input.inicio.format("%Y-%m-%d").to_string(),
    ).await?;

    let reserva = Reserva {
        id: None,
        usuario_id: input.usuario_id,
        quadra_id: input.quadra_id,
        inicio: input.inicio,
        fim: input.fim,
        status_id: StatusReserva::Aberto,
        modalidade: Some(input.modalidade.unwrap_or_else(|| "usuarios".to_string())),
        min_pagantes: input.min_pagantes.unwrap_or(0),
        service_id: service.service_id.unwrap_or_default(),
    };
    create_reserva(client, reserva).await
}

pub async fn update_reserva_status_service(
    client: &mut Client,
    reserva_id: i32,
    new_status: StatusReserva,
) -> Result<(), String> {
    let status_str = new_status.to_string();
    update_reserva_status(client, reserva_id, new_status).await
}

pub async fn get_reserva_details_service(
    client: &mut Client,
    reserva_id: i32,
) -> Result<ReservaDetalhesResponse, String> {
    get_reserva_details(client, reserva_id).await
}
