use std::ffi::c_void;

use crate::options::VcmpPlayerOption;
use crate::states::VcmpPlayerState;
use crate::utils::{Color, Vectorf32};
use crate::{PlayerId, VcmpError, VcmpResult, VehicleId, encodes::decode_gbk, func::VcmpFunctions};

pub trait PlayerMethods {
    /// 发送 Stream
    fn send_client_script_data(&self, player_id: PlayerId, data: &[u8]) -> VcmpResult<()>;

    /// 发送消息
    fn send_client_message(
        &self,
        player_id: PlayerId,
        color: Color,
        message: &str,
    ) -> VcmpResult<()>;

    /// 发送公告
    fn send_announce(
        &self,
        player_id: PlayerId,
        announce_type: i32,
        message: &str,
    ) -> VcmpResult<()>;

    fn play_sound_for_player(&self, player_id: PlayerId, sound: i32, position: Option<Vectorf32>);

    /*
        Admins?
    */
    fn is_player_admin(&self, player_id: PlayerId) -> bool;

    fn set_player_admin(&self, player_id: PlayerId, admin: bool);
    fn get_player_ip(&self, player: i32) -> String;
    fn get_player_uid(&self, player: i32) -> String;
    fn get_player_uid2(&self, player: i32) -> String;
    fn kick_player(&self, player_id: PlayerId);
    fn ban_player(&self, player_id: PlayerId);

    fn is_player_connected(&self, player: i32) -> bool;
    fn is_player_streamed_for_target(&self, player: i32, target: i32) -> bool;

    fn get_player_key(&self, player: i32) -> u32;

    fn get_player_name(&self, player: i32) -> String;

    fn set_player_name(&self, player: i32, name: &str);

    fn get_player_state(&self, player: i32) -> VcmpPlayerState;

    fn set_player_option(&self, player: i32, option: VcmpPlayerOption, value: bool);

    fn get_player_option(&self, player: i32, option: VcmpPlayerOption) -> bool;

