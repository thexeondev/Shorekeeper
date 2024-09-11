use std::sync::{Arc, OnceLock};

use common::time_util;
use shorekeeper_database::{models, query, query_as, PgPool};
use shorekeeper_network::{config::ServiceEndPoint, ServiceListener, ServiceMessage};
use shorekeeper_protocol::{
    message::Message, CreatePlayerDataRequest, CreatePlayerDataResponse, ErrorCode,
    ForwardClientMessagePush, MessageID, PlayerSaveData, Protobuf, ProtocolUnit,
    StartPlayerSessionRequest, StartPlayerSessionResponse, StopPlayerSessionPush,
};

use crate::{
    gateway_connection,
    logic::{self, player::Player, thread_mgr::LogicInput},
    session::{Session, SessionManager},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to bind ServiceListener")]
    BindFailed,
}

pub async fn run(
    listen_end_point: &ServiceEndPoint,
    session_mgr: &'static SessionManager,
    db: Arc<PgPool>,
) -> Result<(), Error> {
    static IS_STARTED: OnceLock<()> = OnceLock::new();
    if IS_STARTED.set(()).is_err() {
        tracing::error!("service_message_handler: task already started");
        return Ok(());
    }

    let listener = ServiceListener::bind(listen_end_point)
        .await
        .map_err(|_| Error::BindFailed)?;

    handler_loop(listener, session_mgr, db).await;
    Ok(())
}

