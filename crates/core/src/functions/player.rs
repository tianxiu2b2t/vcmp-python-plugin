use std::ops::Add;

use pyo3::{
    Bound, PyResult, Python, pyclass, pymethods,
    types::{PyModule, PyModuleMethods},
};
use vcmp_bindings::{func::QueryVehicle, states::VcmpPlayerState, utils::Vectorf32};

use crate::{functions::{object::ObjectPy, vehicle::VehiclePy}, pool::{EntityPoolTrait, ENTITY_POOL}, py::types::{EntityVectorType, VectorPy}};
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

impl PlayerPy {
    pub fn _position(&self) -> VectorPy {
        VectorPy::from((EntityVectorType::PlayerPosition, self.id))
    }

    pub fn _speed(&self) -> VectorPy {
        VectorPy::from((EntityVectorType::PlayerSpeed, self.id))
    }
}

#[pymethods]
impl PlayerPy {
    #[getter]
    fn get_action(&self) -> i32 {
        vcmp_func().get_player_action(self.id)
    }

    fn add_position(&mut self, pos: VectorPy) {
        let origin = self._position();
        origin.add(pos);
    }

    fn add_speed(&mut self, speed: VectorPy) {
        let origin = self._speed();
        origin.add(speed);
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
    fn get_player_aim_direction(&self) -> VectorPy {
        let res = vcmp_func().get_player_aim_direction(self.id);
        if let Ok(pos) = res {
            VectorPy::from(pos)
        } else {
            VectorPy::default()
        }
    }

    #[getter]
    fn get_player_aim_position(&self) -> VectorPy {
        let res = vcmp_func().get_player_aim_position(self.id);
        if let Ok(pos) = res {
            VectorPy::from(pos)
        } else {
            VectorPy::default()
        }
    }

    #[getter]
    fn get_alpha(&self) -> i32 {
        vcmp_func().get_player_alpha(self.id)
    }

    fn set_alpha(&self, alpha: i32, fade_time: u32) {
        let _ = vcmp_func().set_player_alpha(self.id, alpha, fade_time);
    }

    #[getter]
    fn get_angle(&self) -> f32 {
        vcmp_func().get_player_angle(self.id)
    }

    #[setter]
    fn set_angle(&self, angle: f32) {
        let _ = vcmp_func().set_player_angle(self.id, angle);
    }

    #[getter]
    fn get_armour(&self) -> f32 {
        vcmp_func().get_player_armour(self.id)
    }

    #[setter]
    fn set_armour(&self, armour: f32) {
        let _ = vcmp_func().set_player_armour(self.id, armour);
    }

    #[getter]
    fn get_away(&self) -> bool {
        vcmp_func().is_player_away(self.id)
    }

    fn ban(&self, message: Option<&str>) {
        if let Some(message) = message {
            self.send_message(message);
        }
        vcmp_func().ban_player(self.id);
    }

    #[getter]
    fn get_camera_locked(&self) -> bool {
        vcmp_func().is_camera_locked(self.id)
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
    fn get_cash(&self) -> i32 {
        vcmp_func().get_player_money(self.id)
    }

    #[setter]
    fn set_cash(&self, cash: i32) {
        let _ = vcmp_func().set_player_money(self.id, cash);
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
    fn get_class_id(&self) -> i32 {
        vcmp_func().get_player_class(self.id)
    }

    fn clear_weapons(&self) {
        let _ = vcmp_func().remove_all_weapons(self.id);
    }

    fn disarm(&self) {
        self.clear_weapons()
    }

    #[getter]
    fn get_color(&self) -> RGBPy {
        RGBPy::from(vcmp_func().get_player_color(self.id))
    }

    #[setter]
    fn set_color(&self, value: RGBPy) {
        let _ = vcmp_func().set_player_color(self.id, value.into());
    }

        #[getter]
    fn get_controllable(&self) -> bool {
        vcmp_func().get_player_option(self.id, VcmpPlayerOption::Controllable)
    }

    #[setter]
    fn set_controllable(&self, controllable: bool) {
        vcmp_func().set_player_option(self.id, VcmpPlayerOption::Controllable, controllable);
    }

    #[getter]
    fn get_frozen(&self) -> bool {
        !self.get_controllable()
    }

    #[setter]
    fn set_frozen(&self, frozen: bool) {
        self.set_controllable(!frozen);
    }

    #[getter]
    fn crouching(&self) -> bool {
        vcmp_func().is_player_crouching(self.id)
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
    fn get_fps(&self) -> f64 {
        vcmp_func().get_player_fps(self.id)
    }

    #[getter]
    fn get_game_keys(&self) -> u32 {
        vcmp_func().get_player_game_keys(self.id)
    }

    #[getter]
    fn get_weapon(&self) -> i32 {
        vcmp_func().get_player_weapon(self.id)
    }

    #[getter]
    fn get_weapon_ammo(&self) -> i32 {
        vcmp_func().get_player_weapon_ammo(self.id)
    }

    fn get_weapon_ammo_at_slot(&self, slot: i32) -> i32 {
        vcmp_func().get_player_ammo_at_slot(self.id, slot)
    }

    fn get_weapon_at_slot(&self, slot: i32) -> i32 {
        vcmp_func().get_player_weapon_at_slot(self.id, slot)
    }

    #[getter]
    fn get_weapon_slot(&self) -> i32 {
        vcmp_func().get_player_weapon_slot(self.id)
    }

    fn give_weapon(&self, weapon: i32, ammo: i32) {
        vcmp_func().give_player_weapon(self.id, weapon, ammo);
    }

    #[getter]
    fn get_green_scanlines(&self) -> bool {
        vcmp_func().get_player_option(self.id, VcmpPlayerOption::GreenScanlines)
    }

    #[setter]
    fn set_green_scanlines(&self, green_scanlines: bool) {
        vcmp_func().set_player_option(
            self.id,
            VcmpPlayerOption::GreenScanlines,
            green_scanlines,
        );
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
    fn get_health(&self) -> f32 {
        vcmp_func().get_player_health(self.id)
    }

    #[setter]
    fn set_health(&self, health: f32) {
        let _ = vcmp_func().set_player_health(self.id, health);
    }

    #[getter]
    fn get_id(&self) -> i32 {
        self.id
    }

    #[getter]
    fn get_immunity(&self) -> u32 {
        vcmp_func().get_player_immunity(self.id)
    }

    #[setter]
    fn set_immunity(&self, flags: u32) {
        vcmp_func().set_player_immunity(self.id, flags);
    }

    #[getter]
    fn get_ip(&self) -> String {
        vcmp_func().get_player_ip(self.id)
    }

    #[getter]
    fn is_alive(&self) -> bool {
        vcmp_func().is_player_connected(self.id)
    }

    fn is_streamed_for_target(&self, player: i32) -> bool {
        vcmp_func().is_player_streamed_for_target(self.id, player)
    }

    #[getter]
    fn get_key(&self) -> u32 {
        vcmp_func().get_player_key(self.id)
    }

    fn kick(&self, message: Option<&str>) {
        if let Some(message) = message {
            self.send_message(message);
        }
        vcmp_func().kick_player(self.id);
    }

    fn kill(&self) {
        vcmp_func().kill_player(self.id);
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
    fn on_fire(&self) -> bool {
        vcmp_func().is_player_on_fire(self.id)
    }

    #[getter]
    fn get_ping(&self) -> i32 {
        vcmp_func().get_player_ping(self.id)
    }

    fn play_animation(&self, group_id: i32, animation_id: i32) {
        let _ = vcmp_func().set_player_animation(self.id, group_id, animation_id);
    }

    fn play_sound(&self, sound: i32, position: Option<VectorPy>) {
        let pos = position.map(Vectorf32::from).unwrap_or_else(|| Vectorf32::from((f32::NAN, f32::NAN, f32::NAN)));
        vcmp_func().play_sound_for_player(self.id, sound, Some(pos));
    }

    #[getter]
    fn get_position(&self) -> VectorPy {
        self._position()
    }

    #[setter]
    fn set_position(&self, position: VectorPy) {
        let _ = vcmp_func().set_player_position(self.id, position.get_entity_pos());
    }

    fn redirect(&self, ip: &str, port: u32, nick: &str, password: &str, user_password: &str) {
        vcmp_func().redirect_player_to_server(self.id, ip, port, nick, password, user_password);
    }

    fn remove_weapon(&self, weapon: i32) {
        vcmp_func().remove_player_weapon(self.id, weapon);
    }

    fn request_module_list(&self) {
        vcmp_func().get_player_module_list(self.id);
    }

    fn restore_camera(&self) {
        vcmp_func().restore_camera(self.id);
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
    fn get_sec_world(&self) -> i32 {
        vcmp_func().get_player_secondary_world(self.id)
    }

    #[setter]
    fn set_sec_world(&self, sec_world: i32) {
        let _ = vcmp_func().set_player_secondary_world(self.id, sec_world);
    }

    #[getter]
    fn get_world(&self) -> i32 {
        vcmp_func().get_player_world(self.id)
    }

    #[setter]
    fn set_world(&self, world: i32) {
        let _ = vcmp_func().set_player_world(self.id, world);
    }

    fn select(&self) {
        vcmp_func().force_player_select(self.id);
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

    #[getter]
    fn get_unique_world(&self) -> i32 {
        vcmp_func().get_player_unique_world(self.id)
    }
    
    fn set_camera_position(&self, position: VectorPy, look_at: VectorPy) {
        let _ = vcmp_func().set_camera_position(self.id, position.get_entity_pos(), look_at.get_entity_pos());
    }

    fn set_camera(&self, position: VectorPy, look_yaw: f32, look_pitch: f32, range: Option<f32>) {
        /*
        look = Vector(0, 0, 0)
        look.x = math.cos(math.radians(look_yaw)) * range
        look.y = math.sin(math.radians(look_yaw)) * range
        look.z = math.sin(math.radians(look_pitch)) * range
        look = look + position */

        let mut look = Vectorf32::default();
        let range = range.unwrap_or(0.5);
        look.x = (look_yaw.cos() * range) as f32;
        look.y = (look_yaw.sin() * range) as f32;
        look.z = (look_pitch.sin() * range) as f32;
        let py_look = VectorPy::from(look);
        let origin = position.clone();
        let camera_position = position.clone();
        self.set_camera_position(origin, py_look + camera_position);
    }

    fn set_vehicle_slot(&self, vehicle: Option<VehiclePy>, slot: i32) {
        if vehicle.is_none() {
            let _ = vcmp_func().remove_player_from_vehicle(self.id);
            return;
        }

        let _ = vcmp_func().put_player_in_vehicle(self.id, vehicle.unwrap().get_id(), slot, 1, 0);
    }

    fn set_weapon(&self, weapon: i32, ammo: i32) {
        let _ = vcmp_func().set_player_weapon(self.id, weapon, ammo);
    }

    fn set_weapon_slot(&self, slot: i32) {
        let _ = vcmp_func().set_player_weapon_slot(self.id, slot);
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
    fn get_skin(&self) -> i32 {
        vcmp_func().get_player_skin(self.id)
    }

    #[setter]
    fn set_skin(&self, skin: i32) {
        let _ = vcmp_func().set_player_skin(self.id, skin);
    }

    fn spawn(&self) {
        let _ = vcmp_func().spawn_player(self.id);
    }

    
    #[getter]
    fn get_spawned(&self) -> bool {
        vcmp_func().is_player_spawned(self.id)
    }

    #[getter]
    fn get_spectate_target(&self) -> Option<PlayerPy> {
        let pool = ENTITY_POOL.lock().unwrap();
        let id = vcmp_func().get_player_spectate_target(self.id);
        pool.get_player(id).map(|p| *p)
    }

    #[getter]
    fn get_speed(&self) -> VectorPy {
        VectorPy::from((EntityVectorType::PlayerSpeed, self.id))
    }

    #[setter]
    fn set_speed(&self, speed: VectorPy) {
        let _ = vcmp_func().set_player_speed(self.id, speed.get_entity_pos());
    }

    #[getter]
    fn get_standing_on_object(&self) -> Option<ObjectPy> {
        let id = vcmp_func().get_player_standing_on_object(self.id);
        let pool = ENTITY_POOL.lock().unwrap();
        pool.get_object(id).map(|o| *o)
    }

    #[getter]
    fn get_standing_vehicle(&self) -> Option<VehiclePy> {
        let id = vcmp_func().get_player_standing_on_vehicle(self.id);
        let pool = ENTITY_POOL.lock().unwrap();
        pool.get_vehicle(id).map(|o| *o)
    }

    #[getter]
    fn get_state(&self) -> i32 {
        vcmp_func().get_player_state(self.id) as i32
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
    fn is_typing(&self) -> bool {
        vcmp_func().is_player_typing(self.id)
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
    fn get_vehicle(&self) -> Option<VehiclePy> {
        let vehicle_id = vcmp_func().get_player_vehicle_id(self.id);
        let pool = ENTITY_POOL.lock().unwrap();
        pool.get_vehicle(vehicle_id).map(|veh| *veh)
    }

    #[setter]
    fn set_vehicle(&self, vehicle: Option<VehiclePy>) {
        let mut id = None;
        if let Some(vehicle) = vehicle {
            id = Some(vehicle.get_id());
        }
        if let Some(id) = id && vcmp_func().is_vehicle_alive(id) {
            let origin = vcmp_func().get_player_vehicle_id(self.id);
            if origin != id {
                let _ = vcmp_func().remove_player_from_vehicle(self.id);
                let _ = vcmp_func().put_player_in_vehicle(self.id, id, 0, 0, 1);
            }
        } else {
            let _ = vcmp_func().remove_player_from_vehicle(self.id);
        }
    }

    #[getter]
    fn get_vehicle_status(&self) -> i32 {
        vcmp_func().get_player_in_vehicle_status(self.id) as i32
    }

    #[getter]
    fn get_wanted_level(&self) -> i32 {
        vcmp_func().get_player_wanted_level(self.id)
    }

    #[setter]
    fn set_wanted_level(&self, wanted_level: i32) {
        let _ = vcmp_func().set_player_wanted_level(self.id, wanted_level);
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
    fn get_widescreen(&self) -> bool {
        vcmp_func().get_player_option(self.id, VcmpPlayerOption::Widescreen)
    }

    #[setter]
    fn set_widescreen(&self, widescreen: bool) {
        vcmp_func().set_player_option(self.id, VcmpPlayerOption::Widescreen, widescreen);
    }

    #[getter]
    fn get_chat_tags_enabled(&self) -> bool {
        vcmp_func().get_player_option(self.id, VcmpPlayerOption::ChatTagsEnabled)
    }

    #[getter]
    fn get_bleeding(&self) -> bool {
        vcmp_func().get_player_option(self.id, VcmpPlayerOption::Bleeding)
    }

    #[setter]
    fn set_bleeding(&self, bleeding: bool) {
        vcmp_func().set_player_option(self.id, VcmpPlayerOption::Bleeding, bleeding);
    }

    fn is_world_compatible(&self, world: i32) -> bool {
        vcmp_func().is_player_world_compatible(self.id, world)
    }


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
