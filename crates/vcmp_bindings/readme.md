<div align="center">

# VC:MP Server Bindings For Rust

VC:MP Server `plugin.h` Bindings For Rust

</div>

# Usage

```rust
use vcmp_bindings::{
    func::VcmpFunctions,
    raw::{PluginCallbacks, PluginFuncs, PluginInfo},
};

#[unsafe(no_mangle)]
extern "C" fn VcmpPluginInit(
    plugin_functions: *mut PluginFuncs,
    plugin_callbacks: *mut PluginCallbacks,
    plugin_info: *mut PluginInfo,
) -> u32 {
    {
        // check null
        if plugin_functions.is_null() {
            println!("!!! plugin_functions is null !!!");
            return 0;
        }
        if plugin_callbacks.is_null() {
            println!("!!! plugin_callbacks is null !!!");
            return 0;
        }
        if plugin_info.is_null() {
            println!("!!! plugin_info is null !!!");
            return 0;
        }
    }

    let (callbacks, info) = unsafe { (&mut *plugin_callbacks, &mut *plugin_info) };

    let functions = VcmpFunctions::from(plugin_functions);

    let functions = vcmp_bindings::init_vcmp_func(functions);

    info.apiMajorVersion = 2;
    info.apiMinorVersion = 0; 
    info.pluginVersion = PLUGIN_VERSION;


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

    1
}

```