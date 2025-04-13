use tokio_postgres::Client;
use crate::modules::service::service_model::{Service, ServiceType};

pub async fn get_service_type_by_type(client: &Client, service_type: &str) -> Result<ServiceType, String> {
    let row = client
        .query_one(
            "SELECT service_type_id, service_type, service_name, description, created_at 
             FROM services_types 
             WHERE service_type = $1",
            &[&service_type],
        )
        .await
        .map_err(|e| format!("Error fetching service type: {}", e))?;

    Ok(ServiceType {
        service_type_id: row.get("service_type_id"),
        service_type: row.get("service_type"),
        service_name: row.get("service_name"),
        description: row.get("description"),
        created_at: row.get("created_at"),
    })
}

pub async fn create_service(client: &mut Client, service: Service) -> Result<Service, String> {
    let row = client
        .query_one(
            "INSERT INTO services (service_type_id, service_name, description) 
             VALUES ($1, $2, $3) 
             RETURNING service_id, service_type_id, service_name, description, created_at",
            &[
                &service.service_type_id,
                &service.service_name,
                &service.description,
            ],
        )
        .await
        .map_err(|e| format!("Error creating service: {}", e))?;

    Ok(Service {
        service_id: Some(row.get("service_id")),
        service_type_id: row.get("service_type_id"),
        service_name: row.get("service_name"),
        description: row.get("description"),
        created_at: row.get("created_at"),
    })
} 