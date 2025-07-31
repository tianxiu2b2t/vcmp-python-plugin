use std::fmt::Display;

use crate::raw;

#[derive(Debug, Clone, Copy)]
pub struct VcmpPluginInfo {
    inner: raw::PluginInfo,
}

impl VcmpPluginInfo {
    pub fn new_empty() -> Self {
        Self {
            inner: raw::PluginInfo {
                structSize: 48, // 神奇的结构体（？
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

    pub fn plugin_id(&self) -> u32 {
        self.inner.pluginId
    }

    pub fn name(&self) -> String {
        crate::encodes::decode_gbk(
            &(self
                .inner
                .name
                .iter()
                .map(|v| *v as u8)
                .collect::<Vec<u8>>()),
        ).trim_end_matches("\0").to_string()
    }

    pub fn plugin_version(&self) -> u32 {
        self.inner.pluginVersion
    }

    pub fn api_major_version(&self) -> u16 {
        self.inner.apiMajorVersion
    }

    pub fn api_minor_version(&self) -> u16 {
        self.inner.apiMinorVersion
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
impl From<*mut raw::PluginInfo> for VcmpPluginInfo {
    fn from(value: *mut raw::PluginInfo) -> Self {
        Self {
            inner: unsafe { *value },
        }
    }
}

impl Display for VcmpPluginInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PluginInfo {{ plugin_id: {}, name: {}, plugin_version: {}, api_major_version: {}, api_minor_version: {} }}",
            self.plugin_id(),
            self.name(),
            self.plugin_version(),
            self.api_major_version(),
            self.api_minor_version()
        )
    }
}
