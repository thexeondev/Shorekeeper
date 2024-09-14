use common::time_util;
use shorekeeper_database::{models, query_as};
use shorekeeper_network::ServiceMessage;
use shorekeeper_protocol::{
    message::Message, CreateCharacterRequest, CreatePlayerDataRequest, EnterGameRequest, ErrorCode,
    HeartbeatRequest, HeartbeatResponse, LoginRequest, LoginResponse, MessageID, ProtoKeyRequest,
    ProtoKeyResponse, Protobuf, StartPlayerSessionRequest,
};

use crate::session::Session;

use super::game_server_connection;

macro_rules! requests {
    ($($name:ident;)*) => {
        async fn handle_request_impl(session: &mut crate::session::Session, mut msg: Message) -> Result<(), crate::session::SessionError> {
            use ::shorekeeper_protocol::{MessageID, Protobuf};

            ::paste::paste! {
                match msg.get_message_id() {
                    $(
                        ::shorekeeper_protocol::[<$name Request>]::MESSAGE_ID => {
                            let request = ::shorekeeper_protocol::[<$name Request>]::decode(&*msg.remove_payload())?;
                            let mut response = ::shorekeeper_protocol::[<$name Response>]::default();
                            [<on_ $name:snake _request>](session, request, &mut response).await;

                            session.send_response(response, msg.get_rpc_id()).await?;
                        },
                    )*
                    unhandled => ::tracing::warn!("can't find handler for request with message_id={unhandled}")
                }
            }

            Ok(())
        }
    };
}

requests! {
    ProtoKey;
    Login;
    Heartbeat;
}

pub async fn handle_request(
    session: &mut Session,
    msg: Message,
) -> Result<(), crate::session::SessionError> {
    match msg.get_message_id() {
        CreateCharacterRequest::MESSAGE_ID => on_create_character_request(session, msg).await,
        EnterGameRequest::MESSAGE_ID => on_enter_game_request(session, msg).await,
        _ => handle_request_impl(session, msg).await,
    }
}

async fn on_proto_key_request(
    session: &mut Session,
    request: ProtoKeyRequest,
    response: &mut ProtoKeyResponse,
) {
    tracing::debug!("on_proto_key_request: {request:?}");

    let Ok(key) = session.generate_session_key() else {
        response.error_code = ErrorCode::InternalError.into();
        return;
    };

    response.r#type = 2;
    response.key = key;
}

async fn on_login_request(
    session: &mut Session,
    request: LoginRequest,
    response: &mut LoginResponse,
) {
    tracing::debug!("on_login: {request:?}");

    let Some(account): Option<models::AccountRow> =
        query_as("SELECT * FROM t_user_account WHERE user_id = ($1)")
            .bind(&request.account)
            .fetch_optional(session.database.as_ref())
            .await
            .inspect_err(|err| tracing::error!("failed to fetch account: {err}"))
            .ok()
            .flatten()
    else {
        tracing::debug!("login: account '{}' not found", &request.account);
        response.code = ErrorCode::InvalidUserId.into();
        return;
    };

    // TODO: token check. We don't have proper sdk/login system yet.

    let last_login_trace_id = account.last_login_trace_id.unwrap_or_default();
    if last_login_trace_id != request.login_trace_id {
        tracing::debug!(
            "login: trace_id mismatch! Server: {}, client: {}, user_id: {}",
            &last_login_trace_id,
            &request.login_trace_id,
            &account.user_id
        );
        response.code = ErrorCode::LoginRetry.into();
        return;
    }

    if let Some(ban_time_stamp) = account.ban_time_stamp {
        let cur_time_stamp = time_util::unix_timestamp() as i64;
        if ban_time_stamp > cur_time_stamp {
            tracing::debug!(
                "login: account with id {} is banned until {} ({} seconds remaining)",
                &account.user_id,
                ban_time_stamp,
                ban_time_stamp - cur_time_stamp
            );
            response.code = ErrorCode::AccountIsBlocked.into();
            return;
        }
    }

    session.user_id = Some(request.account.clone());

    let player_id = query_as("SELECT * from t_user_uid WHERE user_id = ($1)")
        .bind(&request.account)
        .fetch_optional(session.database.as_ref())
        .await
        .inspect_err(|err| tracing::error!("failed to fetch player_id: {err}"))
        .ok()
        .flatten()
        .map(|u: models::UserUidRow| u.player_id);

    let Some(player_id) = player_id else {
        tracing::debug!(
            "login: first login on account {}, awaiting create character request",
            &account.user_id
        );
        response.code = ErrorCode::HaveNoCharacter.into();
        return;
    };

    session.player_id = Some(player_id);
    response.code = ErrorCode::Success.into();
    response.timestamp = time_util::unix_timestamp_ms() as i64;

    tracing::info!(
        "login success, user_id: {}, player_id: {}",
        &request.account,
        player_id
    );
}

async fn on_heartbeat_request(
    session: &mut Session,
    _: HeartbeatRequest,
    _: &mut HeartbeatResponse,
) {
    session.update_last_heartbeat_time();
}

async fn on_enter_game_request(
    session: &mut Session,
    message: Message,
) -> Result<(), crate::session::SessionError> {
    let Some(player_id) = session.player_id else {
        tracing::debug!(
            "EnterGameRequest: player_id is None, session_id: {}",
            session.get_conv_id()
        );
        return Ok(());
    };

    game_server_connection::push_message(ServiceMessage {
        src_service_id: 0,
        rpc_id: message.get_rpc_id(),
        message_id: StartPlayerSessionRequest::MESSAGE_ID,
        data: StartPlayerSessionRequest {
            gateway_session_id: session.get_conv_id(),
            player_id,
        }
        .encode_to_vec()
        .into_boxed_slice(),
    })
    .await;

    Ok(())
}

async fn on_create_character_request(
    session: &mut Session,
    mut message: Message,
) -> Result<(), crate::session::SessionError> {
    let client_request = CreateCharacterRequest::decode(message.remove_payload().as_ref())?;

    let Some(user_id) = session.user_id.clone() else {
        tracing::error!("create_character: session.user_id is None");
        return Ok(());
    };

    let create_player_request = CreatePlayerDataRequest {
        session_id: session.get_conv_id(),
        user_id,
        sex: client_request.sex,
        name: client_request.name,
    };

    game_server_connection::push_message(ServiceMessage {
        src_service_id: 0,
        rpc_id: message.get_rpc_id(),
        message_id: CreatePlayerDataRequest::MESSAGE_ID,
        data: create_player_request.encode_to_vec().into_boxed_slice(),
    })
    .await;

    Ok(())
}
