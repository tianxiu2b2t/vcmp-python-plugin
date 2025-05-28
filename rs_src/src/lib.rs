
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