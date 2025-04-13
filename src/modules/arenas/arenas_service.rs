use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Error};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

use crate::modules::arenas::arenas_model::{Estabelecimento, LocalInput, Quadra};

use super::{
    arenas_model::{Horario, Local, RegisterQuadraInput},
    arenas_repository::{create_estabelecimento, create_quadra, list_free_times},
};

pub async fn register_estabelecimento(
    client: &mut tokio_postgres::Client,
    estabelecimento: Estabelecimento,
) -> Result<Estabelecimento, String> {
    create_estabelecimento(client, estabelecimento).await
}

pub async fn register_quadra(
    client: &mut Client,
    input: RegisterQuadraInput,
) -> Result<Quadra, Error> {
    create_quadra(client, &input.quadra, input.horarios).await
}

pub async fn get_available_hours(
    client: &mut Client,
    quadra_id: i32,
    date: NaiveDate,
) -> Result<Vec<(NaiveTime, NaiveTime)>, String> {
    list_free_times(client, quadra_id, date)
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_estabelecimento_data(client: &Client, estabelecimento_id: i32) -> Result<String, String> {
    let query = "SELECT nome FROM estabelecimentos WHERE id = $1";
    let row = client
        .query_one(query, &[&estabelecimento_id])
        .await
        .map_err(|e| format!("Error fetching estabelecimento data: {}", e))?;

    Ok(row.get("nome"))
}

pub async fn get_local_data(client: &Client, local_id: i32) -> Result<Local, String> {
    let query = "SELECT * FROM locais WHERE id = $1";
    let row = client
        .query_one(query, &[&local_id])
        .await
        .map_err(|e| format!("Error fetching local data: {}", e))?;

    let local = Local {
        id: Some(row.get("id")),
        nome: row.get("nome"),
        rua: row.get("rua"),
        numero: row.get("numero"),
        complemento: row.get("complemento"),
        bairro: row.get("bairro"),
        cidade: row.get("cidade"),
        estado: row.get("estado"),
        codigo_postal: row.get("codigo_postal"),
        country: row.get("country"),
        latitude: row.get::<_, Decimal>("latitude").to_f64().unwrap_or_default(),
        longitude: row.get::<_, Decimal>("longitude").to_f64().unwrap_or_default(),
        estabelecimento_id: Some(row.get("estabelecimento_id")),
    };

    Ok(local)
}

pub async fn get_quadra_data(client: &Client, quadra_id: i32) -> Result<Quadra, String> {
    let query = "SELECT * FROM quadras WHERE id = $1";
    let row = client
        .query_one(query, &[&quadra_id])
        .await
        .map_err(|e| format!("Error fetching quadra data: {}", e))?;

    let quadra = Quadra {
        id: Some(row.get("id")),
        nome: row.get("nome"),
        photo_url: row.get("photo_url"),
        local_id: row.get("local_id"),
    };

    Ok(quadra)
}
