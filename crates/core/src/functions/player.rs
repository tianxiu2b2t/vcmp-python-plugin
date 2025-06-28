use pyo3::{pyclass, pymethods, types::{PyModule, PyModuleMethods}, Bound, PyResult, Python};
use vcmp_bindings::{states::VcmpPlayerState, utils::Vectorf32};

use crate::pool::EntityPoolTrait;
use crate::py::streams::WriteStream;
use crate::py::types::RGBPy;
use vcmp_bindings::{func::PlayerMethods, options::VcmpPlayerOption, vcmp_func};

#[pyclass]
#[pyo3(name = "Player")]
#[derive(Debug, Clone, Copy)]
pub struct PlayerPy {
    id: i32,
}

impl PlayerPy {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

impl From<i32> for PlayerPy {
    fn from(val: i32) -> Self {
        PlayerPy::new(val)
    }
}

impl EntityPoolTrait for PlayerPy {
    fn entity_id(&self) -> crate::consts::EntityId {
        self.id
    }

    fn entity_pool_type() -> vcmp_bindings::options::VcmpEntityPool {
        vcmp_bindings::options::VcmpEntityPool::Player
    }
}

#[pymethods]
impl PlayerPy {
    #[getter]
    pub fn get_id(&self) -> i32 {
        self.id
    }

    #[getter]
    fn get_name(&self) -> String {
        vcmp_func().get_player_name(self.id)
    }

    #[setter]
    fn set_name(&self, name: String) {
        vcmp_func().set_player_name(self.id, name.as_str());
    }

    #[getter]
    fn get_is_alive(&self) -> bool {
        vcmp_func().is_player_connected(self.id)
    }

    #[getter]
    fn get_admin(&self) -> bool {
        vcmp_func().is_player_admin(self.id)
    }

    #[setter]
    fn set_admin(&self, admin: bool) {
        vcmp_func().set_player_admin(self.id, admin);
    }

    #[getter]
    fn get_ip(&self) -> String {
        vcmp_func().get_player_ip(self.id)
    }

    #[getter]
    fn get_unique_id(&self) -> String {
        vcmp_func().get_player_uid(self.id)
    }

    #[getter]
    fn get_unique_id2(&self) -> String {
        vcmp_func().get_player_uid2(self.id)
    }

    #[getter]
    fn get_ping(&self) -> i32 {
        vcmp_func().get_player_ping(self.id)
    }

    #[getter]
    fn get_score(&self) -> i32 {
        vcmp_func().get_player_score(self.id)
    }

    #[setter]
    fn set_score(&self, score: i32) {
        let _ = vcmp_func().set_player_score(self.id, score);
    }

    #[getter]
    fn get_color(&self) -> RGBPy {
        RGBPy::from(vcmp_func().get_player_color(self.id))
    }

    #[setter]
    fn set_color(&self, value: RGBPy) {
        let _ = vcmp_func().set_player_color(self.id, value.into());
    }

    fn send_data(&self, data: WriteStream) {
        let _ = vcmp_func().send_client_script_data(self.id, data.raw_buffer().as_slice());
    }

    fn send_raw_message(&self, color: RGBPy, message: &str) {
        let _ = vcmp_func().send_client_message(self.id, color.into(), message);
    }

    fn send_message(&self, message: &str) {
        self.send_raw_message(RGBPy::from_rgb(0xFFFFFF, None), message);
    }

    fn send_announce(&self, announce_type: i32, message: &str) {
        let _ = vcmp_func().send_announce(self.id, announce_type, message);
    }

    fn play_sound(&self, sound: i32, position: Option<(f32, f32, f32)>) {
        let pos = position.map(Vectorf32::from);
        vcmp_func().play_sound_for_player(self.id, sound, pos);
    }

    fn kick(&self, message: Option<&str>) {
        if let Some(message) = message {
            self.send_message(message);
        }
        vcmp_func().kick_player(self.id);
    }

    fn ban(&self, message: Option<&str>) {
        if let Some(message) = message {
            self.send_message(message);
        }
        vcmp_func().ban_player(self.id);
    }

    fn is_streamed_for_player(&self, player: i32) -> bool {
        vcmp_func().is_player_streamed_for_target(self.id, player)
    }

    #[getter]
    fn get_key(&self) -> u32 {
        vcmp_func().get_player_key(self.id)
    }

    // #[getter]
    // fn get_state(&self) -> VcmpPlayerState {
    //     vcmp_func().get_player_state(self.id)
    // }

