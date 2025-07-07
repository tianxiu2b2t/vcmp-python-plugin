use vcmp_bindings::{
    func::{VcmpFunctions, server::ServerMethods},
    raw::{PluginCallbacks, PluginFuncs, PluginInfo},
};

use tracing::{Level, event};

pub mod callbacks;
pub mod consts;
pub mod functions;

pub mod cfg;
pub mod pool;
pub mod py;

/// 插件版本
///
/// semver:
/// xx yy zz
#[allow(clippy::zero_prefixed_literal)]
pub const PLUGIN_VERSION: u32 = 00_01_00;

use crate::{callbacks::init_callbacks, cfg::init_config, py::init_py};

/// 日志
pub mod logger;

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
    // first logger init
    logger::init();

    event!(Level::INFO, "loading vcmp-plugin-rs");
    {
        // check null
        if plugin_functions.is_null() {
            event!(Level::ERROR, "!!! plugin_functions is null !!!");
            return 0;
        }
        if plugin_callbacks.is_null() {
            event!(Level::ERROR, "!!! plugin_callbacks is null !!!");
            return 0;
        }
        if plugin_info.is_null() {
            event!(Level::ERROR, "!!! plugin_info is null !!!");
            return 0;
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

    event!(Level::TRACE, "vcmp-plugin-rs info: {info:?}");

    init_config();
    init_py();

    // struct size check
    if !(functions.inner_ffi_size() == functions.inner_struct_size()
        && std::mem::size_of::<PluginCallbacks>() == callbacks.structSize as usize)
    {
        event!(Level::WARN, "WARNING!! struct size not matching");
        if functions.inner_ffi_size() != functions.inner_struct_size() {
            event!(
                Level::WARN,
                "func expect size: {}, actuall ffi size: {}",
                functions.inner_ffi_size(),
                functions.inner_struct_size()
            );
        }
        if std::mem::size_of::<PluginCallbacks>() != callbacks.structSize as usize {
            event!(
                Level::WARN,
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
    event!(Level::DEBUG, "server version: {version}");

    //println!("ready to getsetting");
    //let server_settings = functions.get_server_settings();
    //println!("server settings: {}", server_settings);
    //functions.set_server_name("测试服务器");
    //let server_settings = functions.get_server_settings();
    //println!("server settings: {}", server_settings);

    init_callbacks(callbacks);

    event!(Level::INFO, "vcmp-plugin-rs loaded");

    1
}
