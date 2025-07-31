pub mod sq;
pub mod sq_ffi;

use tracing::event;
use vcmp_bindings::{func::PluginMethods, vcmp_func};

use crate::sq::init_sq_imports;

pub fn init_squirrel() {
    let id = match vcmp_func().find_plugin("SQHost2\0") {
        Some(id) => id,
        None => {
            event!(tracing::Level::ERROR, "Failed to find SQHost2 plugin");
            return;
        },
    };
    let res = init_sq_imports(vcmp_func().get_plugin_exports(id));
    if res.is_err() {
        event!(tracing::Level::ERROR, "Failed to initialize Squirrel FFI");
        return;
    }
    event!(tracing::Level::INFO, "Squirrel FFI initialized");
}
