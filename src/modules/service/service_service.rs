use tokio_postgres::Client;
use crate::modules::service::service_model::{Service, ServiceInput};
use crate::modules::service::service_repository::{create_service, get_service_type_by_type};

pub async fn create_service_for_reserva(
    client: &mut Client,
    estabelecimento_name: &str,
    user_name: &str,
    date: &str,
) -> Result<Service, String> {
    // Get the service type for 'reserva'
    let service_type = get_service_type_by_type(client, "reserva").await?;
    
    // Create service name by concatenating the information
    let service_name = format!("Reserva - {} - {} - {}", estabelecimento_name, user_name, date);
    
    let service = Service {
        service_id: None,
        service_type_id: service_type.service_type_id,
        service_name,
        description: Some(format!("Reserva de quadra para {} em {}", user_name, date)),
        created_at: service_type.created_at,
    };

    create_service(client, service).await
} 