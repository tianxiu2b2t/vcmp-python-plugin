use tracing::Level;

pub fn init() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
}