    fn set_player_world(&self, player: i32, world: i32) -> VcmpResult<()>;
    fn get_player_world(&self, player: i32) -> i32;
    fn set_player_select_world(&self, player: i32, world: i32) -> VcmpResult<()>;
    fn get_player_select_world(&self, player: i32) -> i32;
    fn get_player_unique_world(&self, player: i32) -> i32;
    fn is_player_world_compatible(&self, player: i32, world: i32) -> bool;
    fn get_player_class(&self, player: i32) -> i32;
    fn set_player_team(&self, player: i32, team: i32) -> VcmpResult<()>;
    fn get_player_team(&self, player: i32) -> i32;
    fn set_player_skin(&self, player: i32, skin: i32) -> VcmpResult<()>;
    fn get_player_skin(&self, player: i32) -> i32;
    fn set_player_color(&self, player: i32, color: Color) -> VcmpResult<()>;
    fn get_player_color(&self, player: i32) -> Color;
    fn is_player_spawned(&self, player: i32) -> bool;
    fn spawn_player(&self, player: i32) -> VcmpResult<()>;
    fn force_player_select(&self, player: i32) -> VcmpResult<()>;
    fn force_all_select(&self);
    fn is_player_typing(&self, player: i32) -> bool;
    fn give_player_money(&self, player: i32, amount: i32) -> VcmpResult<()>;
    fn set_player_money(&self, player: i32, amount: i32) -> VcmpResult<()>;
    fn get_player_money(&self, player: i32) -> i32;
    fn set_player_score(&self, player: i32, score: i32) -> VcmpResult<()>;
    fn get_player_score(&self, player: i32) -> i32;
    fn set_player_wanted_level(&self, player: i32, level: i32) -> VcmpResult<()>;
    fn get_player_wanted_level(&self, player: i32) -> i32;
    fn get_player_ping(&self, player: i32) -> i32;
    fn get_player_fps(&self, player: i32) -> f64;
    fn set_player_health(&self, player: i32, health: f32) -> VcmpResult<()>;
    fn get_player_health(&self, player: i32) -> f32;
    fn set_player_armour(&self, player: i32, armour: f32) -> VcmpResult<()>;
    fn get_player_armour(&self, player: i32) -> f32;
    fn set_player_immunity(&self, player: i32, flags: u32) -> VcmpResult<()>;
    fn get_player_immunity(&self, player: i32) -> u32;
    fn set_player_position(&self, player: i32, position: Vectorf32) -> VcmpResult<()>;
    fn get_player_position(&self, player: i32) -> VcmpResult<Vectorf32>;
    fn set_player_speed(&self, player: i32, x: f32, y: f32, z: f32) -> VcmpResult<()>;
    fn get_player_speed(&self, player: i32) -> VcmpResult<Vectorf32>;
    fn add_player_speed(&self, player: i32, x: f32, y: f32, z: f32) -> VcmpResult<()>;
    fn set_player_angle(&self, player: i32, angle: f32) -> VcmpResult<()>;
    fn get_player_angle(&self, player: i32) -> f32;
    fn set_player_alpha(&self, player: i32, alpha: i32, fade_time: u32) -> VcmpResult<()>;
    fn get_player_alpha(&self, player: i32) -> i32;
    fn get_player_aim_position(&self, player: i32) -> VcmpResult<Vectorf32>;
    fn get_player_aim_direction(&self, player: i32) -> VcmpResult<Vectorf32>;
    fn is_player_on_fire(&self, player: i32) -> bool;
    fn is_player_crouching(&self, player: i32) -> bool;
    fn get_player_action(&self, player: i32) -> i32;
    fn get_player_game_keys(&self, player: i32) -> u32;
    fn put_player_in_vehicle(
        &self,
        player: i32,
        vehicle_id: VehicleId,
        slot_index: i32,
        make_room: u8,
        warp: u8,
    ) -> VcmpResult<()>;
    fn remove_player_from_vehicle(&self, player: i32) -> VcmpResult<()>;
    fn get_player_in_vehicle_slot(&self, player: i32) -> i32;
    fn get_player_vehicle_id(&self, player: i32) -> i32;
    fn give_player_weapon(&self, player: i32, weapon_id: i32, ammo: i32) -> VcmpResult<()>;
    fn set_player_weapon(&self, player: i32, weapon_id: i32, ammo: i32) -> VcmpResult<()>;
    fn get_player_weapon(&self, player: i32) -> i32;
    fn get_player_weapon_ammo(&self, player: i32) -> i32;
    fn set_player_weapon_slot(&self, player: i32, slot: i32) -> VcmpResult<()>;
    fn get_player_weapon_slot(&self, player: i32) -> i32;
    fn get_player_weapon_at_slot(&self, player: i32, slot: i32) -> i32;
    fn get_player_ammo_at_slot(&self, player: i32, slot: i32) -> i32;
    fn remove_player_weapon(&self, player: i32, weapon_id: i32) -> VcmpResult<()>;
    fn remove_all_weapons(&self, player: i32) -> VcmpResult<()>;
    fn set_camera_position(
        &self,
        player: i32,
        position: Vectorf32,
        look: Vectorf32,
    ) -> VcmpResult<()>;
    fn restore_camera(&self, player: i32) -> VcmpResult<()>;
    fn is_camera_locked(&self, player: i32) -> bool;
    fn set_player_animation(&self, player: i32, group_id: i32, animation_id: i32)
    -> VcmpResult<()>;
    fn get_player_standing_on_vehicle(&self, player: i32) -> i32;
    fn get_player_standing_on_object(&self, player: i32) -> i32;
    fn is_player_away(&self, player: i32) -> bool;
    fn get_player_spectate_target(&self, player: i32) -> i32;
    fn set_player_spectate_target(&self, player: i32, target_id: i32) -> VcmpResult<()>;
    fn redirect_player_to_server(
        &self,
        player: i32,
        ip: &str,
        port: u32,
        nick: &str,
        server_password: &str,
        user_password: &str,
    ) -> VcmpResult<()>;

