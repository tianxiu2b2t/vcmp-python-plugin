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
        let name = encode_to_gbk(name);
        let name_ptr = name.as_ptr() as *const i8;
        let code = (self.inner.SetServerName)(name_ptr);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    pub fn get_server_name(&self) -> String {
        self.get_server_settings().server_name()
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

    pub fn get_server_password(&self) -> String {
        let buf = vec![0u8; 1024];
        let buf_ptr = buf.as_ptr() as *mut i8;
        let _ = (self.inner.GetServerPassword)(buf_ptr, 1024);
        decode_gbk(&(buf.iter().map(|v| *v as u8).collect::<Vec<u8>>()))
    }

    pub fn set_gamemode(&self, gamemode: &str) -> VcmpResult<()> {
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

    pub fn get_gamemode(&self) -> String {
        let buf = vec![0u8; 1024];
        let buf_ptr = buf.as_ptr() as *mut i8;
        let _ = (self.inner.GetGameModeText)(buf_ptr, 1024);
        decode_gbk(&(buf.iter().map(|v| *v as u8).collect::<Vec<u8>>()))
    }
    //

    pub fn max_players(&self) -> u32 {
        (self.inner.GetMaxPlayers)()
    }

    pub fn set_max_players(&self, max_player: u32) -> VcmpResult<()> {
        let code = (self.inner.SetMaxPlayers)(max_player);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    pub fn shutdown(&self) {
        (self.inner.ShutdownServer)()
    }

    /*
    环境设置
     */

    /*
    TODO: SetServerOption

     */

    pub fn get_world_bounds(&self) -> (f32, f32, f32, f32) {
        let (mut max_x, mut min_x, mut max_y, mut min_y) = (0f32, 0f32, 0f32, 0f32);
        (self.inner.GetWorldBounds)(&mut max_x, &mut min_x, &mut max_y, &mut min_y);
        (max_x, min_x, max_y, min_y)
    }

    pub fn set_world_bounds(&self, max_x: f32, min_x: f32, max_y: f32, min_y: f32) {
        (self.inner.SetWorldBounds)(max_x, min_x, max_y, min_y);
    }

    pub fn set_wasted_settings(
        &self,
        death_timer: u32,
        fade_timer: u32,
        fade_in_speed: f32,
        fade_out_speed: f32,
        color: impl Into<u32>,
        corpse_fade_start: u32,
        corpse_fade_time: u32,
    ) {
        (self.inner.SetWastedSettings)(
            death_timer,
            fade_timer,
            fade_in_speed,
            fade_out_speed,
            color.into(),
            corpse_fade_start,
            corpse_fade_time,
        )
    }

    pub fn get_wasted_settings(&self) -> (u32, u32, f32, f32, u32, u32, u32) {
        let (
            mut death_timer,
            mut fade_timer,
            mut fade_in_speed,
            mut fade_out_speed,
            mut color,
            mut corpse_fade_start,
            mut corpse_fade_time,
        ) = (0, 0, 0f32, 0f32, 0, 0, 0);
        (self.inner.GetWastedSettings)(
            &mut death_timer,
            &mut fade_timer,
            &mut fade_in_speed,
            &mut fade_out_speed,
            &mut color,
            &mut corpse_fade_start,
            &mut corpse_fade_time,
        );

        (
            death_timer,
            fade_timer,
            fade_in_speed,
            fade_out_speed,
            color,
            corpse_fade_start,
            corpse_fade_time,
        )
    }

    pub fn set_time_rate(&self, rate: i32) {
        (self.inner.SetTimeRate)(rate);
    }

    pub fn get_time_rate(&self) -> i32 {
        (self.inner.GetTimeRate)()
    }

    pub fn get_time(&self) -> (i32, i32) {
        ((self.inner.GetHour)(), (self.inner.GetMinute)())
    }

    pub fn set_hour(&self, hour: i32) {
        (self.inner.SetHour)(hour);
    }

    pub fn set_minute(&self, minute: i32) {
        (self.inner.SetMinute)(minute);
    }

    pub fn get_water_level(&self) -> f32 {
        (self.inner.GetWaterLevel)()
    }

    pub fn set_water_level(&self, level: f32) {
        (self.inner.SetWaterLevel)(level);
    }

    pub fn get_weather(&self) -> i32 {
        (self.inner.GetWeather)()
    }

    pub fn set_weather(&self, weather: i32) {
        (self.inner.SetWeather)(weather);
    }

    pub fn get_gravity(&self) -> f32 {
        (self.inner.GetGravity)()
    }

    pub fn set_gravity(&self, gravity: f32) {
        (self.inner.SetGravity)(gravity);
    }

    pub fn set_gamespeed(&self, gamespeed: f32) {
        (self.inner.SetGameSpeed)(gamespeed);
    }

    pub fn get_gamespeed(&self) -> f32 {
        (self.inner.GetGameSpeed)()
    }

    pub fn set_maximum_flight_altitude(&self, height: f32) {
        (self.inner.SetMaximumFlightAltitude)(height);
    }

    pub fn get_maximum_flight_altitude(&self) -> f32 {
        (self.inner.GetMaximumFlightAltitude)()
    }

    pub fn set_kill_command_delay(&self, delay: i32) {
        (self.inner.SetKillCommandDelay)(delay);
    }

    pub fn get_kill_command_delay(&self) -> i32 {
        (self.inner.GetKillCommandDelay)()
    }

    pub fn disable_kill_command_delay(&self) {
        self.set_kill_command_delay(2147483647);
    }

    pub fn set_vehicles_forced_respawn_height(&self, height: f32) {
        (self.inner.SetVehiclesForcedRespawnHeight)(height);
    }

    pub fn get_vehicles_forced_respawn_height(&self) -> f32 {
        (self.inner.GetVehiclesForcedRespawnHeight)()
    }

    /*
       misc
    */

    pub fn create_explosion(
        &self,
        world: i32,
        explosion_type: i32,
        pos: (f32, f32, f32),
        responsible_player: i32,
        on_ground: bool,
    ) {
        (self.inner.CreateExplosion)(
            world,
            explosion_type,
            pos.0,
            pos.1,
            pos.2,
            responsible_player,
            on_ground as u8,
        );
    }

    pub fn play_sound(&self, world: i32, sound: i32, x: f32, y: f32, z: f32) {
        (self.inner.PlaySound)(world, sound, x, y, z);
    }

    pub fn play_sound_for_player(&self, player_id: i32, sound: i32, pos: Option<(f32, f32, f32)>) {
        let world = (self.inner.GetPlayerUniqueWorld)(player_id);

        let (x, y, z) = pos.unwrap_or((f32::NAN, f32::NAN, f32::NAN));
        (self.inner.PlaySound)(world, sound, x, y, z);
    }

    pub fn hide_map_object(&self, object_id: i32, pos: (f32, f32, f32)) {
        let (x, y, z) = pos;
        let x = ((x * 10.0).floor() + 0.5) as i16;
        let y = ((y * 10.0).floor() + 0.5) as i16;
        let z = ((z * 10.0).floor() + 0.5) as i16;

        (self.inner.HideMapObject)(object_id, x, y, z);
    }

    pub fn show_map_object(&self, object_id: i32, pos: (f32, f32, f32)) {
        let (x, y, z) = pos;
        let x = ((x * 10.0).floor() + 0.5) as i16;
        let y = ((y * 10.0).floor() + 0.5) as i16;
        let z = ((z * 10.0).floor() + 0.5) as i16;

        (self.inner.ShowMapObject)(object_id, x, y, z);
    }

    pub fn show_all_map_objects(&self) {
        (self.inner.ShowAllMapObjects)();
    }

    /*
       Weapon

    */
}
