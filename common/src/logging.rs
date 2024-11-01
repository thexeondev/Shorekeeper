use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init(max_level: Level) {
    tracing_subscriber::fmt()
        .with_max_level(max_level)
        .with_target(false)
        .init();
}

pub fn init_axum(max_level: Level) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}={},tower_http={},axum::rejection=trace",
                    env!("CARGO_CRATE_NAME"),
                    max_level.as_str(),
                    max_level.as_str()
                ).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .init();
}