    fn get_player_module_list(&self, player: i32) -> VcmpResult<()>;
    fn kill_player(&self, player: i32) -> VcmpResult<()>;
    fn set_player_drunk_handling(&self, player: i32, drunk_level: u32) -> VcmpResult<()>;

    fn get_player_drunk_handling(&self, player: i32) -> u32;

    fn set_player_drunk_visuals(&self, player: i32, drunk: bool) -> VcmpResult<()>;

    fn get_player_drunk_visuals(&self, player: i32) -> bool;

    fn set_player_3d_arrow_for_target(
        &self,
        player: i32,
        target: i32,
        show: bool,
    ) -> VcmpResult<()>;

    fn is_player_3d_arrow_for_target(&self, player: i32, target: i32) -> bool;

    fn interpolate_camera_look_at(&self, player: i32, look: Vectorf32, time: u32)
    -> VcmpResult<()>;
}

impl PlayerMethods for VcmpFunctions {
    /// 发送 Stream
    fn send_client_script_data(&self, player_id: PlayerId, data: &[u8]) -> VcmpResult<()> {
        let msg_ptr = data.as_ptr() as *const c_void;
        let code = (self.inner.SendClientScriptData)(player_id, msg_ptr, data.len());
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    /// 发送消息
    fn send_client_message(
        &self,
        player_id: PlayerId,
        color: Color,
        message: &str,
    ) -> VcmpResult<()> {
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
    fn send_announce(
        &self,
        player_id: PlayerId,
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

    */

    fn play_sound_for_player(&self, player_id: PlayerId, sound: i32, position: Option<Vectorf32>) {
        let world = (self.inner.GetPlayerUniqueWorld)(player_id);

        let pos = position.unwrap_or(Vectorf32 {
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

    fn is_player_admin(&self, player_id: PlayerId) -> bool {
        (self.inner.IsPlayerAdmin)(player_id) != 0
    }

    fn set_player_admin(&self, player_id: PlayerId, admin: bool) {
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
        let _ = (self.inner.GetPlayerName)(player, buf_ptr, 1024);
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

    fn set_player_world(&self, player: i32, world: i32) -> VcmpResult<()> {
        let code = (self.inner.SetPlayerWorld)(player, world);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_player_world(&self, player: i32) -> i32 {
        (self.inner.GetPlayerWorld)(player)
    }

    fn set_player_select_world(&self, player: i32, world: i32) -> VcmpResult<()> {
        let code = (self.inner.SetPlayerSecondaryWorld)(player, world);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_player_select_world(&self, player: i32) -> i32 {
        (self.inner.GetPlayerSecondaryWorld)(player)
    }

    fn get_player_unique_world(&self, player: i32) -> i32 {
        (self.inner.GetPlayerUniqueWorld)(player)
    }

    fn is_player_world_compatible(&self, player: i32, world: i32) -> bool {
        (self.inner.IsPlayerWorldCompatible)(player, world) != 0
    }

    fn get_player_class(&self, player: i32) -> i32 {
        (self.inner.GetPlayerClass)(player)
    }

    fn set_player_team(&self, player: i32, team: i32) -> VcmpResult<()> {
        let code = (self.inner.SetPlayerTeam)(player, team);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_player_team(&self, player: i32) -> i32 {
        (self.inner.GetPlayerTeam)(player)
    }

    fn set_player_skin(&self, player: i32, skin: i32) -> VcmpResult<()> {
        let code = (self.inner.SetPlayerSkin)(player, skin);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_player_skin(&self, player: i32) -> i32 {
        (self.inner.GetPlayerSkin)(player)
    }

    fn set_player_color(&self, player: i32, color: Color) -> VcmpResult<()> {
        let code = (self.inner.SetPlayerColour)(player, color.as_rgb());
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_player_color(&self, player: i32) -> Color {
        Color::from_rgb((self.inner.GetPlayerColour)(player), None)
    }

    fn is_player_spawned(&self, player: i32) -> bool {
        (self.inner.IsPlayerSpawned)(player) != 0
    }

    fn spawn_player(&self, player: i32) -> VcmpResult<()> {
        let code = (self.inner.ForcePlayerSpawn)(player);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn force_player_select(&self, player: i32) -> VcmpResult<()> {
        let code = (self.inner.ForcePlayerSelect)(player);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn force_all_select(&self) {
        (self.inner.ForceAllSelect)();
    }

    fn is_player_typing(&self, player: i32) -> bool {
        (self.inner.IsPlayerTyping)(player) != 0
    }

    fn give_player_money(&self, player: i32, amount: i32) -> VcmpResult<()> {
        let code = (self.inner.GivePlayerMoney)(player, amount);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn set_player_money(&self, player: i32, amount: i32) -> VcmpResult<()> {
        let code = (self.inner.SetPlayerMoney)(player, amount);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_player_money(&self, player: i32) -> i32 {
        (self.inner.GetPlayerMoney)(player)
    }

    fn set_player_score(&self, player: i32, score: i32) -> VcmpResult<()> {
        let code = (self.inner.SetPlayerScore)(player, score);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_player_score(&self, player: i32) -> i32 {
        (self.inner.GetPlayerScore)(player)
    }

    fn set_player_wanted_level(&self, player: i32, level: i32) -> VcmpResult<()> {
        let code = (self.inner.SetPlayerWantedLevel)(player, level);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_player_wanted_level(&self, player: i32) -> i32 {
        (self.inner.GetPlayerWantedLevel)(player)
    }

    fn get_player_ping(&self, player: i32) -> i32 {
        (self.inner.GetPlayerPing)(player)
    }

    fn get_player_fps(&self, player: i32) -> f64 {
        (self.inner.GetPlayerFPS)(player)
    }

    fn set_player_health(&self, player: i32, health: f32) -> VcmpResult<()> {
        let code = (self.inner.SetPlayerHealth)(player, health);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_player_health(&self, player: i32) -> f32 {
        (self.inner.GetPlayerHealth)(player)
    }

    fn set_player_armour(&self, player: i32, armour: f32) -> VcmpResult<()> {
        let code = (self.inner.SetPlayerArmour)(player, armour);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_player_armour(&self, player: i32) -> f32 {
        (self.inner.GetPlayerArmour)(player)
    }

    fn set_player_immunity(&self, player: i32, flags: u32) -> VcmpResult<()> {
        let code = (self.inner.SetPlayerImmunityFlags)(player, flags);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_player_immunity(&self, player: i32) -> u32 {
        (self.inner.GetPlayerImmunityFlags)(player)
    }

    fn set_player_position(&self, player: i32, position: Vectorf32) -> VcmpResult<()> {
        let code = (self.inner.SetPlayerPosition)(player, position.x, position.y, position.z);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_player_position(&self, player: i32) -> VcmpResult<Vectorf32> {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let code = (self.inner.GetPlayerPosition)(player, &mut x, &mut y, &mut z);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(Vectorf32 { x, y, z })
        }
    }

    fn set_player_speed(&self, player: i32, x: f32, y: f32, z: f32) -> VcmpResult<()> {
        let code = (self.inner.SetPlayerSpeed)(player, x, y, z);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_player_speed(&self, player: i32) -> VcmpResult<Vectorf32> {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let code = (self.inner.GetPlayerSpeed)(player, &mut x, &mut y, &mut z);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(Vectorf32 { x, y, z })
        }
    }

    fn add_player_speed(&self, player: i32, x: f32, y: f32, z: f32) -> VcmpResult<()> {
        let code = (self.inner.AddPlayerSpeed)(player, x, y, z);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn set_player_angle(&self, player: i32, angle: f32) -> VcmpResult<()> {
        let code = (self.inner.SetPlayerHeading)(player, angle);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_player_angle(&self, player: i32) -> f32 {
        (self.inner.GetPlayerHeading)(player)
    }

    fn set_player_alpha(&self, player: i32, alpha: i32, fade_time: u32) -> VcmpResult<()> {
        let code = (self.inner.SetPlayerAlpha)(player, alpha, fade_time);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_player_alpha(&self, player: i32) -> i32 {
        (self.inner.GetPlayerAlpha)(player)
    }

    fn get_player_aim_position(&self, player: i32) -> VcmpResult<Vectorf32> {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let code = (self.inner.GetPlayerAimPosition)(player, &mut x, &mut y, &mut z);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(Vectorf32 { x, y, z })
        }
    }

    fn get_player_aim_direction(&self, player: i32) -> VcmpResult<Vectorf32> {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let code = (self.inner.GetPlayerAimDirection)(player, &mut x, &mut y, &mut z);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(Vectorf32 { x, y, z })
        }
    }

    fn is_player_on_fire(&self, player: i32) -> bool {
        (self.inner.IsPlayerOnFire)(player) != 0
    }

    fn is_player_crouching(&self, player: i32) -> bool {
        (self.inner.IsPlayerCrouching)(player) != 0
    }

    fn get_player_action(&self, player: i32) -> i32 {
        (self.inner.GetPlayerAction)(player)
    }

    fn get_player_game_keys(&self, player: i32) -> u32 {
        (self.inner.GetPlayerGameKeys)(player)
    }

    fn put_player_in_vehicle(
        &self,
        player: i32,
        vehicle_id: VehicleId,
        slot_index: i32,
        make_room: u8,
        warp: u8,
    ) -> VcmpResult<()> {
        let code = (self.inner.PutPlayerInVehicle)(player, vehicle_id, slot_index, make_room, warp);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn remove_player_from_vehicle(&self, player: i32) -> VcmpResult<()> {
        let code = (self.inner.RemovePlayerFromVehicle)(player);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    // TODO: GetPlayerInVehicleStatus
    //fn get_player_in_vehicle_status(&self, player: i32) ->  {
    //    (self.inner.GetPlayerInVehicleStatus)(player)
    //}

    fn get_player_in_vehicle_slot(&self, player: i32) -> i32 {
        (self.inner.GetPlayerInVehicleSlot)(player)
    }

    fn get_player_vehicle_id(&self, player: i32) -> i32 {
        (self.inner.GetPlayerVehicleId)(player)
    }

    fn give_player_weapon(&self, player: i32, weapon_id: i32, ammo: i32) -> VcmpResult<()> {
        let code = (self.inner.GivePlayerWeapon)(player, weapon_id, ammo);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn set_player_weapon(&self, player: i32, weapon_id: i32, ammo: i32) -> VcmpResult<()> {
        let code = (self.inner.SetPlayerWeapon)(player, weapon_id, ammo);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_player_weapon(&self, player: i32) -> i32 {
        (self.inner.GetPlayerWeapon)(player)
    }

    fn get_player_weapon_ammo(&self, player: i32) -> i32 {
        (self.inner.GetPlayerWeaponAmmo)(player)
    }

    fn set_player_weapon_slot(&self, player: i32, slot: i32) -> VcmpResult<()> {
        let code = (self.inner.SetPlayerWeaponSlot)(player, slot);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_player_weapon_slot(&self, player: i32) -> i32 {
        (self.inner.GetPlayerWeaponSlot)(player)
    }

    fn get_player_weapon_at_slot(&self, player: i32, slot: i32) -> i32 {
        (self.inner.GetPlayerWeaponAtSlot)(player, slot)
    }

    fn get_player_ammo_at_slot(&self, player: i32, slot: i32) -> i32 {
        (self.inner.GetPlayerAmmoAtSlot)(player, slot)
    }

    fn remove_player_weapon(&self, player: i32, weapon_id: i32) -> VcmpResult<()> {
        let code = (self.inner.RemovePlayerWeapon)(player, weapon_id);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn remove_all_weapons(&self, player: i32) -> VcmpResult<()> {
        let code = (self.inner.RemoveAllWeapons)(player);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn set_camera_position(
        &self,
        player: i32,
        position: Vectorf32,
        look: Vectorf32,
    ) -> VcmpResult<()> {
        let code = (self.inner.SetCameraPosition)(
            player, position.x, position.y, position.z, look.x, look.y, look.z,
        );
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn restore_camera(&self, player: i32) -> VcmpResult<()> {
        let code = (self.inner.RestoreCamera)(player);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn is_camera_locked(&self, player: i32) -> bool {
        (self.inner.IsCameraLocked)(player) != 0
    }

    fn set_player_animation(
        &self,
        player: i32,
        group_id: i32,
        animation_id: i32,
    ) -> VcmpResult<()> {
        let code = (self.inner.SetPlayerAnimation)(player, group_id, animation_id);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_player_standing_on_vehicle(&self, player: i32) -> i32 {
        (self.inner.GetPlayerStandingOnVehicle)(player)
    }

    fn get_player_standing_on_object(&self, player: i32) -> i32 {
        (self.inner.GetPlayerStandingOnObject)(player)
    }

    fn is_player_away(&self, player: i32) -> bool {
        (self.inner.IsPlayerAway)(player) != 0
    }

    fn get_player_spectate_target(&self, player: i32) -> i32 {
        (self.inner.GetPlayerSpectateTarget)(player)
    }

    fn set_player_spectate_target(&self, player: i32, target_id: i32) -> VcmpResult<()> {
        let code = (self.inner.SetPlayerSpectateTarget)(player, target_id);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    // TODO: need to rewrite code
    fn redirect_player_to_server(
        &self,
        player: i32,
        ip: &str,
        port: u32,
        nick: &str,
        server_password: &str,
        user_password: &str,
    ) -> VcmpResult<()> {
        use std::ffi::CString;

        let c_ip = CString::new(ip).unwrap();
        let c_nick = CString::new(nick).unwrap();
        let c_server_password = CString::new(server_password).unwrap();
        let c_user_password = CString::new(user_password).unwrap();

        let code = (self.inner.RedirectPlayerToServer)(
            player,
            c_ip.as_ptr(),
            port,
            c_nick.as_ptr(),
            c_server_password.as_ptr(),
            c_user_password.as_ptr(),
        );

        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_player_module_list(&self, player: i32) -> VcmpResult<()> {
        let code = (self.inner.GetPlayerModuleList)(player);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn kill_player(&self, player: i32) -> VcmpResult<()> {
        let code = (self.inner.KillPlayer)(player);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_player_drunk_handling(&self, player: i32, drunk_level: u32) -> VcmpResult<()> {
        let code = (self.inner.SetPlayerDrunkHandling)(player, drunk_level);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_player_drunk_handling(&self, player: i32) -> u32 {
        (self.inner.GetPlayerDrunkHandling)(player)
    }

    fn set_player_drunk_visuals(&self, player: i32, drunk: bool) -> VcmpResult<()> {
        let code = (self.inner.SetPlayerDrunkVisuals)(player, drunk as u8);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_player_drunk_visuals(&self, player: i32) -> bool {
        (self.inner.GetPlayerDrunkVisuals)(player) != 0
    }

    fn is_player_3d_arrow_for_target(&self, player: i32, target: i32) -> bool {
        (self.inner.GetPlayer3DArrowForPlayer)(player, target) != 0
    }

    fn set_player_3d_arrow_for_target(
        &self,
        player: i32,
        target: i32,
        show: bool,
    ) -> VcmpResult<()> {
        let code = (self.inner.SetPlayer3DArrowForPlayer)(player, target, show as u8);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn interpolate_camera_look_at(
        &self,
        player: i32,
        look: Vectorf32,
        time: u32,
    ) -> VcmpResult<()> {
        let code = (self.inner.InterpolateCameraLookAt)(player, look.x, look.y, look.z, time);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
}
