use std::ffi::c_void;

use crate::{ServerSettings, VcmpError, VcmpPluginInfo, VcmpResult, raw::PluginFuncs};

pub struct VcmpFunctions {
    inner: PluginFuncs,
}

unsafe impl Sync for PluginFuncs {}

unsafe impl Send for PluginFuncs {}

impl From<PluginFuncs> for VcmpFunctions {
    fn from(value: PluginFuncs) -> Self {
        Self { inner: value }
    }
}

impl From<*mut PluginFuncs> for VcmpFunctions {
    fn from(value: *mut PluginFuncs) -> Self {
        let inner = unsafe { *value };
        Self { inner }
    }
}

impl VcmpFunctions {
    /// 获取 vcmp 服务器给过来的 struct 的大小
    pub fn inner_struct_size(&self) -> usize {
        self.inner.structSize as usize
    }

    /// 获取 bindgen 生成的结构体的大小
    pub fn inner_ffi_size(&self) -> usize {
        std::mem::size_of::<PluginFuncs>()
    }

    /// 获取服务器版本号
    pub fn server_version(&self) -> u32 {
        (self.inner.GetServerVersion)()
    }

    /// 获取服务器设置
    pub fn get_server_settings(&self) -> Option<ServerSettings> {
        let mut setting = ServerSettings::new_empty();
        let setting_ptr = setting.inner_mut_ptr();
        let _ = (self.inner.GetServerSettings)(setting_ptr);
        setting
    }

    pub fn get_plugin_count(&self) -> u32 {
        (self.inner.GetNumberOfPlugins)()
    }

    pub fn get_plugin_info(&self, plugin_id: i32) -> VcmpResult<VcmpPluginInfo> {
        let mut info = VcmpPluginInfo::new_empty();
        let info_ptr = info.inner_mut_ptr();
        let code = (self.inner.GetPluginInfo)(plugin_id, info_ptr);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(info)
        }
    }

    pub fn find_plugin(&self, plugin_name: &str) -> Option<i32> {
        let name = plugin_name.as_bytes();
        let ptr = name.as_ptr() as *const i8;
        let res = (self.inner.FindPlugin)(ptr);
        if res == -1 { None } else { Some(res) }
    }

    /// TODO
    pub fn send_plugin_command(&self, command_identifier: i32, command: &str) -> VcmpResult<()> {
        let cmd = command.as_bytes();
        let cmd_ptr = cmd.as_ptr() as *const i8;
        let code = (self.inner.SendPluginCommand)(command_identifier as u32, cmd_ptr);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    pub fn get_last_error(&self) -> VcmpError {
        VcmpError::from((self.inner.GetLastError)())
    }

    pub fn log_message(&self, message: &str) {
        let msg = message.as_bytes();
        let msg_ptr = msg.as_ptr() as *const i8;
        let _ = (self.inner.LogMessage)(msg_ptr);
    }

    /**
     * Client messages
     */

    /// 发送玩家脚本数据
    ///
    /// data: &[u8]
    pub fn send_client_script_data(&self, player_id: i32, data: &[u8]) -> VcmpResult<()> {
        let msg_ptr = data.as_ptr() as *const c_void;
        let code = (self.inner.SendClientScriptData)(player_id, msg_ptr, data.len());
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    ///  发送玩家消息
    ///
    /// color: 反正是什么可以 into u32 的东西
    pub fn send_client_message(
        &self,
        player_id: i32,
        color: impl Into<u32>,
        message: &str,
    ) -> VcmpResult<()> {
        let color: u32 = color.into();

        let msg = message.as_bytes();
        let msg_ptr = msg.as_ptr() as *const i8;
        let code = (self.inner.SendClientMessage)(player_id, color, msg_ptr);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    pub fn send_announce(
        &self,
        player_id: i32,
        announce_type: i32,
        message: &str,
    ) -> VcmpResult<()> {
        let msg = message.as_bytes();
        let msg_ptr = msg.as_ptr() as *const i8;
        let code = (self.inner.SendGameMessage)(player_id, announce_type, msg_ptr);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    /*
       Server Settings
    */

    pub fn set_server_name(&self, name: &str) -> VcmpResult<()> {
        // TODO: name need convert to gbk
        let name = name.as_bytes();
        let name_ptr = name.as_ptr() as *const i8;
        let code = (self.inner.SetServerName)(name_ptr);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    pub fn set_server_password(&self, password: &str) -> VcmpResult<()> {
        let password = password.as_bytes();
        let password_ptr = password.as_ptr() as *const i8;
        let code = (self.inner.SetServerPassword)(password_ptr);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
}
