use chrono::{ NaiveDate, NaiveTime };
use tokio_postgres::{ Client, Error };


use super::{
    arenas_model::{ Local, Quadra, RegisterQuadraInput },
    arenas_repository::list_free_times,
};

pub async fn get_available_hours(
    client: &mut Client,
    quadra_id: i32,
    date: NaiveDate
) -> Result<Vec<(NaiveTime, NaiveTime)>, String> {
    list_free_times(client, quadra_id, date).await.map_err(|e| e.to_string())
}