async fn handler_loop(
    mut listener: ServiceListener,
    session_mgr: &'static SessionManager,
    db: Arc<PgPool>,
) {
    loop {
        let Some(message) = listener.receive().await else {
            tracing::warn!("service_message_handler: channel was closed, exitting handler task");
            return;
        };

        tracing::debug!(
            "received message from service: {}, rpc_id: {} message_id: {}",
            message.src_service_id,
            message.rpc_id,
            message.message_id
        );

        match message.message_id {
            CreatePlayerDataRequest::MESSAGE_ID => {
                on_create_player_data_request(message, db.as_ref()).await
            }
            StartPlayerSessionRequest::MESSAGE_ID => {
                on_start_player_session_request(message, session_mgr, db.as_ref()).await
            }
            StopPlayerSessionPush::MESSAGE_ID => {
                on_stop_player_session_push(message, session_mgr).await
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

async fn on_start_player_session_request(
    message: ServiceMessage,
    session_mgr: &'static SessionManager,
    db: &PgPool,
) {
    let Ok(request) = StartPlayerSessionRequest::decode(message.data.as_ref()) else {
        tracing::warn!(
            "failed to decode StartPlayerSessionRequest, data: {}",
            hex::encode(message.data.as_ref())
        );
        return;
    };

    tracing::debug!(
        "StartPlayerSession: gateway_id: {}, session_id: {}, player_id: {}",
        message.src_service_id,
        request.gateway_session_id,
        request.player_id
    );

    let mut response = StartPlayerSessionResponse {
        code: ErrorCode::Success.into(),
        gateway_session_id: request.gateway_session_id,
    };

    let Ok(player_data): Result<models::PlayerDataRow, _> =
        query_as("SELECT * FROM t_player_data WHERE player_id = ($1)")
            .bind(request.player_id)
            .fetch_one(db)
            .await
            .inspect_err(|err| {
                tracing::error!(
                    "failed to fetch player data, player_id: {}, err: {err}",
                    request.player_id
                )
            })
    else {
        response.code = ErrorCode::QueryPlayerDataFailed.into();
        send_to_gateway(response, message.rpc_id).await;
        return;
    };

    let Ok(player_save_data) = PlayerSaveData::decode(player_data.bin_data.as_slice()) else {
        tracing::error!(
            "StartPlayerSession: player data is corrupted, player id: {}",
            request.player_id
        );
        return;
    };

    let logic_thread = logic::thread_mgr::get_least_loaded_thread();
    let session = Arc::new(Session {
        gateway_id: message.src_service_id,
        session_id: request.gateway_session_id,
        player_id: player_data.player_id,
        logic_thread,
    });

    session.logic_thread.input(LogicInput::AddPlayer {
        player_id: player_data.player_id,
        enter_rpc_id: message.rpc_id,
        session: session.clone(),
        player_save_data,
    });

    session_mgr.add(session.clone());

    send_to_gateway(response, message.rpc_id).await;
}

async fn on_stop_player_session_push(
    message: ServiceMessage,
    session_mgr: &'static SessionManager,
) {
    let Ok(push) = StopPlayerSessionPush::decode(message.data.as_ref()) else {
        tracing::warn!(
            "failed to decode StopPlayerSessionPush, data: {}",
            hex::encode(message.data.as_ref())
        );
        return;
    };

    let Some(session) = session_mgr.remove(message.src_service_id, push.gateway_session_id) else {
        tracing::debug!(
            "StopPlayerSessionPush: session with id {} ({}-{}) not found",
            Session::global_id(message.src_service_id, push.gateway_session_id),
            message.src_service_id,
            push.gateway_session_id
        );
        return;
    };

    session.logic_thread.input(LogicInput::RemovePlayer {
        player_id: session.player_id,
    });

    tracing::debug!(
        "StopPlayerSession: player_id: {}, session stopped successfully",
        session.player_id
    );
}

async fn on_forward_client_message_push(
    message: ServiceMessage,
    session_mgr: &'static SessionManager,
) {
    let Ok(push) = ForwardClientMessagePush::decode(message.data.as_ref()) else {
        tracing::warn!(
            "failed to decode ForwardClientMessagePush, data: {}",
            hex::encode(message.data.as_ref())
        );
        return;
    };

    let Some(session) = session_mgr.get(message.src_service_id, push.gateway_session_id) else {
        tracing::debug!(
            "ForwardClientMessagePush: session with id {} ({}-{}) not found",
            Session::global_id(message.src_service_id, push.gateway_session_id),
            message.src_service_id,
            push.gateway_session_id
        );
        return;
    };

    let Ok(message) = Message::decode(&push.data).inspect_err(|err| {
        tracing::warn!("ForwardClientMessagePush: failed to decode underlying message, err: {err}")
    }) else {
        return;
    };

    session.logic_thread.input(LogicInput::ProcessMessage {
        player_id: session.player_id,
        message,
    });
}

async fn on_create_player_data_request(message: ServiceMessage, db: &PgPool) {
    let Ok(request) = CreatePlayerDataRequest::decode(message.data.as_ref()) else {
        tracing::warn!(
            "failed to decode CreatePlayerDataRequest, data: {}",
            hex::encode(message.data.as_ref())
        );
        return;
    };

    let mut response = CreatePlayerDataResponse {
        code: ErrorCode::Success.into(),
        session_id: request.session_id,
        name: request.name.clone(),
        sex: request.sex,
        player_id: 0,
    };

    if !matches!(request.name.len(), (1..=12)) {
        tracing::debug!(
            "character name is too long, name: {}, len: {}",
            &request.name,
            request.name.len()
        );
        response.code = ErrorCode::InvalidCharacterName.into();
        send_to_gateway(response, message.rpc_id).await;
        return;
    }

    if !matches!(request.sex, (0..=1)) {
        response.code = ErrorCode::CreateCharacterFailed.into();
        send_to_gateway(response, message.rpc_id).await;
        return;
    }

    if query("SELECT * from t_user_uid WHERE user_id = ($1)")
        .bind(request.user_id.as_str())
        .fetch_optional(db)
        .await
        .inspect_err(|err| tracing::error!("failed to fetch data from t_user_uid: {err}"))
        .ok()
        .flatten()
        .is_some()
    {
        response.code = ErrorCode::CharacterAlreadyCreated.into();
        send_to_gateway(response, message.rpc_id).await;
        return;
    }

    let user_uid_row: models::UserUidRow = match query_as(
        "INSERT INTO t_user_uid (user_id, sex, create_time_stamp) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(request.user_id.as_str())
    .bind(request.sex)
    .bind(time_util::unix_timestamp() as i64)
    .fetch_one(db)
    .await
    {
        Ok(row) => row,
        Err(err) => {
            tracing::error!("failed to create t_user_uid entry, error: {err}");
            response.code = ErrorCode::InternalError.into();
            send_to_gateway(response, message.rpc_id).await;
            return;
        }
    };

    if let Some(err) =
        query("INSERT INTO t_player_data (player_id, name, bin_data) VALUES ($1, $2, $3)")
            .bind(user_uid_row.player_id)
            .bind(request.name.as_str())
            .bind(
                Player::create_default_save_data(
                    user_uid_row.player_id,
                    request.name.clone(),
                    request.sex,
                )
                .encode_to_vec(),
            )
            .execute(db)
            .await
            .err()
    {
        tracing::error!("failed to create default player data entry, error: {err}");
        response.code = ErrorCode::InternalError.into();
        send_to_gateway(response, message.rpc_id).await;
        return;
    }

    response.player_id = user_uid_row.player_id;
    send_to_gateway(response, message.rpc_id).await;
}

async fn send_to_gateway(content: impl ProtocolUnit, rpc_id: u16) {
    gateway_connection::push_message(ServiceMessage {
        src_service_id: 0,
        rpc_id,
        message_id: content.get_message_id(),
        data: content.encode_to_vec().into_boxed_slice(),
    })
    .await;
}
