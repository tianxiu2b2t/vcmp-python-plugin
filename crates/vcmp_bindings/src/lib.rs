/// bindgen 生成的东西还是放在 raw 里把
///
/// bindgen .\c_src\plugin.h -o .\rs_src\bindings\raw.rs  --no-layout-tests --allowlist-item="(vcmp|Server|Plugin|PLUGIN).*"
#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
pub mod raw;

/// vcmp error & vcmp result
///
/// error wrapper
pub mod error;
/// PluginFunction 的包装
///
/// 帮你解决好各种 call 问题
pub mod func;

/// gbk <-> utf8
pub mod encodes;

use std::fmt::Display;

pub use error::{VcmpError, VcmpResult};

// TODO: wrapper for bindings

#[derive(Debug, Clone, Copy)]
pub struct PluginInfo {
    inner: raw::PluginInfo,
}

impl PluginInfo {
    pub fn new_empty() -> Self {
        Self {
            inner: raw::PluginInfo {
                structSize: 0,
                pluginId: 0,
                name: [0; 32],
                pluginVersion: 0,
                apiMajorVersion: 0,
                apiMinorVersion: 0,
            },
        }
    }

    /// 获取 内部的可变指针
    pub fn inner_mut_ptr(&mut self) -> *mut raw::PluginInfo {
        &mut self.inner
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ServerSettings {
    inner: raw::ServerSettings,
}

impl ServerSettings {
    pub fn server_name(&self) -> String {
        crate::encodes::decode_gbk(
            &(self
                .inner
                .serverName
                .iter()
                .map(|v| *v as u8)
                .collect::<Vec<u8>>()),
        )
    }

    pub fn port(&self) -> u32 {
        self.inner.port
    }

    pub fn max_players(&self) -> u32 {
        self.inner.maxPlayers
    }

    pub fn flags(&self) -> u32 {
        self.inner.flags
    }

    pub fn new_empty() -> Self {
        Self {
            inner: raw::ServerSettings {
                structSize: 0,
                serverName: [0; 128],
                maxPlayers: 0,
                port: 0,
                flags: 0,
            },
        }
    }

    /// 获取 内部的可变指针
    pub fn inner_mut_ptr(&mut self) -> *mut raw::ServerSettings {
        &mut self.inner
    }
}

impl From<raw::ServerSettings> for ServerSettings {
    fn from(value: raw::ServerSettings) -> Self {
        Self { inner: value }
    }
}

impl From<*mut raw::ServerSettings> for ServerSettings {
    fn from(value: *mut raw::ServerSettings) -> Self {
        Self {
            inner: unsafe { *value },
        }
    }
}

impl Display for ServerSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ServerSettings {{ server_name: {}, port: {}, max_players: {}, flags: {} }}",
            self.server_name(),
            self.port(),
            self.max_players(),
            self.flags()
        )
    }
}
