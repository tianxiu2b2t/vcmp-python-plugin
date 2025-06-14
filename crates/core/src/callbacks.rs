use std::ffi::CStr;
use std::os::raw::c_char;

use vcmp_bindings::{
    func::server::ServerMethods, options::VcmpEntityPool, raw::PluginCallbacks, vcmp_func,
};

use crate::pool::ENTITY_POOL;

#[unsafe(no_mangle)]
pub extern "C" fn on_server_init() -> u8 {
    println!("[Rust] Server init callback");

    println!("server settings {}", vcmp_func().server_version());

    // println!("gamemode: {}", vcmp_func().get_gamemode());

    vcmp_func()
        .set_gamemode(&("*".repeat(63)))
        .expect("set gamemode faild");

    println!("gamemode: {}", vcmp_func().get_gamemode());

    1
}

#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn on_server_frame(elapsed_time: f32) {
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
    // descriptions is array and times is array
    for i in 0..entry_count {
        let description = unsafe { CStr::from_ptr(*descriptions.add(i)) };
        let time = unsafe { *times.add(i) };
        println!(
            "Performance report: {} - {}",
            description.to_string_lossy(),
            time
        );
    }
}

/// # Safety
///
/// ffi!
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_entity_pool_change(c_entity_type: i32, entity_id: i32, is_deleted: u8) {
    let entity_type = VcmpEntityPool::from(c_entity_type);
    let deleted = is_deleted != 0;
    println!("Entity pool change");
    println!("entity type: {entity_type:?}");
    println!("entity id: {entity_id}");
    println!("deleted: {deleted}");

    let mut pool = ENTITY_POOL.lock().expect("pool is poisoned");

    if deleted {
        pool.remove(entity_type, entity_id);
    } else {
        pool.insert(entity_type, entity_id);
    }
}

pub fn init_callbacks(callbacks: &mut PluginCallbacks) {
    callbacks.OnServerInitialise = Some(on_server_init);
    callbacks.OnServerFrame = Some(on_server_frame);
    callbacks.OnServerPerformanceReport = Some(on_server_performance_report);
    callbacks.OnEntityPoolChange = Some(on_entity_pool_change);
}
