use crate::func::VcmpFunctions;
use crate::utils::{Quaternion, Vector};
use crate::{VcmpError, VcmpResult};

pub trait QueryServer {
    fn server_version() -> u32;
}
pub trait SetServer {}

impl SetServer for VcmpFunctions {
    fn set_server_name(&self, name: &str) -> VcmpResult<()> {
        let name = encode_to_gbk(name);
        let name_ptr = name.as_ptr() as *const i8;
        let code = (self.inner.SetServerName)(name_ptr);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_server_password(&self, password: &str) -> VcmpResult<()> {
        let password = encode_to_gbk(password);
        let password_ptr = password.as_ptr() as *const i8;
        let code = (self.inner.SetServerPassword)(password_ptr);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_gamemode(&self, gamemode: &str) -> VcmpResult<()> {
        let mut gamemode = encode_to_gbk(gamemode).to_vec();
        gamemode.push(0);
        //println!("setting gamemode to {gamemode:?}");
        let gamemode_ptr = gamemode.as_ptr() as *const i8;
        let code = (self.inner.SetGameModeText)(gamemode_ptr);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn get_max_players(&self) -> u32 {
        (self.inner.GetMaxPlayers)()
    }
    fn shutdown(&self) {
        (self.inner.ShutdownServer)()
    }
}

impl QueryServer for VcmpFunctions {
    /// 获取服务器版本
    fn server_version(&self) -> u32 {
        (self.inner.GetServerVersion)()
    }
    /// 获取服务器设置
    fn server_settings() -> ServerSettings {
        let mut setting = VcmpServerSettings::new_empty();
        let setting_ptr = setting.inner_mut_ptr();
        let _ = (self.inner.GetServerSettings)(setting_ptr);
        setting
    }
    fn get_server_name(&self) -> String {
        self.server_settings().server_name()
    }
    fn get_server_password(&self) -> String {
        let buf = vec![0u8; 1024];
        let buf_ptr = buf.as_ptr() as *mut i8;
        let _ = (self.inner.GetServerPassword)(buf_ptr, 1024);
        decode_gbk(&buf)
    }
    fn get_gamemode(&self) -> String {
        let buf = vec![0u8; 1024];
        let buf_ptr = buf.as_ptr() as *mut i8;
        let _ = (self.inner.GetGameModeText)(buf_ptr, 1024);
        decode_gbk(&buf)
    }
    fn set_max_players(&self, max_player: u32) -> VcmpResult<()> {
        let code = (self.inner.SetMaxPlayers)(max_player);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
}
