use std::{
    ffi::{CStr, c_char},
    sync::OnceLock,
};

use vcmp_bindings::{
    func::{VcmpFunctions, server::ServerMethods},
    raw::{PluginCallbacks, PluginFuncs, PluginInfo},
    vcmp_func,
};

pub const PLUGIN_VERSION: u32 = 1;

pub static VCMP_INIT: OnceLock<()> = OnceLock::new();

/// 验证 vcmp 是否初始化
pub fn vcmp_inited() -> bool {
    VCMP_INIT.get().is_some()
}

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

    let (callbacks, info) = unsafe { (&mut *plugin_callbacks, &mut *plugin_info) };

    let functions = VcmpFunctions::from(plugin_functions);

    let functions = vcmp_bindings::init_vcmp_func(functions);

    // 参考 cpp.ing
    info.apiMajorVersion = 2;
    // info.apiMinorVersion = bindings::API_MINOR as u16 - 1; // 难蚌 compat
    info.apiMinorVersion = 0; // 就先 .0了
    info.pluginVersion = PLUGIN_VERSION;

    callbacks.OnServerPerformanceReport = Some(on_server_performance_report);
    callbacks.OnServerInitialise = Some(on_server_init);

    println!("vcmp-plugin-rs info: {info:?}");

    // struct size check
    if !(functions.inner_ffi_size() == functions.inner_struct_size()
        && std::mem::size_of::<PluginCallbacks>() == callbacks.structSize as usize)
    {
        println!("WARNING!! struct size not matching");
        if functions.inner_ffi_size() != functions.inner_struct_size() {
            println!(
                "func expect size: {}, actuall ffi size: {}",
                functions.inner_ffi_size(),
                functions.inner_struct_size()
            );
        }
        if std::mem::size_of::<PluginCallbacks>() != callbacks.structSize as usize {
            println!(
                "callback expect size: {}, actuall ffi size {}",
                std::mem::size_of::<PluginCallbacks>(),
                callbacks.structSize
            );
        }
    }

    // println!(
    //     "sizeof callback: {}",
    //     std::mem::size_of::<PluginCallbacks>()
    // );
    // println!("sizeof functions: {}", std::mem::size_of::<PluginFuncs>());

    // println!("give sizeof callback: {}", callbacks.structSize);
    // println!("give sizeof functions: {}", functions.inner_struct_size());

    // get version
    let version: u32 = functions.server_version();
    println!("server version: {version}");
    callbacks.OnServerFrame = Some(on_server_frame);

    //println!("ready to getsetting");
    //let server_settings = functions.get_server_settings();
    //println!("server settings: {}", server_settings);
    //functions.set_server_name("测试服务器");
    //let server_settings = functions.get_server_settings();
    //println!("server settings: {}", server_settings);

    println!("rust 直接输出中文 test");
    functions.log_message("VCMP 输出中文 test\n");

    println!("vcmp-plugin-rs loaded");

    1
}

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
