use tracing_subscriber::prelude::*;


pub fn init() {
    tracing_subscriber::registry().init();
}

pub use tracing::{event, Level};