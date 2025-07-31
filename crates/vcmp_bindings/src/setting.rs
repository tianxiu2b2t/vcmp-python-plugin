use std::fmt::Display;

use crate::raw;

#[derive(Debug, Clone, Copy)]
pub struct VcmpServerSettings {
    inner: raw::ServerSettings,
}

impl VcmpServerSettings {
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

    pub fn server_name(&self) -> String {
        crate::encodes::decode_gbk(
            &(self
                .inner
                .serverName
                .iter()
                .map(|v| *v as u8)
                .collect::<Vec<u8>>()),
        ).trim_end_matches("\0").to_string()
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

    /// 获取 内部的可变指针
    pub fn inner_mut_ptr(&mut self) -> *mut raw::ServerSettings {
        &mut self.inner
    }
}

impl From<raw::ServerSettings> for VcmpServerSettings {
    fn from(value: raw::ServerSettings) -> Self {
        Self { inner: value }
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
impl From<*mut raw::ServerSettings> for VcmpServerSettings {
    fn from(value: *mut raw::ServerSettings) -> Self {
        Self {
            inner: unsafe { *value },
        }
    }
}

impl Display for VcmpServerSettings {
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
