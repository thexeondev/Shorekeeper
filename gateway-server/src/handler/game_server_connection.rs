use std::sync::OnceLock;

use shorekeeper_network::{config::ServiceEndPoint, ServiceClient, ServiceMessage};

static CLIENT: OnceLock<ServiceClient> = OnceLock::new();

pub fn init(own_service_id: u32, game_server_end_point: &'static ServiceEndPoint) {
    if CLIENT.get().is_some() {
        tracing::error!("game_server_connection: already initialized");
        return;
    }

    let _ = CLIENT.set(ServiceClient::new(own_service_id, game_server_end_point));
}

pub async fn push_message(message: ServiceMessage) {
    CLIENT.get().unwrap().push(message).await
}
