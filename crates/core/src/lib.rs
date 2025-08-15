use vcmp_bindings::{
    func::{server::ServerMethods, VcmpFunctions},
    raw::{PluginCallbacks, PluginFuncs, PluginInfo}, utils::set_plugin_name,
};

use tracing::{Level, event};

#[allow(clippy::missing_safety_doc)]
pub mod callbacks;
pub mod consts;
pub mod functions;

pub mod cfg;
pub mod pool;
pub mod py;
pub mod update;

/// 插件版本
///
/// semver:
/// xx yy zz
#[allow(clippy::zero_prefixed_literal)]
pub const PLUGIN_VERSION: u32 = 00_01_01;

/// 插件的指令 Identifier
/// 都是 init 的时候调用的啦
pub const PLUGIN_COMMAND: u32 = 0x7CCC_FFFF;

/// 插件名称
pub const PLUGIN_NAME: &str = "PythonPlugin";

use crate::{
    callbacks::init_callbacks,
    cfg::{get_check_update, init_config},
    py::init_py,
};

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
    init_config();

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
    info.apiMinorVersion = 0; // 就先 .0了
    info.pluginVersion = PLUGIN_VERSION;

    // set name
    let _ = set_plugin_name(PLUGIN_NAME, info);

    event!(Level::DEBUG, "vcmp-plugin-rs info: {info:?}");

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

    let version: u32 = functions.server_version();
    event!(Level::INFO, "server version: {version}");

    init_callbacks(callbacks);

    event!(Level::INFO, "vcmp-plugin-rs loaded");

    if get_check_update() {
        update::init();
    }

    1
}
