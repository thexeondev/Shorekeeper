use std::sync::OnceLock;

use shorekeeper_network::{config::ServiceEndPoint, ServiceClient, ServiceMessage};

static CLIENT: OnceLock<ServiceClient> = OnceLock::new();

pub fn init(own_service_id: u32, gateway_end_point: &'static ServiceEndPoint) {
    if CLIENT.get().is_some() {
        tracing::error!("gateway_connection: already initialized");
        return;
    }

    let _ = CLIENT.set(ServiceClient::new(own_service_id, gateway_end_point));
}

pub async fn push_message(message: ServiceMessage) {
    CLIENT.get().unwrap().push(message).await
}

pub fn push_message_sync(message: ServiceMessage) {
    CLIENT.get().unwrap().push_sync(message)
}
