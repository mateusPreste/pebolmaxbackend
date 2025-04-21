use crate::modules::arenas::arenas_model::Estabelecimento;
use crate::modules::arenas::estabelecimento::repository::{
    create_estabelecimento,
    delete_estabelecimento_by_id,
    find_all_estabelecimentos,
    find_estabelecimento_by_id,
};
use tokio_postgres::Client;

use super::repository::update_estabelecimento;

pub async fn register_estabelecimento(
    client: &mut Client,
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

pub async fn get_all_estabelecimentos_service(client: &mut Client) -> Result<Vec<Estabelecimento>, String> {
    find_all_estabelecimentos(client).await
}

pub async fn update_estabelecimento_service(
    client: &tokio_postgres::Client,
    id: i32,
    estabelecimento: Estabelecimento
) -> Result<Estabelecimento, String> {
    update_estabelecimento(client, id, estabelecimento).await
}

pub async fn delete_estabelecimento(client: &mut Client, id: i32) -> Result<(), String> {
    delete_estabelecimento_by_id(client, id).await
}
