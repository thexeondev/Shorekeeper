use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct AccountRow {
    pub user_name: String,
    pub user_id: String,
    pub token: String,
    pub sex: i32,
    pub create_time_stamp: i64,
    pub create_device_id: String,
    pub ban_time_stamp: Option<i64>,
    pub last_login_trace_id: Option<String>,
}

#[derive(FromRow)]
pub struct UserUidRow {
    pub user_id: String,
    pub player_id: i32,
    pub sex: i32,
    pub create_time_stamp: i64,
}

#[derive(FromRow)]
pub struct PlayerDataRow {
    pub player_id: i32,
    pub name: String,
    pub bin_data: Vec<u8>,
}
