use tokio_postgres::{ Client, Error };
use crate::modules::arenas::arenas_model::{ Quadra, RegisterQuadraInput };
use super::repository::{ create_quadra, update_quadra };

pub async fn register_quadra(
    client: &mut Client,
    input: RegisterQuadraInput
) -> Result<Quadra, Error> {
    create_quadra(client, &input.quadra, input.horarios).await
}

pub async fn update_quadra_service(
    client: &mut Client,
    quadra_id: i32,
    quadra: Quadra
) -> Result<Quadra, String> {
    update_quadra(client, quadra_id, quadra).await
}
