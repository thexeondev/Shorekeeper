use tracing::Level;

pub fn init(max_level: Level) {
    tracing_subscriber::fmt()
        .with_max_level(max_level)
        .with_target(false)
        .init();
}
