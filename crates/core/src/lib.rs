use std::{ffi::{CStr, c_char}, cell::OnceCell};

use vcmp_bindings::raw::{PluginCallbacks, PluginFuncs, PluginInfo};

pub const PLUGIN_VERSION: u32 = 1;

/// 插件入口点
///
/// 好消息: 反正也没别的需要 extern 的函数了
///
/// ```c
/// extern "C" EXPORT uint32_t VcmpPluginInit(PluginFuncs* pluginFunctions, PluginCallbacks* pluginCallbacks, PluginInfo* pluginInfo);
/// ```
#[unsafe(no_mangle)]
extern "C" fn VcmpPluginInit(
    plugin_functions: *mut PluginFuncs,
    plugin_callbacks: *mut PluginCallbacks,
    plugin_info: *mut PluginInfo,
) -> u32 {
    println!("loading vcmp-plugin-rs");

    {
        // check null
        if plugin_functions.is_null() {
            println!("!!! plugin_functions is null !!!");
            return 1;
        }
        if plugin_callbacks.is_null() {
            println!("!!! plugin_callbacks is null !!!");
            return 1;
        }
        if plugin_info.is_null() {
            println!("!!! plugin_info is null !!!");
            return 1;
        }
    }

    let (functions, callbacks, info) = unsafe {
        (
            &mut *plugin_functions,
            &mut *plugin_callbacks,
            &mut *plugin_info,
        )
    };

    // 参考 cpp.ing
    info.apiMajorVersion = 2;
    // info.apiMinorVersion = bindings::API_MINOR as u16 - 1; // 难蚌 compat
    info.apiMinorVersion = 0; // 就先 .0了
    info.pluginVersion = PLUGIN_VERSION;

    callbacks.OnServerPerformanceReport = Some(on_server_performance_report);

    println!("vcmp-plugin-rs info: {:#?}", info);
    println!("vcmp-plugin-rs callback: {:#?}", callbacks);

    println!(
        "sizeof callback: {}",
        std::mem::size_of::<PluginCallbacks>()
    );
    println!("sizeof functions: {}", std::mem::size_of::<PluginFuncs>());

    println!("give sizeof callback: {}", callbacks.structSize);
    println!("give sizeof functions: {}", functions.structSize);

    // get version
    let version: u32 = (functions.GetServerVersion)();
    println!("server version: {}", version);
    callbacks.OnServerFrame = Some(on_server_frame);

    println!("vcmp-plugin-rs loaded");

    1
}

extern "C" fn on_server_frame(elapsed_time: f32) {
    // println!("[Rust] Server frame callback time: {}", elapsed_time);
}

// pub fn log_msg_to_vcmp()

extern "C" fn on_server_performance_report(
    entry_count: usize,
    descriptions: *mut *const c_char,
    times: *mut u64,
) {
    println!("[Rust] Server performance report callback");
    let c_str_descriptions = unsafe { CStr::from_ptr(*descriptions) };
    let description = c_str_descriptions // array
        .to_str()
        .unwrap_or("Could not convert description to string");
    let times = unsafe { *times }; // array
    println!(
        "[Rust] Description: {}, entry count: {}, time: {}",
        description, entry_count, times
    );
}
