use crate::func::VcmpFunctions;

pub trait QueryPlugin {
}
pub trait SetPlugin {
}

impl SetPlugin for VcmpFunctions {
    
}

impl QueryPlugin for VcmpFunctions {
    /// 获取插件(加载?)数量
    fn get_plugin_count(&self) -> u32 {
        (self.inner.GetNumberOfPlugins)()
    }

    /// 获取插件信息
    fn get_plugin_info(&self, id: i32) -> Option<PluginInfo> {
        let mut info = VcmpPluginInfo::new_empty();
        let info_ptr = info.inner_mut_ptr();
        let code = (self.inner.GetPluginInfo)(plugin_id, info_ptr);
        if code == 0 {
            None
        } else {
            Ok(info)
        }
    }
    /// 查找插件的 id
    fn find_plugin(&self, plugin_name: &str) -> Option<i32> {
        let ptr = plugin_name.as_ptr() as *const i8;
        let res = (self.inner.FindPlugin)(ptr);
        if res == -1 { None } else { Some(res) }
    }

    fn send_plugin_command(&self, command_identifier: i32, command: &str) -> VcmpResult<()> {
        let cmd_ptr = command.as_ptr() as *const i8;
        let code = (self.inner.SendPluginCommand)(command_identifier as u32, cmd_ptr);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

}
