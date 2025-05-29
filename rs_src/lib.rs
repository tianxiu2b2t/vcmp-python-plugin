use bindings::raw::{PluginCallbacks, PluginFuncs, PluginInfo};

pub mod bindings;

/// 插件入口点
///
/// 好消息: 反正也没别的需要 extern 的函数了
///
/// ```c
/// extern "C" EXPORT uint32_t VcmpPluginInit(PluginFuncs* pluginFunctions, PluginCallbacks* pluginCallbacks, PluginInfo* pluginInfo);
/// ```
#[unsafe(no_mangle)]
extern "C" fn VcmpPluginInit(plugin_functions: *mut PluginFuncs, plugin_callbacks: *mut PluginCallbacks, plugin_info: *mut PluginInfo) -> u32 {
    
    println!("loading vcmp-plugin-rs");

    unsafe {
        let functions = &mut *plugin_functions;
        let callbacks = &mut *plugin_callbacks;
        let info = &mut *plugin_info;

        // 参考 cpp.ing
        info.apiMajorVersion = bindings::API_MAJOR as u16;
        info.apiMinorVersion = bindings::API_MINOR as u16;
        info.pluginVersion = 1;

        println!("vcmp-plugin-rs id: {}", info.pluginId);
        
    }

    println!("vcmp-plugin-rs loaded");

    1
}
