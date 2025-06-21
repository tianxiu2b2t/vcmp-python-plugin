use tracing_subscriber::prelude::*;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::registry::Registry;

pub mod format;

pub fn init() {
    let subscriber = Registry::default()
        .with(tracing_subscriber::fmt::layer()
            .event_format(format::CustomFormat)
            .with_writer(std::io::stdout));

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");
}

pub use tracing::{Level, event};
