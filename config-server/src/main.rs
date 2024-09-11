use std::fs;
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
    ::common::logging::init(::tracing::Level::DEBUG);

    Application::new()
        .get("/index.json", get_index)
        .with_encryption(&CONFIG.encryption)
        .serve(&CONFIG.network)
        .await?;

    Ok(())
}

async fn get_index() -> &'static str {
    static INDEX: LazyLock<String> =
        LazyLock::new(|| fs::read_to_string("assets/config/index.json").unwrap());

    &*INDEX
}