    /*

        pub const vcmpPlayerOption_vcmpPlayerOptionControllable: vcmpPlayerOption = 0;
    pub const vcmpPlayerOption_vcmpPlayerOptionDriveBy: vcmpPlayerOption = 1;
    pub const vcmpPlayerOption_vcmpPlayerOptionWhiteScanlines: vcmpPlayerOption = 2;
    pub const vcmpPlayerOption_vcmpPlayerOptionGreenScanlines: vcmpPlayerOption = 3;
    pub const vcmpPlayerOption_vcmpPlayerOptionWidescreen: vcmpPlayerOption = 4;
    pub const vcmpPlayerOption_vcmpPlayerOptionShowMarkers: vcmpPlayerOption = 5;
    pub const vcmpPlayerOption_vcmpPlayerOptionCanAttack: vcmpPlayerOption = 6;
    pub const vcmpPlayerOption_vcmpPlayerOptionHasMarker: vcmpPlayerOption = 7;
    pub const vcmpPlayerOption_vcmpPlayerOptionChatTagsEnabled: vcmpPlayerOption = 8;
    pub const vcmpPlayerOption_vcmpPlayerOptionDrunkEffectsDeprecated: vcmpPlayerOption = 9;
    pub const vcmpPlayerOption_vcmpPlayerOptionBleeding: vcmpPlayerOption = 10;
    pub const vcmpPlayerOption_forceSizeVcmpPlayerOption: vcmpPlayerOption = 2147483647;
         */

    #[getter]
    fn get_controllable(&self) -> bool {
        vcmp_func().get_player_option(self.id, VcmpPlayerOption::Controllable)
    }

    #[setter]
    fn set_controllable(&self, controllable: bool) {
        vcmp_func().set_player_option(self.id, VcmpPlayerOption::Controllable, controllable);
    }

    /*
       reverse controllable
    */

    #[getter]
    fn get_frozen(&self) -> bool {
        !self.get_controllable()
    }

    #[setter]
    fn set_frozen(&self, frozen: bool) {
        self.set_controllable(!frozen);
    }

    #[getter]
    fn get_drive_by(&self) -> bool {
        vcmp_func().get_player_option(self.id, VcmpPlayerOption::DriveBy)
    }

    #[setter]
    fn set_drive_by(&self, drive_by: bool) {
        vcmp_func().set_player_option(self.id, VcmpPlayerOption::DriveBy, drive_by);
    }

    #[getter]
    fn get_white_scanlines(&self) -> bool {
        vcmp_func().get_player_option(self.id, VcmpPlayerOption::WhiteScanlines)
    }

    #[setter]
    fn set_white_scanlines(&self, white_scanlines: bool) {
        vcmp_func().set_player_option(self.id, VcmpPlayerOption::WhiteScanlines, white_scanlines);
    }

    #[getter]
    fn get_green_scanlines(&self) -> bool {
        vcmp_func().get_player_option(self.id, VcmpPlayerOption::GreenScanlines)
    }

    #[setter]
    fn set_green_scanlines(&self, green_scanlines: bool) {
        vcmp_func().set_player_option(self.id, VcmpPlayerOption::GreenScanlines, green_scanlines);
    }

    #[getter]
    fn get_widescreen(&self) -> bool {
        vcmp_func().get_player_option(self.id, VcmpPlayerOption::Widescreen)
    }

    #[setter]
    fn set_widescreen(&self, widescreen: bool) {
        vcmp_func().set_player_option(self.id, VcmpPlayerOption::Widescreen, widescreen);
    }

    #[getter]
    fn get_show_markers(&self) -> bool {
        vcmp_func().get_player_option(self.id, VcmpPlayerOption::ShowMarkers)
    }

    #[setter]
    fn set_show_markers(&self, show_markers: bool) {
        vcmp_func().set_player_option(self.id, VcmpPlayerOption::ShowMarkers, show_markers);
    }

    #[getter]
    fn get_can_attack(&self) -> bool {
        vcmp_func().get_player_option(self.id, VcmpPlayerOption::CanAttack)
    }

    #[setter]
    fn set_can_attack(&self, can_attack: bool) {
        vcmp_func().set_player_option(self.id, VcmpPlayerOption::CanAttack, can_attack);
    }

    #[getter]
    fn get_has_marker(&self) -> bool {
        vcmp_func().get_player_option(self.id, VcmpPlayerOption::HasMarker)
    }

    #[setter]
    fn set_has_marker(&self, has_marker: bool) {
        vcmp_func().set_player_option(self.id, VcmpPlayerOption::HasMarker, has_marker);
    }

