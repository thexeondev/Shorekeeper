use std::sync::LazyLock;

use anyhow::Result;

use common::config_util::{self, TomlConfig};
use serde::Deserialize;
use shorekeeper_http::{
    config::{AesSettings, NetworkSettings},
    Application,
};

#[derive(Deserialize)]
pub struct ServerConfig {
    pub network: NetworkSettings,
    pub encryption: AesSettings,
}

impl TomlConfig for ServerConfig {
    const DEFAULT_TOML: &str = include_str!("../configserver.default.toml");
}

#[tokio::main]
async fn main() -> Result<()> {
    static CONFIG: LazyLock<ServerConfig> =
        LazyLock::new(|| config_util::load_or_create("configserver.toml"));

    ::common::splash::print_splash();
    ::common::logging::init_axum(::tracing::Level::DEBUG);

    Application::new()
        .serve_dir("/", "assets/config")
        .with_encryption(&CONFIG.encryption)
        .with_logger()
        .serve(&CONFIG.network)
        .await?;

    Ok(())
}
