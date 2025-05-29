use bindings::raw::{PluginCallbacks, PluginFuncs, PluginInfo};

pub mod bindings;


pub const PLUGIN_VERSION: u32 = 1;

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

    unsafe {
        let functions = &mut *plugin_functions;
        let callbacks = &mut *plugin_callbacks;
        let info = &mut *plugin_info;

        // 参考 cpp.ing
        info.apiMajorVersion = bindings::API_MAJOR as u16;
        // info.apiMinorVersion = bindings::API_MINOR as u16 - 1; // 难蚌 compat
        info.apiMinorVersion = 0;
        info.pluginVersion = PLUGIN_VERSION;

        println!("vcmp-plugin-rs info: {:#?}", info);
        println!("vcmp-plugin-rs callback: {:#?}", callbacks);
        // println!("vcmp-plugin-rs functions: {:?}", functions);

        println!("sizeof callback: {}", std::mem::size_of::<PluginCallbacks>());
        println!("sizeof functions: {}", std::mem::size_of::<PluginFuncs>());

        println!("give sizeof callback: {}", callbacks.structSize);
        println!("give sizeof functions: {}", functions.structSize);

        // get version
        let version: u32 = functions.GetServerVersion.unwrap()();
        println!("server version: {}", version);
    }

    println!("vcmp-plugin-rs loaded");

    1
}
