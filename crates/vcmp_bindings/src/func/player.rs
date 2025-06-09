use std::ffi::c_void;

use crate::options::VcmpPlayerOption;
use crate::states::VcmpPlayerState;
use crate::utils::{Color, Vector};
use crate::{VcmpError, VcmpResult, encodes::decode_gbk, func::VcmpFunctions};

pub trait PlayerMethods {
    /// 发送 Stream
    fn send_client_script_data(&self, player_id: i32, data: &[u8]) -> VcmpResult<()>;

    /// 发送消息
    fn send_client_message(&self, player_id: i32, color: Color, message: &str) -> VcmpResult<()>;

    /// 发送公告
    fn send_announce(&self, player_id: i32, announce_type: i32, message: &str) -> VcmpResult<()>;

    fn play_sound_for_player(&self, player_id: i32, sound: i32, position: Option<Vector>);

    /*
        Admins?
    */
    fn is_player_admin(&self, player_id: i32) -> bool;

    fn set_player_admin(&self, player_id: i32, admin: bool);
    fn get_player_ip(&self, player: i32) -> String;
    fn get_player_uid(&self, player: i32) -> String;
    fn get_player_uid2(&self, player: i32) -> String;
    fn kick_player(&self, player_id: i32);
    fn ban_player(&self, player_id: i32);

    fn is_player_connected(&self, player: i32) -> bool;
    fn is_player_streamed_for_target(&self, player: i32, target: i32) -> bool;

    fn get_player_key(&self, player: i32) -> u32;

    fn get_player_name(&self, player: i32) -> String;

    fn set_player_name(&self, player: i32, name: &str);

    fn get_player_state(&self, player: i32) -> VcmpPlayerState;

    fn set_player_option(&self, player: i32, option: VcmpPlayerOption, value: bool);

    fn get_player_option(&self, player: i32, option: VcmpPlayerOption) -> bool;
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

    /*

    */

    fn play_sound_for_player(&self, player_id: i32, sound: i32, position: Option<Vector>) {
        let world = (self.inner.GetPlayerUniqueWorld)(player_id);

        let pos = position.unwrap_or(Vector {
            x: f32::NAN,
            y: f32::NAN,
            z: f32::NAN,
        });
        let (x, y, z) = (pos.x, pos.y, pos.z);
        (self.inner.PlaySound)(world, sound, x, y, z);
    }

    /*
       Admins?
    */

    fn is_player_admin(&self, player_id: i32) -> bool {
        (self.inner.IsPlayerAdmin)(player_id) != 0
    }

    fn set_player_admin(&self, player_id: i32, admin: bool) {
        (self.inner.SetPlayerAdmin)(player_id, admin as u8);
    }

    fn get_player_ip(&self, player: i32) -> String {
        let buf = vec![0u8; 1024];
        let buf_ptr = buf.as_ptr() as *mut i8;
        let _ = (self.inner.GetPlayerIP)(player, buf_ptr, 1024);
        decode_gbk(&buf)
    }
    fn get_player_uid(&self, player: i32) -> String {
        let buf = vec![0u8; 1024];
        let buf_ptr = buf.as_ptr() as *mut i8;
        let _ = (self.inner.GetPlayerUID)(player, buf_ptr, 1024);
        decode_gbk(&buf)
    }
    fn get_player_uid2(&self, player: i32) -> String {
        let buf = vec![0u8; 1024];
        let buf_ptr = buf.as_ptr() as *mut i8;
        let _ = (self.inner.GetPlayerUID2)(player, buf_ptr, 1024);
        decode_gbk(&buf)
    }
    fn kick_player(&self, player: i32) {
        (self.inner.KickPlayer)(player);
    }
    fn ban_player(&self, player: i32) {
        (self.inner.BanPlayer)(player);
    }

    fn is_player_connected(&self, player: i32) -> bool {
        (self.inner.IsPlayerConnected)(player) != 0
    }
    fn is_player_streamed_for_target(&self, player: i32, target: i32) -> bool {
        (self.inner.IsPlayerStreamedForPlayer)(player, target) != 0
    }

    fn get_player_key(&self, player: i32) -> u32 {
        (self.inner.GetPlayerKey)(player)
    }

    fn get_player_name(&self, player: i32) -> String {
        let buf = vec![0u8; 1024];
        let buf_ptr = buf.as_ptr() as *mut i8;
        let _ = (self.inner.GetPlayerUID)(player, buf_ptr, 1024);
        decode_gbk(&buf)
    }

    fn set_player_name(&self, player: i32, name: &str) {
        let name_ptr = name.as_ptr() as *const i8;
        (self.inner.SetPlayerName)(player, name_ptr);
    }

    fn get_player_state(&self, player: i32) -> VcmpPlayerState {
        VcmpPlayerState::from((self.inner.GetPlayerState)(player))
    }

    fn set_player_option(&self, player: i32, option: VcmpPlayerOption, value: bool) {
        (self.inner.SetPlayerOption)(player, option.into(), value as u8);
    }

    fn get_player_option(&self, player: i32, option: VcmpPlayerOption) -> bool {
        (self.inner.GetPlayerOption)(player, option.into()) != 0
    }
}
