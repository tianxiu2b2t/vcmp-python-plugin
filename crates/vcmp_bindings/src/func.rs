use std::ffi::c_void;

use crate::{VcmpError, VcmpPluginInfo, VcmpResult, raw::PluginFuncs, setting::VcmpServerSettings};

use crate::encodes::{decode_gbk, encode_to_gbk};

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
    pub fn get_server_settings(&self) -> VcmpServerSettings {
        let mut setting = VcmpServerSettings::new_empty();
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
        let ptr = plugin_name.as_ptr() as *const i8;
        let res = (self.inner.FindPlugin)(ptr);
        if res == -1 { None } else { Some(res) }
    }

    /// TODO
    pub fn send_plugin_command(&self, command_identifier: i32, command: &str) -> VcmpResult<()> {
        let cmd_ptr = command.as_ptr() as *const i8;
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
        let msg = encode_to_gbk(message);
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
        let msg_ptr = message.as_ptr() as *const i8;
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
        let name = encode_to_gbk(name);
        let name_ptr = name.as_ptr() as *const i8;
        let code = (self.inner.SetServerName)(name_ptr);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    pub fn set_server_password(&self, password: &str) -> VcmpResult<()> {
        let password = encode_to_gbk(password);
        let password_ptr = password.as_ptr() as *const i8;
        let code = (self.inner.SetServerPassword)(password_ptr);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    pub fn set_gamemode(&self, gamemode: &str) -> VcmpResult<()> {
        let mut gamemode = encode_to_gbk(gamemode).to_vec();
        gamemode.push(0);
        println!("setting gamemode to {gamemode:?}");
        let gamemode_ptr = gamemode.as_ptr() as *const i8;
        let code = (self.inner.SetGameModeText)(gamemode_ptr);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    pub fn get_gamemode_text(&self) -> String {
        // static char buffer[96];

        let buf = vec![0u8; 96];
        let buf_ptr = buf.as_ptr() as *mut i8;
        let code = (self.inner.GetGameModeText)(buf_ptr, 96);
        println!("{code} - {buf:?}");

        decode_gbk(&(buf
                .iter()
                .map(|v| *v as u8)
                .collect::<Vec<u8>>()),)
    }

    pub fn get_gamemode(&self) -> String {
        let mut len = 96;
        // loop {
        //     let mut buffer = Vec::with_capacity(len);
        //     buffer.clear();
        //     let code = (self.inner.GetGameModeText)(buffer.as_mut_ptr() as *mut i8, len);
        //     println!("{code} - {len} {buffer:?}");
        //     if code == VcmpError::BufferTooSmall.into() {
        //         len *= 2;
        //         continue;
        //     } else if code != 0 {
        //         panic!("GetGameModeText error: {}", code);
        //     } else {
        //         return decode_gbk(&buffer);
        //     }
        // }
        const TARGET_OPCODE: &[u8] = &[0x48, 0x8b, 0x0d, 0x34, 0xfa, 0x0e, 0x00];

        fn find_instruction(func_ptr: usize, max_search: usize) -> Option<usize> {
            unsafe {// qq(?
                for i in 0..max_search {
                    let ptr = (func_ptr + i) as *const u8;
                    if std::slice::from_raw_parts(ptr, TARGET_OPCODE.len()) == TARGET_OPCODE {
                        return Some(func_ptr + i);
                    }
                }
            }
            None
        }
        unsafe {
            let func_ptr = self.inner.GetGameModeText as usize;
            println!("GetGameModeText: {func_ptr:X}");
            let mode_text_ptr = func_ptr + 32;
            let game_mode_text_addr: u64 = u64::from_le(*(mode_text_ptr as *const u64));
            println!("game_mode_text_addr: {game_mode_text_addr:X}");
        }
        unsafe {
            let func_ptr = self.inner.GetGameModeText as usize;

            // 查找目标指令的地址
            let target_addr = match find_instruction(func_ptr, 0x100) {
                Some(addr) => addr,
                None => panic!("Failed to find target instruction"),
            };

            // 计算 RIP 值（下一条指令地址）
            let rip = target_addr + 7; // 当前指令长度为 7 字节

            // 读取 32 位立即数（小端序）
            let offset = std::ptr::read_unaligned((target_addr + 3) as *const u32);
            let offset = i32::from_le(offset as i32);

            // 计算全局变量地址：RIP + offset
            let global_var_address = rip.wrapping_add(offset as usize) + 0x108;

            println!("Global variable address: {global_var_address:X}");

            // 读取全局变量的内容（qword）
            let value = std::ptr::read_unaligned(global_var_address as *const u64);
            println!("Value at address: {value:X}");
        }
        todo!()
    }
}

/*
string getSomethingFromVCMP(
    function<vcmpError(char*, size_t)> func,
    string extra = ""
) {
    vcmpError error = vcmpErrorBufferTooSmall;
    char buffer[256];
    while (error == vcmpErrorBufferTooSmall) {
        error = func(buffer, sizeof(buffer));
        if (error == vcmpErrorNone) {
            string ret = gbk_to_utf8(std::string(buffer));
            // remove ending \0
            if (ret.length() > 0 && ret[ret.length() - 1] == '\0') {
                ret = ret.substr(0, ret.length() - 1);
            }
            return ret;
        }
        buffer = realloc(buffer, sizeof(buffer) * 2);
    }
    throwVCMPError(error, extra);
    return "";
}

*/
