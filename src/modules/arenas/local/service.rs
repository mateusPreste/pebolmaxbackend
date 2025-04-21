use crate::modules::arenas::arenas_model::Local;
use super::repository::{
    delete_local_by_id, find_local_by_id, find_locais_by_estabelecimento_id, update_local,
};
use tokio_postgres::Client;

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

pub async fn update_local_service(
    client: &mut Client,
    local_id: i32,
    local: Local
) -> Result<Local, String> {
    update_local(client, local_id, local).await
}

pub async fn delete_local_service(client: &mut Client, id: i32) -> Result<(), String> {
    delete_local_by_id(client, id).await
}