    #[getter]
    fn get_chat_tags_enabled(&self) -> bool {
        vcmp_func().get_player_option(self.id, VcmpPlayerOption::ChatTagsEnabled)
    }

    #[setter]
    fn set_chat_tags_enabled(&self, chat_tags_enabled: bool) {
        vcmp_func().set_player_option(
            self.id,
            VcmpPlayerOption::ChatTagsEnabled,
            chat_tags_enabled,
        );
    }

    #[getter]
    fn get_bleeding(&self) -> bool {
        vcmp_func().get_player_option(self.id, VcmpPlayerOption::Bleeding)
    }

    #[setter]
    fn set_bleeding(&self, bleeding: bool) {
        vcmp_func().set_player_option(self.id, VcmpPlayerOption::Bleeding, bleeding);
    }

    #[getter]
    fn get_drunk_effects(&self) -> bool {
        vcmp_func().get_player_option(self.id, VcmpPlayerOption::DrunkEffectsDeprecated)
    }

    #[setter]
    fn set_drunk_effects(&self, drunk_effects: bool) {
        vcmp_func().set_player_option(
            self.id,
            VcmpPlayerOption::DrunkEffectsDeprecated,
            drunk_effects,
        );
    }

    #[getter]
    fn get_world(&self) -> i32 {
        vcmp_func().get_player_world(self.id)
    }

    #[setter]
    fn set_world(&self, world: i32) {
        let _ = vcmp_func().set_player_world(self.id, world);
    }

    #[getter]
    fn get_select_world(&self) -> i32 {
        vcmp_func().get_player_select_world(self.id)
    }

    #[setter]
    fn set_select_world(&self, world: i32) {
        let _ = vcmp_func().set_player_select_world(self.id, world);
    }

    #[getter]
    fn get_unique_world(&self) -> i32 {
        vcmp_func().get_player_unique_world(self.id)
    }

    fn is_world_compatible(&self, world: i32) -> bool {
        vcmp_func().is_player_world_compatible(self.id, world)
    }

    #[getter]
    fn get_class_id(&self) -> i32 {
        vcmp_func().get_player_class(self.id)
    }

    #[getter]
    fn get_team(&self) -> i32 {
        vcmp_func().get_player_team(self.id)
    }

    #[setter]
    fn set_team(&self, team: i32) {
        let _ = vcmp_func().set_player_team(self.id, team);
    }

    #[getter]
    fn get_skin(&self) -> i32 {
        vcmp_func().get_player_skin(self.id)
    }

    #[setter]
    fn set_skin(&self, skin: i32) {
        let _ = vcmp_func().set_player_skin(self.id, skin);
    }

    // #[getter]
    // fn get_spawned(&self) -> bool {
    //     vcmp_func().get_player_spaw(self.id)
    // }

    // fn spawn(&self) {
    //     let _ = vcmp_func().spawn_player(self.id);
    // }

    // fn force_select(&self) {
    //     vcmp_func().force_player_select(self.id, self.id);
    // }

