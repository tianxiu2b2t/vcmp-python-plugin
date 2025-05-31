/// bindgen 生成的东西还是放在 raw 里把
///
/// bindgen .\c_src\plugin.h -o .\rs_src\bindings\raw.rs  --no-layout-tests --allowlist-item="(vcmp|Server|Plugin|PLUGIN).*"
#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
pub mod raw;

pub mod error;
/// PluginFunction 的包装
/// 
/// 帮你解决好各种 call 问题
pub mod func;

// TODO: wrapper for bindings

#[derive(Debug, Clone, Copy)]
pub struct PluginInfo {
    inner: raw::PluginInfo,
}
