use sqlx::migrate;

mod config;
pub mod models;

pub use config::DatabaseSettings;
pub use sqlx::{query, query_as, Error, PgPool}; // re-export

pub async fn connect_to(settings: &DatabaseSettings) -> sqlx::Result<PgPool> {
    sqlx::PgPool::connect(&settings.to_string()).await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), migrate::MigrateError> {
    migrate!("./migrations").run(pool).await
}
