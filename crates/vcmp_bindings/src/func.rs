use std::ffi::c_void;

use crate::{PluginInfo, ServerSettings, VcmpError, VcmpResult, raw::PluginFuncs};

pub struct VcmpFunctions {
    inner: PluginFuncs,
}

impl From<PluginFuncs> for VcmpFunctions {
    fn from(value: PluginFuncs) -> Self {
        Self { inner: value }
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
    pub fn get_server_version(&self) -> u32 {
        (self.inner.GetServerVersion)()
    }

    /// 获取服务器设置
    pub fn get_server_settings(&self) -> VcmpResult<ServerSettings> {
        let mut setting = ServerSettings::new_empty();
        let setting_ptr = setting.inner_mut_ptr();
        let code = (self.inner.GetServerSettings)(setting_ptr);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(setting)
        }
    }

    pub fn get_plugin_count(&self) -> u32 {
        (self.inner.GetNumberOfPlugins)()
    }

    pub fn get_plugin_info(&self, plugin_id: i32) -> VcmpResult<PluginInfo> {
        let mut info = PluginInfo::new_empty();
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
        if res == -1 {
            None
        } else {
            Some(res)
        }
    }

    /// TODO
    pub fn send_plugin_command(&self, command_identifier: i32, command: &str) -> VcmpResult<()> {
        let cmd = command.as_bytes();
        let cmd_ptr =  cmd.as_ptr() as *const i8;
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

    pub fn send_client_script_data(&self, player_id: i32, data: &[u8]) -> VcmpResult<()> {
        let msg_ptr = data.as_ptr() as *const c_void;
        let code = (self.inner.SendClientScriptData)(player_id, msg_ptr, data.len());
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    // pub fn send_client_message_ex(&self, player_id: i32, color: u32, message: &str) -> VcmpResult<()> {
        
    // }
}
