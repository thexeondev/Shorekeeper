use std::sync::OnceLock;

use shorekeeper_network::ServiceMessage;
use shorekeeper_protocol::{
    message::Message, proto_config, ForwardClientMessagePush, MessageID, MessageRoute, Protobuf,
};
use tokio::sync::mpsc;

use crate::session::{Session, SessionManager};

use super::game_server_connection;

static MSG_TX: OnceLock<mpsc::Sender<(u32, Message)>> = OnceLock::new();

pub fn start_task(session_mgr: &'static SessionManager) {
    let (tx, rx) = mpsc::channel(32);
    if MSG_TX.set(tx).is_err() {
        tracing::error!("client_message_handler task is already running");
        return;
    }

    tokio::spawn(async move { handler_task_fn(rx, session_mgr).await });
}

pub async fn push_message(session_id: u32, msg: Message) {
    let _ = MSG_TX.get().unwrap().send((session_id, msg)).await;
}

async fn handler_task_fn(
    mut rx: mpsc::Receiver<(u32, Message)>,
    session_mgr: &'static SessionManager,
) {
    loop {
        let Some((session_id, message)) = rx.recv().await else {
            continue;
        };

        let Some(mut session) = session_mgr.get_mut(session_id) else {
            continue;
        };

        if let Err(err) = handle_message(session.value_mut(), message).await {
            tracing::error!(
                "handle_message failed, session_id: {}, err: {err}",
                session_id
            );
        }
    }
}

async fn handle_message(
    session: &mut Session,
    message: Message,
) -> Result<(), crate::session::SessionError> {
    match proto_config::get_message_flags(message.get_message_id()).into() {
        MessageRoute::Gateway => handle_message_impl(session, message).await,
        MessageRoute::GameServer => forward_to_game_server(session, message).await,
        route => {
            tracing::warn!(
                "received message with wrong route, id: {}, route: {route:?}",
                message.get_message_id()
            );
            Ok(())
        }
    }
}

async fn forward_to_game_server(
    session: &Session,
    message: Message,
) -> Result<(), crate::session::SessionError> {
    let mut data = vec![0u8; message.get_encoding_length()];
    message.encode(&mut data)?;

    game_server_connection::push_message(ServiceMessage {
        src_service_id: 0,
        rpc_id: message.get_rpc_id(),
        message_id: ForwardClientMessagePush::MESSAGE_ID,
        data: ForwardClientMessagePush {
            gateway_session_id: session.get_conv_id(),
            data,
        }
        .encode_to_vec()
        .into_boxed_slice(),
    })
    .await;

    Ok(())
}

async fn handle_message_impl(
    session: &mut Session,
    message: Message,
) -> Result<(), crate::session::SessionError> {
    if message.is_request() {
        super::client_request_handler::handle_request(session, message).await
    } else if message.is_push() {
        super::client_push_handler::handle_push(session, message).await
    } else {
        tracing::warn!(
            "handle_message: wrong message type: {}, message_id: {}, session_id: {}",
            message.get_message_type(),
            message.get_message_id(),
            session.get_conv_id()
        );
        Ok(())
    }
}
