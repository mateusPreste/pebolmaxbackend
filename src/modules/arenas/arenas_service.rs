use chrono::{ NaiveDate, NaiveTime };
use tokio_postgres::{ Client, Error };

use crate::modules::arenas::arenas_model::Estabelecimento;

use super::{
    arenas_model::{ Local, Quadra, RegisterQuadraInput },
    arenas_repository::{
        create_estabelecimento,
        create_quadra,
        delete_estabelecimento_by_id,
        find_all_estabelecimentos,
        find_estabelecimento_by_id,
        find_locais_by_estabelecimento_id,
        find_local_by_id,
        list_free_times,
        update_estabelecimento,
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

pub async fn get_locais_by_estabelecimento_id_service(
    client: &tokio_postgres::Client,
    estabelecimento_id: i32
) -> Result<Vec<Local>, String> {
    find_locais_by_estabelecimento_id(client, estabelecimento_id).await
}

//retorna um local especifico pelo seu id
pub async fn get_local_by_id_service(
    client: &tokio_postgres::Client,
    local_id: i32
) -> Result<Option<Local>, String> {
    find_local_by_id(client, local_id).await
}
