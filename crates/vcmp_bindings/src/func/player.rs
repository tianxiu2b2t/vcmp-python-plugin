use std::ffi::c_void;

use crate::{VcmpError, VcmpResult, func::VcmpFunctions, utils::Color};

pub trait PlayerMethods {
    /// 发送 Stream
    fn send_client_script_data(&self, player_id: i32, data: &[u8]) -> VcmpResult<()>;

    /// 发送消息
    fn send_client_message(&self, player_id: i32, color: Color, message: &str) -> VcmpResult<()>;

    /// 发送公告
    fn send_announce(&self, player_id: i32, announce_type: i32, message: &str) -> VcmpResult<()>;
}

impl PlayerMethods for VcmpFunctions {
    /// 发送 Stream
    fn send_client_script_data(&self, player_id: i32, data: &[u8]) -> VcmpResult<()> {
        let msg_ptr = data.as_ptr() as *const c_void;
        let code = (self.inner.SendClientScriptData)(player_id, msg_ptr, data.len());
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    /// 发送消息
    fn send_client_message(&self, player_id: i32, color: Color, message: &str) -> VcmpResult<()> {
        let color = color.as_rgba();
        let msg_ptr = message.as_ptr() as *const i8;
        let code = (self.inner.SendClientMessage)(player_id, color, msg_ptr);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    /// 发送公告（）
    fn send_announce(&self, player_id: i32, announce_type: i32, message: &str) -> VcmpResult<()> {
        let msg = message.as_bytes();
        let msg_ptr = msg.as_ptr() as *const i8;
        let code = (self.inner.SendGameMessage)(player_id, announce_type, msg_ptr);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
}