    // fn force_player_select(&self, player: i32) -> VcmpResult<()>;
    // fn force_all_select(&self);
    // fn is_player_typing(&self, player: i32) -> bool;
    // fn give_player_money(&self, player: i32, amount: i32) -> VcmpResult<()>;
    // fn set_player_money(&self, player: i32, amount: i32) -> VcmpResult<()>;
    // fn get_player_money(&self, player: i32) -> i32;
    // fn set_player_score(&self, player: i32, score: i32) -> VcmpResult<()>;
    // fn get_player_score(&self, player: i32) -> i32;
    // fn set_player_wanted_level(&self, player: i32, level: i32) -> VcmpResult<()>;
    // fn get_player_wanted_level(&self, player: i32) -> i32;
    // fn get_player_ping(&self, player: i32) -> i32;
    // fn get_player_fps(&self, player: i32) -> f64;
    // fn set_player_health(&self, player: i32, health: f32) -> VcmpResult<()>;
    // fn get_player_health(&self, player: i32) -> f32;
    // fn set_player_armour(&self, player: i32, armour: f32) -> VcmpResult<()>;
    // fn get_player_armour(&self, player: i32) -> f32;
    // fn set_player_immunity(&self, player: i32, flags: u32) -> VcmpResult<()>;
    // fn get_player_immunity(&self, player: i32) -> u32;
    // fn set_player_position(&self, player: i32, position: Vectorf32) -> VcmpResult<()>;
    // fn get_player_position(&self, player: i32) -> VcmpResult<Vectorf32>;
    // fn set_player_speed(&self, player: i32, x: f32, y: f32, z: f32) -> VcmpResult<()>;
    // fn get_player_speed(&self, player: i32) -> VcmpResult<Vectorf32>;
    // fn add_player_speed(&self, player: i32, x: f32, y: f32, z: f32) -> VcmpResult<()>;
    // fn set_player_angle(&self, player: i32, angle: f32) -> VcmpResult<()>;
    // fn get_player_angle(&self, player: i32) -> f32;
    // fn set_player_alpha(&self, player: i32, alpha: i32, fade_time: u32) -> VcmpResult<()>;
    // fn get_player_alpha(&self, player: i32) -> i32;
    // fn get_player_aim_position(&self, player: i32) -> VcmpResult<Vectorf32>;
    // fn get_player_aim_direction(&self, player: i32) -> VcmpResult<Vectorf32>;
    // fn is_player_on_fire(&self, player: i32) -> bool;
    // fn is_player_crouching(&self, player: i32) -> bool;
    // fn get_player_action(&self, player: i32) -> i32;
    // fn get_player_game_keys(&self, player: i32) -> u32;
    // fn put_player_in_vehicle(
    //     &self,
    //     player: i32,
    //     vehicle_id: VehicleId,
    //     slot_index: i32,
    //     make_room: u8,
    //     warp: u8,
    // ) -> VcmpResult<()>;
    // fn remove_player_from_vehicle(&self, player: i32) -> VcmpResult<()>;
    // fn get_player_in_vehicle_slot(&self, player: i32) -> i32;
    // fn get_player_vehicle_id(&self, player: i32) -> i32;
    // fn give_player_weapon(&self, player: i32, weapon_id: i32, ammo: i32) -> VcmpResult<()>;
    // fn set_player_weapon(&self, player: i32, weapon_id: i32, ammo: i32) -> VcmpResult<()>;
    // fn get_player_weapon(&self, player: i32) -> i32;
    // fn get_player_weapon_ammo(&self, player: i32) -> i32;
    // fn set_player_weapon_slot(&self, player: i32, slot: i32) -> VcmpResult<()>;
    // fn get_player_weapon_slot(&self, player: i32) -> i32;
    // fn get_player_weapon_at_slot(&self, player: i32, slot: i32) -> i32;
    // fn get_player_ammo_at_slot(&self, player: i32, slot: i32) -> i32;
    // fn remove_player_weapon(&self, player: i32, weapon_id: i32) -> VcmpResult<()>;
    // fn remove_all_weapons(&self, player: i32) -> VcmpResult<()>;
    // fn set_camera_position(
    //     &self,
    //     player: i32,
    //     position: Vectorf32,
    //     look: Vectorf32,
    // ) -> VcmpResult<()>;
    // fn restore_camera(&self, player: i32) -> VcmpResult<()>;
    // fn is_camera_locked(&self, player: i32) -> bool;
    // fn set_player_animation(&self, player: i32, group_id: i32, animation_id: i32)
    // -> VcmpResult<()>;
    // fn get_player_standing_on_vehicle(&self, player: i32) -> i32;
    // fn get_player_standing_on_object(&self, player: i32) -> i32;
    // fn is_player_away(&self, player: i32) -> bool;
    // fn get_player_spectate_target(&self, player: i32) -> i32;
    // fn set_player_spectate_target(&self, player: i32, target_id: i32) -> VcmpResult<()>;
    // fn redirect_player_to_server(
    //     &self,
    //     player: i32,
    //     ip: &str,
    //     port: u32,
    //     nick: &str,
    //     server_password: &str,
    //     user_password: &str,
    // ) -> VcmpResult<()>;

    // fn get_player_module_list(&self, player: i32) -> VcmpResult<()>;
    // fn kill_player(&self, player: i32) -> VcmpResult<()>;
    // fn set_player_drunk_handling(&self, player: i32, drunk_level: u32) -> VcmpResult<()>;

    // fn get_player_drunk_handling(&self, player: i32) -> u32;

    // fn set_player_drunk_visuals(&self, player: i32, drunk: bool) -> VcmpResult<()>;

    // fn get_player_drunk_visuals(&self, player: i32) -> bool;

    // fn set_player_3d_arrow_for_target(
    //     &self,
    //     player: i32,
    //     target: i32,
    //     show: bool,
    // ) -> VcmpResult<()>;

    // fn is_player_3d_arrow_for_target(&self, player: i32, target: i32) -> bool;

    // fn interpolate_camera_look_at(&self, player: i32, look: Vectorf32, time: u32)
    // -> VcmpResult<()>;
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PlayerPy>()?;
    Ok(())
}
