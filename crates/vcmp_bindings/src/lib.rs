/// bindgen 生成的东西还是放在 raw 里把
///
/// bindgen .\c_src\plugin.h -o .\rs_src\bindings\raw.rs  --no-layout-tests --allowlist-item="(vcmp|Server|Plugin|PLUGIN).*"
#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
pub mod raw;

/// gbk <-> utf8
pub mod encodes;
/// vcmp error & vcmp result
///
/// error wrapper
pub mod error;
/// PluginFunction 的包装
///
/// 帮你解决好各种 call 问题
pub mod func;
/// wrapper for PluginInfo
pub mod plugin_info;

pub mod setting;

pub use error::{VcmpError, VcmpResult};
pub use plugin_info::VcmpPluginInfo;

// TODO: wrapper for bindings
