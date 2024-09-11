use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct DeviceId(pub String);

impl Default for DeviceId {
    fn default() -> Self {
        Self(String::from("ffffffff-ffff-ffff-ffff-ffffffffffff"))
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginParameters {
    pub login_type: u32,
    pub user_id: String,
    pub user_name: String,
    pub token: String,
    pub user_data: i32,
    #[serde(default)]
    pub device_id: DeviceId,
    pub login_trace_id: String,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoginResult {
    code: i32,
    has_rpc: bool,
    err_message: Option<String>,
    token: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    user_data: Option<i32>,
    ban_time_stamp: Option<i64>,
    sex: Option<i32>,
}

impl LoginResult {
    pub fn success(token: &str, host: &str, port: u16, sex: i32) -> Self {
        Self {
            code: 0,
            token: Some(token.to_string()),
            host: Some(host.to_string()),
            port: Some(port),
            sex: Some(sex),
            ..Default::default()
        }
    }

    pub fn error(code: i32, err_message: String) -> Self {
        Self {
            code,
            err_message: Some(err_message),
            ..Default::default()
        }
    }

    pub fn banned(err_message: String, ban_time_stamp: i64) -> Self {
        Self {
            code: -1,
            err_message: Some(err_message),
            ban_time_stamp: Some(ban_time_stamp),
            ..Default::default()
        }
    }

    pub fn with_user_data(self, user_data: i32) -> Self {
        Self {
            user_data: Some(user_data),
            has_rpc: true,
            ..self
        }
    }
}
