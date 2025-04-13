use crate::modules::rent::rent_model::{Reserva, StatusReserva};
use crate::modules::rent::rent_model::{ReservaDetalhesResponse, UserDTO, QuadraDTO, LocalDTO, EstabelecimentoDTO};
use tokio_postgres::Client;
use chrono::NaiveDateTime;

pub async fn create_reserva(client: &mut Client, mut reserva: Reserva) -> Result<Reserva, String> {
    let stmt =
        "INSERT INTO reservas (usuario_id, quadra_id, service_id, inicio, fim, status_id, modalidade, min_pagantes) \
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id";

    let status_str = reserva.status_id.to_string();

    let row = client
        .query_one(
            stmt,
            &[
                &reserva.usuario_id,
                &reserva.quadra_id,
                &reserva.service_id,
                &reserva.inicio,
                &reserva.fim,
                &status_str,
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
    new_status: StatusReserva,
) -> Result<(), String> {
    let stmt = "UPDATE reservas SET status_id = $1 WHERE id = $2";
    let status_str = new_status.to_string();
    client
        .execute(stmt, &[&status_str, &reserva_id])
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

async fn get_reserva_by_id(client: &mut Client, reserva_id: i32) -> Result<(i32, i32, i32, NaiveDateTime, NaiveDateTime, StatusReserva, Option<String>, i32), String> {
    let reserva_query = "SELECT * FROM reservas WHERE id = $1";
    let reserva_row = client
        .query_one(reserva_query, &[&reserva_id])
        .await
        .map_err(|e| format!("Error fetching reservation: {}", e))?;

    let id: i32 = reserva_row.get("id");
    let usuario_id: i32 = reserva_row.get("usuario_id");
    let quadra_id: i32 = reserva_row.get("quadra_id");
    let inicio: NaiveDateTime = reserva_row.get("inicio");
    let fim: NaiveDateTime = reserva_row.get("fim");
    let status_id_str: String = reserva_row.get("status_id");
    let status_id = match status_id_str.as_str() {
        "Aberto" => StatusReserva::Aberto,
        "Reservado" => StatusReserva::Reservado,
        "Em Disputa" => StatusReserva::EmDisputa,
        "Cancelado" => StatusReserva::Cancelado,
        "Perdido" => StatusReserva::Perdido,
        _ => return Err(format!("Invalid status_id: {}", status_id_str)),
    };
    let modalidade: Option<String> = reserva_row.get("modalidade");
    let min_pagantes: i32 = reserva_row.get("min_pagantes");

    Ok((id, usuario_id, quadra_id, inicio, fim, status_id, modalidade, min_pagantes))
}

async fn get_quadra_by_id(client: &mut Client, quadra_id: i32) -> Result<QuadraDTO, String> {
    let quadra_query = "SELECT * FROM quadras WHERE id = $1";
    let quadra_row = client
        .query_one(quadra_query, &[&quadra_id])
        .await
        .map_err(|e| format!("Error fetching quadra: {}", e))?;

    let quadra = QuadraDTO {
        id: quadra_row.get("id"),
        nome: quadra_row.get("nome"),
        photo_url: quadra_row.get("photo_url"),
        local_id: quadra_row.get("local_id"),
    };

    Ok(quadra)
}

async fn get_local_by_id(client: &mut Client, local_id: i32) -> Result<LocalDTO, String> {
    let local_query = "SELECT * FROM locais WHERE id = $1";
    let local_row = client
        .query_one(local_query, &[&local_id])
        .await
        .map_err(|e| format!("Error fetching local: {}", e))?;

    let local = LocalDTO {
        id: local_row.get("id"),
        nome: local_row.get("nome"),
        rua: local_row.get("rua"),
        numero: local_row.get("numero"),
        complemento: local_row.get("complemento"),
        bairro: local_row.get("bairro"),
        cidade: local_row.get("cidade"),
        estado: local_row.get("estado"),
        codigo_postal: local_row.get("codigo_postal"),
        country: local_row.get("country"),
        latitude: local_row.get("latitude"),
        longitude: local_row.get("longitude"),
        estabelecimento_id: local_row.get("estabelecimento_id"),
    };

    Ok(local)
}

async fn get_estabelecimento_by_id(client: &mut Client, estabelecimento_id: i32) -> Result<EstabelecimentoDTO, String> {
    let estabelecimento_query = "SELECT * FROM estabelecimentos WHERE id = $1";
    let estabelecimento_row = client
        .query_one(estabelecimento_query, &[&estabelecimento_id])
        .await
        .map_err(|e| format!("Error fetching estabelecimento: {}", e))?;

    let estabelecimento = EstabelecimentoDTO {
        id: estabelecimento_row.get("id"),
        nome: estabelecimento_row.get("nome"),
        tax_id: estabelecimento_row.get("tax_id"),
        tipo: estabelecimento_row.get("tipo"),
        pais: estabelecimento_row.get("pais"),
    };

    Ok(estabelecimento)
}

async fn get_user_by_id(client: &mut Client, usuario_id: i32) -> Result<UserDTO, String> {
    let organizador_query = "SELECT id, nome, apelido, foto, reputacao FROM usuarios WHERE id = $1";
    let organizador_row = client
        .query_one(organizador_query, &[&usuario_id])
        .await
        .map_err(|e| format!("Error fetching user: {}", e))?;

    let user = UserDTO {
        id: organizador_row.get("id"),
        nome: organizador_row.get("nome"),
        apelido: organizador_row.get("apelido"),
        foto: organizador_row.get("foto"),
        reputacao: organizador_row.get("reputacao"),
    };

    Ok(user)
}

async fn get_participantes_by_reserva_id(client: &mut Client, reserva_id: i32) -> Result<Vec<UserDTO>, String> {
    let participantes_query = "
        SELECT u.id, u.nome, u.apelido, u.foto, u.reputacao 
        FROM reserva_usuarios ru
        JOIN usuarios u ON ru.usuario_id = u.id
        WHERE ru.reserva_id = $1";

    let participantes_rows = client
        .query(participantes_query, &[&reserva_id])
        .await
        .map_err(|e| format!("Error fetching participants: {}", e))?;

    let participantes = participantes_rows
        .iter()
        .map(|row| UserDTO {
            id: row.get("id"),
            nome: row.get("nome"),
            apelido: row.get("apelido"),
            foto: row.get("foto"),
            reputacao: row.get("reputacao"),
        })
        .collect();

    Ok(participantes)
}

pub async fn get_reserva_details(
    client: &mut Client,
    reserva_id: i32,
) -> Result<ReservaDetalhesResponse, String> {
    // 1. Get the reservation info
    let (id, usuario_id, quadra_id, inicio, fim, status_id, modalidade, min_pagantes) = 
        match get_reserva_by_id(client, reserva_id).await {
            Ok(reserva) => reserva,
            Err(e) => return Err("Reserva não encontrada".to_string()),
        };

    // 2. Get the court (quadra) info
    let quadra = get_quadra_by_id(client, quadra_id).await?;

    // 3. Get local info
    let local = get_local_by_id(client, quadra.local_id).await?;

    // 4. Get estabelecimento info
    let estabelecimento = get_estabelecimento_by_id(client, local.estabelecimento_id).await?;

    // 5. Get organizer info (without CPF)
    let organizador = get_user_by_id(client, usuario_id).await?;

    // 6. Get all participants (without CPF)
    let participantes = get_participantes_by_reserva_id(client, reserva_id).await?;

    Ok(ReservaDetalhesResponse {
        id,
        inicio,
        fim,
        status_id,
        modalidade,
        min_pagantes,
        quadra,
        local,
        estabelecimento,
        organizador,
        participantes,
    })
}
