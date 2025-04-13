use chrono::{ NaiveDate, NaiveTime };
use tokio_postgres::{ Client, Error };

use crate::modules::arenas::arenas_model::Estabelecimento;

use super::{
    arenas_model::{ Quadra, RegisterQuadraInput },
    arenas_repository::{
        create_estabelecimento,
        create_quadra,
        delete_estabelecimento_by_id,
        find_all_estabelecimentos,
        find_estabelecimento_by_id,
        list_free_times, update_estabelecimento,
    },
};

pub async fn register_estabelecimento(
    client: &mut tokio_postgres::Client,
    estabelecimento: Estabelecimento
) -> Result<Estabelecimento, String> {
    create_estabelecimento(client, estabelecimento).await
}

pub async fn get_estabelecimento(
    client: &mut Client,
    id: i32
) -> Result<Option<Estabelecimento>, String> {
    find_estabelecimento_by_id(client, id).await
}

pub async fn get_all_estabelecimentos(client: &mut Client) -> Result<Vec<Estabelecimento>, String> {
    find_all_estabelecimentos(client).await
}

pub async fn delete_estabelecimento(client: &mut Client, id: i32) -> Result<(), String> {
    delete_estabelecimento_by_id(client, id).await
}

pub async fn register_quadra(
    client: &mut Client,
    input: RegisterQuadraInput
) -> Result<Quadra, Error> {
    create_quadra(client, &input.quadra, input.horarios).await
}

pub async fn get_available_hours(
    client: &mut Client,
    quadra_id: i32,
    date: NaiveDate
) -> Result<Vec<(NaiveTime, NaiveTime)>, String> {
    list_free_times(client, quadra_id, date).await.map_err(|e| e.to_string())
}

pub async fn update_estabelecimento_service(
    client: &tokio_postgres::Client,
    id: i32,
    estabelecimento: Estabelecimento
) -> Result<Estabelecimento, String> {
    update_estabelecimento(client, id, estabelecimento).await
}
