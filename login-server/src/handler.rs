use crate::{schema, ServiceState};
use common::time_util;
use shorekeeper_database::{models::AccountRow, query_as};
use shorekeeper_http::{Json, Query, State};

#[tracing::instrument(skip(state))]
pub async fn handle_login_api_call(
    State(state): State<ServiceState>,
    Query(parameters): Query<schema::LoginParameters>,
) -> Json<schema::LoginResult> {
    tracing::debug!("login requested");

    let user_data = parameters.user_data;
    let result = login(&state, parameters).await.unwrap_or_else(|err| {
        tracing::warn!("login: internal error occurred {err:?}");
        schema::LoginResult::error(-1, String::from("Internal server error"))
    });

    Json(result.with_user_data(user_data))
}

async fn login(state: &ServiceState, params: schema::LoginParameters) -> Result<schema::LoginResult, shorekeeper_database::Error> {
    if params.login_type != 0 {
        return Ok(schema::LoginResult::error(-1, String::from("SDK login is not allowed on this server")));
    }

    let result: Option<AccountRow> = query_as("SELECT * FROM t_user_account WHERE user_id = ($1)").bind(params.user_id.as_str()).fetch_optional(&state.pool).await?;
    let account: AccountRow = match result {
        Some(account) => { 
            if let Some(ban_time_stamp) = account.ban_time_stamp {
                if time_util::unix_timestamp() < ban_time_stamp as u64 {
                    return Ok(schema::LoginResult::banned(String::from("You're banned MF"), ban_time_stamp));
                }
            }

            query_as("UPDATE t_user_account SET last_login_trace_id = ($1) where user_id = ($2) RETURNING *").bind(params.login_trace_id).bind(params.user_id).fetch_one(&state.pool).await?
        },
        None => query_as("INSERT INTO t_user_account (user_name, user_id, token, create_time_stamp, create_device_id, last_login_trace_id) values ($1, $2, $3, $4, $5, $6) RETURNING *")
            .bind(params.user_name.as_str())
            .bind(params.user_id.as_str())
            .bind(params.token.as_str())
            .bind(time_util::unix_timestamp() as i64)
            .bind(params.device_id.0.as_str())
            .bind(params.login_trace_id.as_str())
            .fetch_one(&state.pool).await?
    };

    Ok(schema::LoginResult::success(&account.token, &state.gateway.host, state.gateway.port, account.sex))
}
