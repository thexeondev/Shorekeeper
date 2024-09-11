use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub host: String,
    pub user_name: String,
    pub password: String,
    pub db_name: String,
}

impl fmt::Display for DatabaseSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "postgres://{}:{}@{}/{}",
            &self.user_name, &self.password, &self.host, &self.db_name
        )
    }
}
