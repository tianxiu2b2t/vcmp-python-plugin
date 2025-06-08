use std::ffi::c_void;

use crate::options::{VcmpNetworkStatisticsQueryOption, VcmpServerOption};
use crate::{VcmpError, VcmpPluginInfo, VcmpResult, raw::PluginFuncs, setting::VcmpServerSettings};

use crate::encodes::{decode_gbk, encode_to_gbk};

pub mod environment;
pub mod network;
pub mod vehicle;

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

    /*
        /// 检查 IP 是否被封禁
        pub fn is_ip_banned(&self, ip: &str) -> bool {
            (self.inner.IsIPBanned)(ip.as_ptr() as *mut i8) != 0
        }

        /// 根据玩家名称获取玩家 ID
        pub fn get_player_id_from_name(&self, name: &str) -> i32 {
            (self.inner.GetPlayerIdFromName)(name.as_ptr() as *mut i8)
        }

        /// 检查玩家是否在线
        pub fn is_player_connected(&self, player_id: i32) -> bool {
            (self.inner.IsPlayerConnected)(player_id) != 0
        }

        /// 检查玩家是否对某个玩家可见
        pub fn is_player_streamed_for_player(&self, checked_player: i32, player: i32) -> bool {
            (self.inner.IsPlayerStreamedForPlayer)(checked_player, player) != 0
        }

        /// 获取玩家按键状态
        pub fn get_player_key(&self, player_id: i32) -> u32 {
            (self.inner.GetPlayerKey)(player_id)
        }

        /// 获取玩家名称
        pub fn get_player_name(&self, player_id: i32) -> Result<String, VcmpError> {
            let mut buffer = vec![0i8; 1024];
            let code = (self.inner.GetPlayerName)(player_id, buffer.as_mut_ptr(), buffer.len());

            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                let c_str = unsafe { std::ffi::CStr::from_ptr(buffer.as_ptr()) };
                let name = decode_gbk(c_str.to_bytes());
                Ok(name)
            }
        }

        /// 设置玩家名称
        pub fn set_player_name(&self, player_id: i32, name: &str) -> VcmpResult<()> {
            let name = encode_to_gbk(name);
            let name_ptr = name.as_ptr() as *const i8;
            let code = (self.inner.SetPlayerName)(player_id, name_ptr);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家状态
        pub fn get_player_state(&self, player_id: i32) -> i32 {
            (self.inner.GetPlayerState)(player_id)
        }

        /// 设置玩家选项
        pub fn set_player_option(&self, player_id: i32, option: i32, toggle: bool) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerOption)(player_id, option, toggle as u8);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家选项
        pub fn get_player_option(&self, player_id: i32, option: i32) -> bool {
            (self.inner.GetPlayerOption)(player_id, option) != 0
        }

        /// 设置玩家世界
        pub fn set_player_world(&self, player_id: i32, world: i32) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerWorld)(player_id, world);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家世界
        pub fn get_player_world(&self, player_id: i32) -> i32 {
            (self.inner.GetPlayerWorld)(player_id)
        }

        /// 设置玩家次级世界
        pub fn set_player_secondary_world(
            &self,
            player_id: i32,
            secondary_world: i32,
        ) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerSecondaryWorld)(player_id, secondary_world);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家次级世界
        pub fn get_player_secondary_world(&self, player_id: i32) -> i32 {
            (self.inner.GetPlayerSecondaryWorld)(player_id)
        }

        /// 获取玩家唯一世界
        pub fn get_player_unique_world(&self, player_id: i32) -> i32 {
            (self.inner.GetPlayerUniqueWorld)(player_id)
        }

        /// 检查玩家世界是否兼容
        pub fn is_player_world_compatible(&self, player_id: i32, world: i32) -> bool {
            (self.inner.IsPlayerWorldCompatible)(player_id, world) != 0
        }

        /// 获取玩家玩家类
        pub fn get_player_class(&self, player_id: i32) -> i32 {
            (self.inner.GetPlayerClass)(player_id)
        }

        /// 设置玩家团队
        pub fn set_player_team(&self, player_id: i32, team_id: i32) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerTeam)(player_id, team_id);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家团队
        pub fn get_player_team(&self, player_id: i32) -> i32 {
            (self.inner.GetPlayerTeam)(player_id)
        }

        /// 设置玩家皮肤
        pub fn set_player_skin(&self, player_id: i32, skin_id: i32) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerSkin)(player_id, skin_id);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家皮肤
        pub fn get_player_skin(&self, player_id: i32) -> i32 {
            (self.inner.GetPlayerSkin)(player_id)
        }

        /// 设置玩家颜色
        pub fn set_player_colour(&self, player_id: i32, colour: u32) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerColour)(player_id, colour);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家颜色
        pub fn get_player_colour(&self, player_id: i32) -> u32 {
            (self.inner.GetPlayerColour)(player_id)
        }

        /// 检查玩家是否已生成
        pub fn is_player_spawned(&self, player_id: i32) -> bool {
            (self.inner.IsPlayerSpawned)(player_id) != 0
        }

        /// 强制玩家生成
        pub fn force_player_spawn(&self, player_id: i32) -> VcmpResult<()> {
            let code = (self.inner.ForcePlayerSpawn)(player_id);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 强制玩家选择
        pub fn force_player_select(&self, player_id: i32) -> VcmpResult<()> {
            let code = (self.inner.ForcePlayerSelect)(player_id);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 强制所有玩家选择
        pub fn force_all_select(&self) {
            (self.inner.ForceAllSelect)();
        }

        /// 检查玩家是否正在输入
        pub fn is_player_typing(&self, player_id: i32) -> bool {
            (self.inner.IsPlayerTyping)(player_id) != 0
        }

        /// 给予玩家金钱
        pub fn give_player_money(&self, player_id: i32, amount: i32) -> VcmpResult<()> {
            let code = (self.inner.GivePlayerMoney)(player_id, amount);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 设置玩家金钱
        pub fn set_player_money(&self, player_id: i32, amount: i32) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerMoney)(player_id, amount);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家金钱
        pub fn get_player_money(&self, player_id: i32) -> i32 {
            (self.inner.GetPlayerMoney)(player_id)
        }

        /// 设置玩家分数
        pub fn set_player_score(&self, player_id: i32, score: i32) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerScore)(player_id, score);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家分数
        pub fn get_player_score(&self, player_id: i32) -> i32 {
            (self.inner.GetPlayerScore)(player_id)
        }

        /// 设置玩家通缉级别
        pub fn set_player_wanted_level(&self, player_id: i32, level: i32) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerWantedLevel)(player_id, level);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家通缉级别
        pub fn get_player_wanted_level(&self, player_id: i32) -> i32 {
            (self.inner.GetPlayerWantedLevel)(player_id)
        }

        /// 获取玩家延迟
        pub fn get_player_ping(&self, player_id: i32) -> i32 {
            (self.inner.GetPlayerPing)(player_id)
        }

        /// 获取玩家 FPS
        pub fn get_player_fps(&self, player_id: i32) -> f64 {
            (self.inner.GetPlayerFPS)(player_id)
        }

        /// 设置玩家生命值
        pub fn set_player_health(&self, player_id: i32, health: f32) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerHealth)(player_id, health);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家生命值
        pub fn get_player_health(&self, player_id: i32) -> f32 {
            (self.inner.GetPlayerHealth)(player_id)
        }

        /// 设置玩家护甲值
        pub fn set_player_armour(&self, player_id: i32, armour: f32) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerArmour)(player_id, armour);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家护甲值
        pub fn get_player_armour(&self, player_id: i32) -> f32 {
            (self.inner.GetPlayerArmour)(player_id)
        }

        /// 设置玩家免疫标志
        pub fn set_player_immunity_flags(&self, player_id: i32, flags: u32) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerImmunityFlags)(player_id, flags);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家免疫标志
        pub fn get_player_immunity_flags(&self, player_id: i32) -> u32 {
            (self.inner.GetPlayerImmunityFlags)(player_id)
        }

        /// 设置玩家位置
        pub fn set_player_position(&self, player_id: i32, x: f32, y: f32, z: f32) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerPosition)(player_id, x, y, z);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家位置
        pub fn get_player_position(&self, player_id: i32) -> Result<(f32, f32, f32), VcmpError> {
            let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
            let code = (self.inner.GetPlayerPosition)(player_id, &mut x, &mut y, &mut z);

            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok((x, y, z))
            }
        }

        /// 设置玩家速度
        pub fn set_player_speed(&self, player_id: i32, x: f32, y: f32, z: f32) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerSpeed)(player_id, x, y, z);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家速度
        pub fn get_player_speed(&self, player_id: i32) -> Result<(f32, f32, f32), VcmpError> {
            let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
            let code = (self.inner.GetPlayerSpeed)(player_id, &mut x, &mut y, &mut z);

            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok((x, y, z))
            }
        }

        /// 添加玩家速度
        pub fn add_player_speed(&self, player_id: i32, x: f32, y: f32, z: f32) -> VcmpResult<()> {
            let code = (self.inner.AddPlayerSpeed)(player_id, x, y, z);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 设置玩家朝向
        pub fn set_player_heading(&self, player_id: i32, angle: f32) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerHeading)(player_id, angle);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家朝向
        pub fn get_player_heading(&self, player_id: i32) -> f32 {
            (self.inner.GetPlayerHeading)(player_id)
        }

        /// 设置玩家透明度
        pub fn set_player_alpha(&self, player_id: i32, alpha: i32, fade_time: u32) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerAlpha)(player_id, alpha, fade_time);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家透明度
        pub fn get_player_alpha(&self, player_id: i32) -> i32 {
            (self.inner.GetPlayerAlpha)(player_id)
        }

        /// 获取玩家瞄准位置
        pub fn get_player_aim_position(&self, player_id: i32) -> Result<(f32, f32, f32), VcmpError> {
            let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
            let code = (self.inner.GetPlayerAimPosition)(player_id, &mut x, &mut y, &mut z);

            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok((x, y, z))
            }
        }

        /// 获取玩家瞄准方向
        pub fn get_player_aim_direction(&self, player_id: i32) -> Result<(f32, f32, f32), VcmpError> {
            let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
            let code = (self.inner.GetPlayerAimDirection)(player_id, &mut x, &mut y, &mut z);

            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok((x, y, z))
            }
        }

        /// 检查玩家是否着火
        pub fn is_player_on_fire(&self, player_id: i32) -> bool {
            (self.inner.IsPlayerOnFire)(player_id) != 0
        }

        /// 检查玩家是否蹲下
        pub fn is_player_crouching(&self, player_id: i32) -> bool {
            (self.inner.IsPlayerCrouching)(player_id) != 0
        }

        /// 获取玩家动作
        pub fn get_player_action(&self, player_id: i32) -> i32 {
            (self.inner.GetPlayerAction)(player_id)
        }

        /// 获取玩家游戏按键状态
        pub fn get_player_game_keys(&self, player_id: i32) -> u32 {
            (self.inner.GetPlayerGameKeys)(player_id)
        }

        /// 将玩家放入车辆
        pub fn put_player_in_vehicle(
            &self,
            player_id: i32,
            vehicle_id: i32,
            slot_index: i32,
            make_room: bool,
            warp: bool,
        ) -> VcmpResult<()> {
            let code = (self.inner.PutPlayerInVehicle)(
                player_id,
                vehicle_id,
                slot_index,
                make_room as u8,
                warp as u8,
            );
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 将玩家从车辆中移除
        pub fn remove_player_from_vehicle(&self, player_id: i32) -> VcmpResult<()> {
            let code = (self.inner.RemovePlayerFromVehicle)(player_id);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家在车辆中的状态
        pub fn get_player_in_vehicle_status(&self, player_id: i32) -> i32 {
            (self.inner.GetPlayerInVehicleStatus)(player_id)
        }

        /// 获取玩家在车辆中的座位
        pub fn get_player_in_vehicle_slot(&self, player_id: i32) -> i32 {
            (self.inner.GetPlayerInVehicleSlot)(player_id)
        }

        /// 获取玩家所在车辆 ID
        pub fn get_player_vehicle_id(&self, player_id: i32) -> i32 {
            (self.inner.GetPlayerVehicleId)(player_id)
        }

        /// 给予玩家武器
        pub fn give_player_weapon(&self, player_id: i32, weapon_id: i32, ammo: i32) -> VcmpResult<()> {
            let code = (self.inner.GivePlayerWeapon)(player_id, weapon_id, ammo);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 设置玩家武器
        pub fn set_player_weapon(&self, player_id: i32, weapon_id: i32, ammo: i32) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerWeapon)(player_id, weapon_id, ammo);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家武器
        pub fn get_player_weapon(&self, player_id: i32) -> i32 {
            (self.inner.GetPlayerWeapon)(player_id)
        }

        /// 获取玩家武器弹药
        pub fn get_player_weapon_ammo(&self, player_id: i32) -> i32 {
            (self.inner.GetPlayerWeaponAmmo)(player_id)
        }

        /// 设置玩家武器槽
        pub fn set_player_weapon_slot(&self, player_id: i32, slot: i32) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerWeaponSlot)(player_id, slot);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家武器槽
        pub fn get_player_weapon_slot(&self, player_id: i32) -> i32 {
            (self.inner.GetPlayerWeaponSlot)(player_id)
        }

        /// 获取玩家指定槽位的武器
        pub fn get_player_weapon_at_slot(&self, player_id: i32, slot: i32) -> i32 {
            (self.inner.GetPlayerWeaponAtSlot)(player_id, slot)
        }

        /// 获取玩家指定槽位的弹药
        pub fn get_player_ammo_at_slot(&self, player_id: i32, slot: i32) -> i32 {
            (self.inner.GetPlayerAmmoAtSlot)(player_id, slot)
        }

        /// 移除玩家武器
        pub fn remove_player_weapon(&self, player_id: i32, weapon_id: i32) -> VcmpResult<()> {
            let code = (self.inner.RemovePlayerWeapon)(player_id, weapon_id);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 移除玩家所有武器
        pub fn remove_all_weapons(&self, player_id: i32) -> VcmpResult<()> {
            let code = (self.inner.RemoveAllWeapons)(player_id);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 设置摄像机位置
        pub fn set_camera_position(
            &self,
            player_id: i32,
            pos_x: f32,
            pos_y: f32,
            pos_z: f32,
            look_x: f32,
            look_y: f32,
            look_z: f32,
        ) -> VcmpResult<()> {
            let code =
                (self.inner.SetCameraPosition)(player_id, pos_x, pos_y, pos_z, look_x, look_y, look_z);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 恢复摄像机
        pub fn restore_camera(&self, player_id: i32) -> VcmpResult<()> {
            let code = (self.inner.RestoreCamera)(player_id);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 检查摄像机是否锁定
        pub fn is_camera_locked(&self, player_id: i32) -> bool {
            (self.inner.IsCameraLocked)(player_id) != 0
        }

        /// 设置玩家动画
        pub fn set_player_animation(
            &self,
            player_id: i32,
            group_id: i32,
            animation_id: i32,
        ) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerAnimation)(player_id, group_id, animation_id);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家站立的车辆
        pub fn get_player_standing_on_vehicle(&self, player_id: i32) -> i32 {
            (self.inner.GetPlayerStandingOnVehicle)(player_id)
        }

        /// 获取玩家站立的对象
        pub fn get_player_standing_on_object(&self, player_id: i32) -> i32 {
            (self.inner.GetPlayerStandingOnObject)(player_id)
        }

        /// 检查玩家是否离开
        pub fn is_player_away(&self, player_id: i32) -> bool {
            (self.inner.IsPlayerAway)(player_id) != 0
        }

        /// 获取玩家观察目标
        pub fn get_player_spectate_target(&self, player_id: i32) -> i32 {
            (self.inner.GetPlayerSpectateTarget)(player_id)
        }

        /// 设置玩家观察目标
        pub fn set_player_spectate_target(&self, player_id: i32, target_id: i32) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerSpectateTarget)(player_id, target_id);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 重定向玩家到服务器
        pub fn redirect_player_to_server(
            &self,
            player_id: i32,
            ip: &str,
            port: u32,
            nick: &str,
            server_password: &str,
            user_password: &str,
        ) -> VcmpResult<()> {
            let code = (self.inner.RedirectPlayerToServer)(
                player_id,
                ip.as_ptr() as *mut i8,
                port,
                nick.as_ptr() as *mut i8,
                server_password.as_ptr() as *mut i8,
                user_password.as_ptr() as *mut i8,
            );
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 检查实体是否存在
        pub fn check_entity_exists(&self, entity_pool: i32, index: i32) -> bool {
            (self.inner.CheckEntityExists)(entity_pool, index) != 0
        }

        /*
         * 拾取物相关
         */

        /// 创建拾取物
        pub fn create_pickup(
            &self,
            model_index: i32,
            world: i32,
            quantity: i32,
            x: f32,
            y: f32,
            z: f32,
            alpha: i32,
            is_automatic: bool,
        ) -> i32 {
            (self.inner.CreatePickup)(
                model_index,
                world,
                quantity,
                x,
                y,
                z,
                alpha,
                is_automatic as u8,
            )
        }

        /// 删除拾取物
        pub fn delete_pickup(&self, pickup_id: i32) -> VcmpError {
            let code = (self.inner.DeletePickup)(pickup_id);
            VcmpError::from(code)
        }

        /// 检查拾取物是否对玩家可见
        pub fn is_pickup_streamed_for_player(&self, pickup_id: i32, player_id: i32) -> bool {
            (self.inner.IsPickupStreamedForPlayer)(pickup_id, player_id) != 0
        }

        /// 设置拾取物世界
        pub fn set_pickup_world(&self, pickup_id: i32, world: i32) -> VcmpError {
            let code = (self.inner.SetPickupWorld)(pickup_id, world);
            VcmpError::from(code)
        }

        /// 获取拾取物世界
        pub fn get_pickup_world(&self, pickup_id: i32) -> i32 {
            (self.inner.GetPickupWorld)(pickup_id)
        }

        /// 设置拾取物透明度
        pub fn set_pickup_alpha(&self, pickup_id: i32, alpha: i32) -> VcmpError {
            let code = (self.inner.SetPickupAlpha)(pickup_id, alpha);
            VcmpError::from(code)
        }

        /// 获取拾取物透明度
        pub fn get_pickup_alpha(&self, pickup_id: i32) -> i32 {
            (self.inner.GetPickupAlpha)(pickup_id)
        }

        /// 设置拾取物是否自动刷新
        pub fn set_pickup_is_automatic(&self, pickup_id: i32, toggle: bool) -> VcmpError {
            let code = (self.inner.SetPickupIsAutomatic)(pickup_id, toggle as u8);
            VcmpError::from(code)
        }

        /// 检查拾取物是否自动刷新
        pub fn is_pickup_automatic(&self, pickup_id: i32) -> bool {
            (self.inner.IsPickupAutomatic)(pickup_id) != 0
        }

        /// 设置拾取物自动刷新计时器
        pub fn set_pickup_auto_timer(&self, pickup_id: i32, duration_millis: u32) -> VcmpError {
            let code = (self.inner.SetPickupAutoTimer)(pickup_id, duration_millis);
            VcmpError::from(code)
        }

        /// 获取拾取物自动刷新计时器
        pub fn get_pickup_auto_timer(&self, pickup_id: i32) -> u32 {
            (self.inner.GetPickupAutoTimer)(pickup_id)
        }

        /// 刷新拾取物
        pub fn refresh_pickup(&self, pickup_id: i32) -> VcmpError {
            let code = (self.inner.RefreshPickup)(pickup_id);
            VcmpError::from(code)
        }

        /// 设置拾取物位置
        pub fn set_pickup_position(&self, pickup_id: i32, x: f32, y: f32, z: f32) -> VcmpError {
            let code = (self.inner.SetPickupPosition)(pickup_id, x, y, z);
            VcmpError::from(code)
        }

        /// 获取拾取物位置
        pub fn get_pickup_position(&self, pickup_id: i32) -> Result<(f32, f32, f32), VcmpError> {
            let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
            let code = (self.inner.GetPickupPosition)(pickup_id, &mut x, &mut y, &mut z);

            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok((x, y, z))
            }
        }

        /// 获取拾取物模型
        pub fn get_pickup_model(&self, pickup_id: i32) -> i32 {
            (self.inner.GetPickupModel)(pickup_id)
        }

        /// 获取拾取物数量
        pub fn get_pickup_quantity(&self, pickup_id: i32) -> i32 {
            (self.inner.GetPickupQuantity)(pickup_id)
        }

        /*
         * 检查点相关
         */

        /// 创建检查点
        pub fn create_check_point(
            &self,
            player_id: i32,
            world: i32,
            is_sphere: bool,
            x: f32,
            y: f32,
            z: f32,
            red: i32,
            green: i32,
            blue: i32,
            alpha: i32,
            radius: f32,
        ) -> i32 {
            (self.inner.CreateCheckPoint)(
                player_id,
                world,
                is_sphere as u8,
                x,
                y,
                z,
                red,
                green,
                blue,
                alpha,
                radius,
            )
        }

        /// 删除检查点
        pub fn delete_check_point(&self, check_point_id: i32) -> VcmpError {
            let code = (self.inner.DeleteCheckPoint)(check_point_id);
            VcmpError::from(code)
        }

        /// 检查检查点是否对玩家可见
        pub fn is_check_point_streamed_for_player(&self, check_point_id: i32, player_id: i32) -> bool {
            (self.inner.IsCheckPointStreamedForPlayer)(check_point_id, player_id) != 0
        }

        /// 检查检查点是否为球形
        pub fn is_check_point_sphere(&self, check_point_id: i32) -> bool {
            (self.inner.IsCheckPointSphere)(check_point_id) != 0
        }

        /// 设置检查点世界
        pub fn set_check_point_world(&self, check_point_id: i32, world: i32) -> VcmpError {
            let code = (self.inner.SetCheckPointWorld)(check_point_id, world);
            VcmpError::from(code)
        }

        /// 获取检查点世界
        pub fn get_check_point_world(&self, check_point_id: i32) -> i32 {
            (self.inner.GetCheckPointWorld)(check_point_id)
        }

        /// 设置检查点颜色
        pub fn set_check_point_colour(
            &self,
            check_point_id: i32,
            red: i32,
            green: i32,
            blue: i32,
            alpha: i32,
        ) -> VcmpError {
            let code = (self.inner.SetCheckPointColour)(check_point_id, red, green, blue, alpha);
            VcmpError::from(code)
        }

        /// 获取检查点颜色
        pub fn get_check_point_colour(
            &self,
            check_point_id: i32,
        ) -> Result<(i32, i32, i32, i32), VcmpError> {
            let (mut r, mut g, mut b, mut a) = (0, 0, 0, 0);
            let code = (self.inner.GetCheckPointColour)(check_point_id, &mut r, &mut g, &mut b, &mut a);

            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok((r, g, b, a))
            }
        }

        /// 设置检查点位置
        pub fn set_check_point_position(
            &self,
            check_point_id: i32,
            x: f32,
            y: f32,
            z: f32,
        ) -> VcmpError {
            let code = (self.inner.SetCheckPointPosition)(check_point_id, x, y, z);
            VcmpError::from(code)
        }

        /// 获取检查点位置
        pub fn get_check_point_position(
            &self,
            check_point_id: i32,
        ) -> Result<(f32, f32, f32), VcmpError> {
            let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
            let code = (self.inner.GetCheckPointPosition)(check_point_id, &mut x, &mut y, &mut z);

            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok((x, y, z))
            }
        }

        /// 设置检查点半径
        pub fn set_check_point_radius(&self, check_point_id: i32, radius: f32) -> VcmpError {
            let code = (self.inner.SetCheckPointRadius)(check_point_id, radius);
            VcmpError::from(code)
        }

        /// 获取检查点半径
        pub fn get_check_point_radius(&self, check_point_id: i32) -> f32 {
            (self.inner.GetCheckPointRadius)(check_point_id)
        }

        /// 获取检查点所有者
        pub fn get_check_point_owner(&self, check_point_id: i32) -> i32 {
            (self.inner.GetCheckPointOwner)(check_point_id)
        }

        /*
         * 对象相关
         */

        /// 创建对象
        pub fn create_object(
            &self,
            model_index: i32,
            world: i32,
            x: f32,
            y: f32,
            z: f32,
            alpha: i32,
        ) -> i32 {
            (self.inner.CreateObject)(model_index, world, x, y, z, alpha)
        }

        /// 删除对象
        pub fn delete_object(&self, object_id: i32) -> VcmpError {
            let code = (self.inner.DeleteObject)(object_id);
            VcmpError::from(code)
        }

        /// 检查对象是否对玩家可见
        pub fn is_object_streamed_for_player(&self, object_id: i32, player_id: i32) -> bool {
            (self.inner.IsObjectStreamedForPlayer)(object_id, player_id) != 0
        }

        /// 获取对象模型
        pub fn get_object_model(&self, object_id: i32) -> i32 {
            (self.inner.GetObjectModel)(object_id)
        }

        /// 设置对象世界
        pub fn set_object_world(&self, object_id: i32, world: i32) -> VcmpError {
            let code = (self.inner.SetObjectWorld)(object_id, world);
            VcmpError::from(code)
        }

        /// 获取对象世界
        pub fn get_object_world(&self, object_id: i32) -> i32 {
            (self.inner.GetObjectWorld)(object_id)
        }

        /// 设置对象透明度
        pub fn set_object_alpha(&self, object_id: i32, alpha: i32, duration: u32) -> VcmpError {
            let code = (self.inner.SetObjectAlpha)(object_id, alpha, duration);
            VcmpError::from(code)
        }

        /// 获取对象透明度
        pub fn get_object_alpha(&self, object_id: i32) -> i32 {
            (self.inner.GetObjectAlpha)(object_id)
        }

        /// 将对象移动到指定位置
        pub fn move_object_to(
            &self,
            object_id: i32,
            x: f32,
            y: f32,
            z: f32,
            duration: u32,
        ) -> VcmpError {
            let code = (self.inner.MoveObjectTo)(object_id, x, y, z, duration);
            VcmpError::from(code)
        }
        pub fn move_object_by(
            &self,
            object_id: i32,
            x: f32,
            y: f32,
            z: f32,
            duration: u32,
        ) -> VcmpResult<()> {
            let code = (self.inner.MoveObjectBy)(object_id, x, y, z, duration);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 设置对象位置
        pub fn set_object_position(&self, object_id: i32, x: f32, y: f32, z: f32) -> VcmpResult<()> {
            let code = (self.inner.SetObjectPosition)(object_id, x, y, z);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取对象位置
        pub fn get_object_position(&self, object_id: i32) -> Result<(f32, f32, f32), VcmpError> {
            let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
            let code = (self.inner.GetObjectPosition)(object_id, &mut x, &mut y, &mut z);

            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok((x, y, z))
            }
        }

        /// 将对象旋转到指定方向
        pub fn rotate_object_to(
            &self,
            object_id: i32,
            x: f32,
            y: f32,
            z: f32,
            w: f32,
            duration: u32,
        ) -> VcmpResult<()> {
            let code = (self.inner.RotateObjectTo)(object_id, x, y, z, w, duration);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 将对象旋转到指定欧拉角
        pub fn rotate_object_to_euler(
            &self,
            object_id: i32,
            x: f32,
            y: f32,
            z: f32,
            duration: u32,
        ) -> VcmpResult<()> {
            let code = (self.inner.RotateObjectToEuler)(object_id, x, y, z, duration);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 将对象按指定方式旋转
        pub fn rotate_object_by(
            &self,
            object_id: i32,
            x: f32,
            y: f32,
            z: f32,
            w: f32,
            duration: u32,
        ) -> VcmpResult<()> {
            let code = (self.inner.RotateObjectBy)(object_id, x, y, z, w, duration);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 将对象按指定欧拉角旋转
        pub fn rotate_object_by_euler(
            &self,
            object_id: i32,
            x: f32,
            y: f32,
            z: f32,
            duration: u32,
        ) -> VcmpResult<()> {
            let code = (self.inner.RotateObjectByEuler)(object_id, x, y, z, duration);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取对象旋转
        pub fn get_object_rotation(&self, object_id: i32) -> Result<(f32, f32, f32, f32), VcmpError> {
            let (mut x, mut y, mut z, mut w) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
            let code = (self.inner.GetObjectRotation)(object_id, &mut x, &mut y, &mut z, &mut w);

            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok((x, y, z, w))
            }
        }

        /// 获取对象欧拉旋转
        pub fn get_object_rotation_euler(&self, object_id: i32) -> Result<(f32, f32, f32), VcmpError> {
            let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
            let code = (self.inner.GetObjectRotationEuler)(object_id, &mut x, &mut y, &mut z);

            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok((x, y, z))
            }
        }

        /// 设置对象被射击报告
        pub fn set_object_shot_report_enabled(&self, object_id: i32, toggle: bool) -> VcmpResult<()> {
            let code = (self.inner.SetObjectShotReportEnabled)(object_id, toggle as u8);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 检查对象被射击报告是否启用
        pub fn is_object_shot_report_enabled(&self, object_id: i32) -> bool {
            (self.inner.IsObjectShotReportEnabled)(object_id) != 0
        }

        /// 设置对象被触摸报告
        pub fn set_object_touched_report_enabled(
            &self,
            object_id: i32,
            toggle: bool,
        ) -> VcmpResult<()> {
            let code = (self.inner.SetObjectTouchedReportEnabled)(object_id, toggle as u8);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 检查对象被触摸报告是否启用
        pub fn is_object_touched_report_enabled(&self, object_id: i32) -> bool {
            (self.inner.IsObjectTouchedReportEnabled)(object_id) != 0
        }

        /*
         * 其他功能
         */

        /// 设置跌落计时器
        pub fn set_fall_timer(&self, time_rate: u16) {
            (self.inner.SetFallTimer)(time_rate);
        }

        /// 获取跌落计时器
        pub fn get_fall_timer(&self) -> u16 {
            (self.inner.GetFallTimer)()
        }

        /// 设置车辆灯光数据
        pub fn set_vehicle_lights_data(&self, vehicle_id: i32, lights_data: u32) -> VcmpResult<()> {
            let code = (self.inner.SetVehicleLightsData)(vehicle_id, lights_data);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取车辆灯光数据
        pub fn get_vehicle_lights_data(&self, vehicle_id: i32) -> u32 {
            (self.inner.GetVehicleLightsData)(vehicle_id)
        }

        /// 杀死玩家
        pub fn kill_player(&self, player_id: i32) -> VcmpResult<()> {
            let code = (self.inner.KillPlayer)(player_id);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 为玩家设置车辆 3D 箭头
        pub fn set_vehicle_3d_arrow_for_player(
            &self,
            vehicle_id: i32,
            target_player_id: i32,
            is_enabled: bool,
        ) -> VcmpResult<()> {
            let code =
                (self.inner.SetVehicle3DArrowForPlayer)(vehicle_id, target_player_id, is_enabled as u8);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家是否为车辆设置 3D 箭头
        pub fn get_vehicle_3d_arrow_for_player(&self, vehicle_id: i32, target_player_id: i32) -> bool {
            (self.inner.GetVehicle3DArrowForPlayer)(vehicle_id, target_player_id) != 0
        }

        /// 为玩家设置玩家 3D 箭头
        pub fn set_player_3d_arrow_for_player(
            &self,
            player_id: i32,
            target_player_id: i32,
            is_enabled: bool,
        ) -> VcmpResult<()> {
            let code =
                (self.inner.SetPlayer3DArrowForPlayer)(player_id, target_player_id, is_enabled as u8);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家是否为玩家设置 3D 箭头
        pub fn get_player_3d_arrow_for_player(&self, player_id: i32, target_player_id: i32) -> bool {
            (self.inner.GetPlayer3DArrowForPlayer)(player_id, target_player_id) != 0
        }

        /// 设置玩家醉酒处理
        pub fn set_player_drunk_handling(&self, player_id: i32, drunk_level: u32) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerDrunkHandling)(player_id, drunk_level);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家醉酒处理级别
        pub fn get_player_drunk_handling(&self, player_id: i32) -> u32 {
            (self.inner.GetPlayerDrunkHandling)(player_id)
        }

        /// 设置玩家醉酒视觉效果
        pub fn set_player_drunk_visuals(&self, player_id: i32, drunk_level: u8) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerDrunkVisuals)(player_id, drunk_level);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家醉酒视觉效果级别
        pub fn get_player_drunk_visuals(&self, player_id: i32) -> u8 {
            (self.inner.GetPlayerDrunkVisuals)(player_id)
        }

        /// 插值摄像机观察目标
        pub fn interpolate_camera_look_at(
            &self,
            player_id: i32,
            look_x: f32,
            look_y: f32,
            look_z: f32,
            interp_time_ms: u32,
        ) -> VcmpResult<()> {
            let code =
                (self.inner.InterpolateCameraLookAt)(player_id, look_x, look_y, look_z, interp_time_ms);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }
    */

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
