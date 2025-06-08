
use crate::options::{VcmpNetworkStatisticsQueryOption, VcmpServerOption};
use crate::{VcmpError, raw::PluginFuncs};

use crate::encodes::{decode_gbk, encode_to_gbk};

pub mod environment;
pub mod network;
pub mod vehicle;
pub mod keybind;
pub mod marker;
pub mod misc;
pub mod server;
pub mod weapon;
pub mod template;
pub mod player;
pub mod plugin;

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

#[allow(clippy::not_unsafe_ptr_arg_deref)]
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

    /// 全局性的 last error
    pub fn get_last_error(&self) -> VcmpError {
        VcmpError::from((self.inner.GetLastError)())
    }

    pub fn log_message(&self, message: &str) {
        let msg = encode_to_gbk(message);
        let msg_ptr = msg.as_ptr() as *const i8;
        let _ = (self.inner.LogMessage)(msg_ptr);
    }



    pub fn play_sound_for_player(&self, player_id: i32, sound: i32, pos: Option<(f32, f32, f32)>) {
        let world = (self.inner.GetPlayerUniqueWorld)(player_id);

        let (x, y, z) = pos.unwrap_or((f32::NAN, f32::NAN, f32::NAN));
        (self.inner.PlaySound)(world, sound, x, y, z);
    }


    /*
       radios
    */

    pub fn add_player_class(
        &self,
        team: i32,
        color: impl Into<u32>,
        skin: i32,
        pos: (f32, f32, f32),
        angle: f32,
        weapon: Option<(i32, i32)>,
        weapon1: Option<(i32, i32)>,
        weapon2: Option<(i32, i32)>,
    ) -> i32 {
        let (x, y, z) = pos;
        let (wep1, ammo1) = weapon.unwrap_or((0, 0));
        let (wep2, ammo2) = weapon1.unwrap_or((0, 0));
        let (wep3, ammo3) = weapon2.unwrap_or((0, 0));
        (self.inner.AddPlayerClass)(
            team,
            color.into(),
            skin,
            x,
            y,
            z,
            angle,
            wep1,
            ammo1,
            wep2,
            ammo2,
            wep3,
            ammo3,
        )
    }

    pub fn set_spawn_player_position(&self, pos: (f32, f32, f32)) {
        let (x, y, z) = pos;
        (self.inner.SetSpawnPlayerPosition)(x, y, z);
    }

    pub fn set_spawn_camera_position(&self, pos: (f32, f32, f32)) {
        let (x, y, z) = pos;
        (self.inner.SetSpawnCameraPosition)(x, y, z);
    }

    pub fn set_spawn_camera_look_at(&self, pos: (f32, f32, f32)) {
        let (x, y, z) = pos;
        (self.inner.SetSpawnCameraLookAt)(x, y, z);
    }

    /*
       Administration
    */
    pub fn is_player_admin(&self, player: i32) -> bool {
        (self.inner.IsPlayerAdmin)(player) != 0
    }

    pub fn set_player_admin(&self, player: i32, toggle: bool) {
        (self.inner.SetPlayerAdmin)(player, toggle as u8);
    }

    pub fn get_player_ip(&self, player: i32) -> String {
        let buf = vec![0u8; 1024];
        let buf_ptr = buf.as_ptr() as *mut i8;
        let _ = (self.inner.GetPlayerIP)(player, buf_ptr, 1024);
        decode_gbk(&buf)
    }

    pub fn get_player_uid(&self, player: i32) -> String {
        let buf = vec![0u8; 1024];
        let buf_ptr = buf.as_ptr() as *mut i8;
        let _ = (self.inner.GetPlayerUID)(player, buf_ptr, 1024);
        decode_gbk(&buf)
    }

    pub fn get_player_uid2(&self, player: i32) -> String {
        let buf = vec![0u8; 1024];
        let buf_ptr = buf.as_ptr() as *mut i8;
        let _ = (self.inner.GetPlayerUID2)(player, buf_ptr, 1024);
        decode_gbk(&buf)
    }
    pub fn kick_player(&self, player: i32) {
        (self.inner.KickPlayer)(player);
    }

    pub fn ban_player(&self, player: i32) {
        (self.inner.BanPlayer)(player);
    }

    pub fn ban_ip(&self, ip: &str) {
        (self.inner.BanIP)(ip.as_ptr() as *mut i8);
    }

    pub fn unban_ip(&self, ip: &str) -> bool {
        (self.inner.UnbanIP)(ip.as_ptr() as *mut i8) != 0
    }


    /// 获取网络统计信息
    #[inline]
    pub fn get_network_statistics(
        &self,
        player_id: i32,
        option: VcmpNetworkStatisticsQueryOption,
    ) -> f64 {
        (self.inner.GetNetworkStatistics)(player_id, option.into())
    }

    /// 获取指定的服务器设置
    ///
    /// 需要啥直接取
    #[inline]
    pub fn get_server_option(&self, option: VcmpServerOption) -> bool {
        (self.inner.GetServerOption)(option.into()) != 0
    }

    /// 设置指定的服务器设置
    #[inline]
    pub fn set_server_option(&self, option: VcmpServerOption, toggle: bool) {
        let _ = (self.inner.SetServerOption)(option.into(), toggle as u8);
    }
}
