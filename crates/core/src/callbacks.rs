use std::os::raw::c_char;

use crate::py::events::{player::*, server::*};
use vcmp_bindings::{
    events::player::ClientScriptDataEvent, options::VcmpEntityPool, raw::PluginCallbacks,
};

use crate::logger;
use crate::{cfg::CONFIG, pool::ENTITY_POOL, py::load_script_as_module};

use crate::py::callbacks::CALLBACK;

#[unsafe(no_mangle)]
pub extern "C" fn on_server_init() -> u8 {
    logger::event!(logger::Level::TRACE, "[Rust] Server init callback");

    if !CONFIG.get().unwrap().preloader {
        load_script_as_module();
    }

    let _ = CALLBACK.call_func(ServerInitialiseEvent, None);

    1
}

#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn on_server_frame(elapsed_time: f32) {
    // println!("[Rust] Server frame callback time: {}", elapsed_time);
    let _ = CALLBACK.call_func(ServerFrameEvent::from(elapsed_time), None);
}

///
/// # Safety
/// it's for ffi
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_server_performance_report(
    entry_count: usize,
    descriptions: *mut *const c_char,
    times: *mut u64,
) {
    let _ = CALLBACK.call_func(
        ServerPerformanceReportEvent::from((entry_count, descriptions, times)),
        None,
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn on_server_shutdown() {
    let _ = CALLBACK.call_func(ServerShutdownEvent, None);
}

/// # Safety
///
/// ffi!
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_entity_pool_change(c_entity_type: i32, entity_id: i32, is_deleted: u8) {
    let entity_type = VcmpEntityPool::from(c_entity_type);
    let deleted = is_deleted != 0;
    logger::event!(logger::Level::TRACE, "Entity pool change");
    logger::event!(logger::Level::TRACE, "entity type: {entity_type:?}");
    logger::event!(logger::Level::TRACE, "entity id: {entity_id}");
    logger::event!(logger::Level::TRACE, "deleted: {deleted}");

    let mut pool = ENTITY_POOL.lock().expect("pool is poisoned");

    if deleted {
        pool.remove(entity_type, entity_id);
    } else {
        pool.insert(entity_type, entity_id);
    }
}

#[unsafe(no_mangle)]
/// # Safety
pub unsafe extern "C" fn on_incoming_connection(
    player_name: *mut ::std::os::raw::c_char,
    name_buffer_size: usize,
    user_password: *const ::std::os::raw::c_char,
    ip_address: *const ::std::os::raw::c_char,
) -> u8 {
    CALLBACK.call_func(
        IncomingConnectionEvent::from((player_name, name_buffer_size, user_password, ip_address)),
        None,
    ) as u8
}

#[unsafe(no_mangle)]
/// # Safety
pub unsafe extern "C" fn on_player_connect(player_id: i32) {
    // first call on_entity_pool_change, maybe vcmp not call on_entity_pool_change?
    unsafe {
        on_entity_pool_change(VcmpEntityPool::Player as i32, player_id, 0);
    }

    let _ = CALLBACK.call_func(PlayerConnectEvent::from(player_id), None);
}

#[unsafe(no_mangle)]
/// # Safety
pub unsafe extern "C" fn on_player_disconnect(player_id: i32, reason: i32) {
    let _ = CALLBACK.call_func(PlayerDisconnectEvent::from((player_id, reason)), None);

    unsafe {
        on_entity_pool_change(VcmpEntityPool::Player as i32, player_id, 1);
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
    callbacks.OnServerShutdown = Some(on_server_shutdown);

    callbacks.OnIncomingConnection = Some(on_incoming_connection);
    callbacks.OnPlayerConnect = Some(on_player_connect);
    callbacks.OnPlayerDisconnect = Some(on_player_disconnect);

    callbacks.OnEntityPoolChange = Some(on_entity_pool_change);

    callbacks.OnClientScriptData = Some(on_client_script_data);
}
