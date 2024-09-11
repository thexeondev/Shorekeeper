use std::sync::OnceLock;

use common::time_util;
use shorekeeper_network::{config::ServiceEndPoint, ServiceListener, ServiceMessage};
use shorekeeper_protocol::{
    message::Message, CreateCharacterResponse, CreatePlayerDataResponse, EnterGameResponse,
    ErrorCode, ForwardClientMessagePush, MessageID, Protobuf, StartPlayerSessionResponse,
};

use crate::session::SessionManager;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to bind ServiceListener")]
    BindFailed,
}

pub async fn start_task(
    listen_end_point: &ServiceEndPoint,
    session_mgr: &'static SessionManager,
) -> Result<(), Error> {
    static IS_STARTED: OnceLock<()> = OnceLock::new();
    if IS_STARTED.set(()).is_err() {
        tracing::error!("service_message_handler: task already started");
        return Ok(());
    }

    let listener = ServiceListener::bind(listen_end_point)
        .await
        .map_err(|_| Error::BindFailed)?;

    tokio::spawn(async move { handler_task_fn(listener, session_mgr).await });
    Ok(())
}

async fn handler_task_fn(mut listener: ServiceListener, session_mgr: &'static SessionManager) {
    loop {
        let Some(message) = listener.receive().await else {
            continue;
        };

        tracing::debug!(
            "received message from service: {}, rpc_id: {} message_id: {}",
            message.src_service_id,
            message.rpc_id,
            message.message_id
        );

        match message.message_id {
            CreatePlayerDataResponse::MESSAGE_ID => {
                on_create_player_data_response(message, session_mgr).await
            }
            StartPlayerSessionResponse::MESSAGE_ID => {
                on_start_player_session_response(message, session_mgr).await
            }
            ForwardClientMessagePush::MESSAGE_ID => {
                on_forward_client_message_push(message, session_mgr).await
            }
            unhandled => tracing::warn!(
                "unhandled service message id: {unhandled}, from service_id: {}",
                message.src_service_id
            ),
        }
    }
}

async fn on_forward_client_message_push(
    message: ServiceMessage,
    session_mgr: &'static SessionManager,
) {
    let Ok(push) = ForwardClientMessagePush::decode(message.data.as_ref()) else {
        tracing::error!(
            "failed to decode ForwardClientMessagePush, data: {}",
            hex::encode(&message.data)
        );
        return;
    };

    let Ok(message) = Message::decode(&push.data) else {
        tracing::error!(
            "ForwardClientMessage: failed to decode underlying message data, session_id: {}",
            push.gateway_session_id
        );
        return;
    };

    let Some(mut session) = session_mgr.get_mut(push.gateway_session_id) else {
        tracing::error!(
            "ForwardClientMessage: session not found, id: {}",
            push.gateway_session_id
        );
        return;
    };

    tracing::debug!(
        "forward message from game server to client, message_id: {}, session_id: {}",
        message.get_message_id(),
        push.gateway_session_id
    );

    let _ = session.send_message(message).await.inspect_err(|err| {
        tracing::error!("ForwardClientMessage: failed to send message, error: {err}")
    });
}

async fn on_start_player_session_response(
    message: ServiceMessage,
    session_mgr: &'static SessionManager,
) {
    let Ok(response) = StartPlayerSessionResponse::decode(message.data.as_ref()) else {
        tracing::error!(
            "failed to decode StartPlayerSessionResponse, data: {}",
            hex::encode(&message.data)
        );
        return;
    };

    let Some(mut session) = session_mgr.get_mut(response.gateway_session_id) else {
        tracing::error!(
            "StartPlayerSession: session not found, id: {}",
            response.gateway_session_id
        );
        return;
    };

    if response.code != 0 {
        tracing::debug!(
            "StartPlayerSession failed, session_id: {}, player_id: {:?}, user_id: {:?}, error code: {}",
            response.gateway_session_id,
            &session.player_id,
            &session.user_id,
            response.code
        );

        let _ = session
            .send_response(
                EnterGameResponse {
                    error_code: response.code,
                    ..Default::default()
                },
                message.rpc_id,
            )
            .await;
    } else {
        tracing::debug!(
            "StartPlayerSession success, game server should forward subsequent messages now. player_id: {}",
            session.player_id.unwrap_or_default()
        );
    }
}

async fn on_create_player_data_response(
    message: ServiceMessage,
    session_mgr: &'static SessionManager,
) {
    let Ok(response) = CreatePlayerDataResponse::decode(message.data.as_ref()) else {
        tracing::error!(
            "failed to decode CreatePlayerDataResponse, data: {}",
            hex::encode(&message.data)
        );
        return;
    };

    let Some(mut session) = session_mgr.get_mut(response.session_id) else {
        tracing::debug!("session with id {} not found", response.session_id);
        return;
    };

    if response.code != 0 {
        tracing::warn!("CreatePlayerData failed, code: {}", response.code);

        let _ = session
            .send_response(
                CreateCharacterResponse {
                    error_code: response.code,
                    ..Default::default()
                },
                message.rpc_id,
            )
            .await;
        return;
    }

    let Some(user_id) = session.user_id.as_ref() else {
        tracing::debug!(
            "session.user_id is None, session_id: {}",
            response.session_id
        );
        return;
    };

    tracing::info!(
        "CreateCharacter success, player_id: {}, user_id: {}",
        response.player_id,
        user_id
    );
    session.player_id = Some(response.player_id);

    let _ = session
        .send_response(
            CreateCharacterResponse {
                error_code: ErrorCode::Success.into(),
                player_id: response.player_id,
                name: response.name,
                create_time: time_util::unix_timestamp() as i32,
            },
            message.rpc_id,
        )
        .await;
}
