use std::{process, sync::LazyLock};

use anyhow::Result;
use config::{GatewayConfig, ServerConfig};
use shorekeeper_database::PgPool;
use shorekeeper_http::{Application, StatusCode};

mod config;
mod handler;
mod schema;

#[derive(Clone)]
pub struct ServiceState {
    pub pool: PgPool,
    pub gateway: &'static GatewayConfig,
}

#[tokio::main]
async fn main() -> Result<()> {
    static CONFIG: LazyLock<ServerConfig> =
        LazyLock::new(|| ::common::config_util::load_or_create("loginserver.toml"));

    ::common::splash::print_splash();
    ::common::logging::init(::tracing::Level::DEBUG);

    let Ok(pool) = shorekeeper_database::connect_to(&CONFIG.database).await else {
        tracing::error!(
            "Failed to connect to database with connection string: {}",
            &CONFIG.database
        );
        process::exit(1);
    };

    shorekeeper_database::run_migrations(&pool).await?;

    Application::new_with_state(ServiceState {
        pool,
        gateway: &CONFIG.gateway,
    })
    .get("/health", || async { StatusCode::OK })
    .get("/api/login", handler::handle_login_api_call)
    .serve(&CONFIG.network)
    .await?;

    Ok(())
}
