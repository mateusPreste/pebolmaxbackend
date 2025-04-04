use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Error};

use crate::modules::arenas::arenas_model::Estabelecimento;

use super::{
    arenas_model::{Horario, Quadra, RegisterQuadraInput},
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
