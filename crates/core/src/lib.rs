use vcmp_bindings::{
    func::{VcmpFunctions, server::ServerMethods},
    raw::{PluginCallbacks, PluginFuncs, PluginInfo},
};

pub mod callbacks;
pub mod func;
pub mod var;

pub mod cfg;
pub mod py;

pub mod event;

use var::PLUGIN_VERSION;

use crate::{callbacks::init_callbacks, cfg::init_config, py::init_py};

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

    println!("vcmp-plugin-rs info: {info:?}");

    init_config();
    init_py();

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

    //println!("ready to getsetting");
    //let server_settings = functions.get_server_settings();
    //println!("server settings: {}", server_settings);
    //functions.set_server_name("测试服务器");
    //let server_settings = functions.get_server_settings();
    //println!("server settings: {}", server_settings);

    init_callbacks(callbacks);

    println!("vcmp-plugin-rs loaded");

    1
}
