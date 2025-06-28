use std::ffi::CStr;
use std::os::raw::c_char;

use vcmp_bindings::{
    events::player::ClientScriptDataEvent, func::server::ServerMethods, options::VcmpEntityPool,
    raw::PluginCallbacks, vcmp_func,
};

use crate::logger;
use crate::{cfg::CONFIG, pool::ENTITY_POOL, py::load_script_as_module};

use crate::py::callbacks::CALLBACK;

#[unsafe(no_mangle)]
pub extern "C" fn on_server_init() -> u8 {
    logger::event!(logger::Level::DEBUG, "[Rust] Server init callback");

    logger::event!(
        logger::Level::DEBUG,
        "server settings {}",
        vcmp_func().server_version()
    );

    // println!("gamemode: {}", vcmp_func().get_gamemode());

    if !CONFIG.get().unwrap().preloader {
        load_script_as_module();
    }

    let _ = CALLBACK.call_func(crate::py::events::server::ServerInitialiseEvent, None);

    1
}

#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn on_server_frame(_elapsed_time: f32) {
    // println!("[Rust] Server frame callback time: {}", elapsed_time);
}

// pub fn log_msg_to_vcmp()

///
/// # Safety
/// it's for ffi
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_server_performance_report(
    entry_count: usize,
    descriptions: *mut *const c_char,
    times: *mut u64,
) {
    let _ = CALLBACK.call_func(crate::py::events::server::ServerPerformanceReportEvent::from((entry_count, descriptions, times)), None);
}

/// # Safety
///
/// ffi!
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_entity_pool_change(c_entity_type: i32, entity_id: i32, is_deleted: u8) {
    let entity_type = VcmpEntityPool::from(c_entity_type);
    let deleted = is_deleted != 0;
    logger::event!(logger::Level::DEBUG, "Entity pool change");
    logger::event!(logger::Level::DEBUG, "entity type: {entity_type:?}");
    logger::event!(logger::Level::DEBUG, "entity id: {entity_id}");
    logger::event!(logger::Level::DEBUG, "deleted: {deleted}");

    let mut pool = ENTITY_POOL.lock().expect("pool is poisoned");

    if deleted {
        pool.remove(entity_type, entity_id);
    } else {
        pool.insert(entity_type, entity_id);
    }
}

#[unsafe(no_mangle)]
/// # Safety
///
/// This function handles raw pointers from FFI. The caller must ensure:
/// - `_data` points to a valid buffer of at least `_size` bytes
/// - The buffer is not modified during the execution of this function
pub unsafe extern "C" fn on_client_script_data(_client_id: i32, _data: *const u8, _size: usize) {
    let _ = ClientScriptDataEvent::from((_client_id, _data, _size));
}

pub fn init_callbacks(callbacks: &mut PluginCallbacks) {
    callbacks.OnServerInitialise = Some(on_server_init);
    callbacks.OnServerFrame = Some(on_server_frame);
    callbacks.OnServerPerformanceReport = Some(on_server_performance_report);
    callbacks.OnEntityPoolChange = Some(on_entity_pool_change);

    callbacks.OnClientScriptData = Some(on_client_script_data);
}
