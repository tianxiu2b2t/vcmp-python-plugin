/// bindgen 生成的绑定
///
/// bindgen .\c_src\plugin.h -o .\rs_src\src\bindings.rs  --no-layout-tests --allowlist-item="(vcmp|Server|Plugin|PLUGIN).*"
#[allow(non_camel_case_types, non_snake_case)]
pub mod bindings;

/// 插件入口点
///
/// 好消息: 反正也没别的需要 extern 的函数了
///
/// ```c
/// extern "C" EXPORT uint32_t VcmpPluginInit(PluginFuncs* pluginFunctions, PluginCallbacks* pluginCallbacks, PluginInfo* pluginInfo);
/// ```
#[unsafe(no_mangle)]
extern "C" fn VcmpPluginInit() -> u32 {
    return 1;
}
