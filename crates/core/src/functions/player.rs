use std::ops::Add;

use pyo3::{
    Bound, PyResult, Python, pyclass, pymethods,
    types::{PyModule, PyModuleMethods},
};
use vcmp_bindings::{
    func::QueryVehicle,
    utils::{Color, Vectorf32},
};

use crate::py::streams::WriteStream;
use crate::py::types::RGBPy;
use crate::{
    functions::{object::ObjectPy, vehicle::VehiclePy},
    pool::{ENTITY_POOL, EntityPoolTrait},
    py::types::{EntityVectorType, VectorPy},
};
use vcmp_bindings::{func::PlayerMethods, options::VcmpPlayerOption, vcmp_func};

#[pyclass]
#[pyo3(name = "Player")]
#[derive(Debug, Clone, Copy)]
pub struct PlayerPy {
    id: i32,
    /*
        存储一些变量，用于检查更新的
    */
    pub last_health: f32,
    pub last_armour: f32,
    pub last_position: Vectorf32,
    pub last_weapon: i32,
    pub last_ammo: i32,
    /*
        存储是否加载
    */
    pub loaded: bool,
}

impl PlayerPy {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            last_health: 100.0,
            last_armour: 100.0,
            last_position: Vectorf32::default(),
            last_weapon: 0,
            last_ammo: 0,
            loaded: false,
        }
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
    fn position(&self) -> VectorPy {
        VectorPy::from((EntityVectorType::PlayerPosition, self.id))
    }

    pub fn _speed(&self) -> VectorPy {
        VectorPy::from((EntityVectorType::PlayerSpeed, self.id))
    }

    pub fn set_position(&self, position: Vectorf32) {
        let _ = vcmp_func().set_player_position(self.id, position);
    }

    pub fn message(&self, message: &str) {
        let _ = vcmp_func().send_client_message(self.id, Color::default(), message);
    }

    pub fn raw_message(&self, color: impl Into<Color>, message: &str) {
        let _ = vcmp_func().send_client_message(self.id, color.into(), message);
    }
    pub fn announce(&self, announce_type: i32, message: &str) {
        let _ = vcmp_func().send_announce(self.id, announce_type, message);
    }
}

#[pymethods]
impl PlayerPy {
    fn __hash__(&self) -> i32 {
        self.id
    }

    fn __eq__(&self, other: &PlayerPy) -> bool {
        self.id == other.id
    }

    #[getter]
    pub fn get_action(&self, py: Python<'_>) -> i32 {
        py.allow_threads(|| vcmp_func().get_player_action(self.id))
    }

    pub fn add_position(&mut self, py: Python<'_>, pos: VectorPy) {
        py.allow_threads(|| {
            let origin = self.position();
            let _ = origin.add(pos);
        })
    }

    pub fn add_speed(&mut self, py: Python<'_>, speed: VectorPy) {
        py.allow_threads(|| {
            let origin = self._speed();
            let _ = origin.add(speed);
        })
    }

    #[getter]
    pub fn get_admin(&self, py: Python<'_>) -> bool {
        py.allow_threads(|| vcmp_func().is_player_admin(self.id))
    }

    #[setter]
    pub fn set_admin(&self, py: Python<'_>, admin: bool) {
        py.allow_threads(|| vcmp_func().set_player_admin(self.id, admin))
    }

    #[getter]
    pub fn get_aim_direction(&self, py: Python<'_>) -> VectorPy {
        py.allow_threads(|| {
            let res = vcmp_func().get_player_aim_direction(self.id);
            if let Ok(pos) = res {
                VectorPy::from(pos)
            } else {
                VectorPy::default()
            }
        })
    }

    #[getter]
    pub fn get_aim_position(&self, py: Python<'_>) -> VectorPy {
        py.allow_threads(|| {
            let res = vcmp_func().get_player_aim_position(self.id);
            if let Ok(pos) = res {
                VectorPy::from(pos)
            } else {
                VectorPy::default()
            }
        })
    }

    #[getter]
    pub fn get_alpha(&self, py: Python<'_>) -> i32 {
        py.allow_threads(|| vcmp_func().get_player_alpha(self.id))
    }

    pub fn set_alpha(&self, py: Python<'_>, alpha: i32, fade_time: u32) {
        py.allow_threads(|| {
            let _ = vcmp_func().set_player_alpha(self.id, alpha, fade_time);
        })
    }

    #[getter]
    pub fn get_angle(&self, py: Python<'_>) -> f32 {
        py.allow_threads(|| vcmp_func().get_player_angle(self.id))
    }

    #[setter]
    pub fn set_angle(&self, py: Python<'_>, angle: f32) {
        py.allow_threads(|| {
            let _ = vcmp_func().set_player_angle(self.id, angle);
        });
    }

    #[getter]
    pub fn get_armour(&self, py: Python<'_>) -> f32 {
        py.allow_threads(|| vcmp_func().get_player_armour(self.id))
    }

    #[setter]
    pub fn set_armour(&self, py: Python<'_>, armour: f32) {
        py.allow_threads(|| {
            let _ = vcmp_func().set_player_armour(self.id, armour);
        });
    }

    #[getter]
    pub fn get_away(&self, py: Python<'_>) -> bool {
        py.allow_threads(|| vcmp_func().is_player_away(self.id))
    }

    #[pyo3(signature = (message = None))]
    pub fn ban(&self, py: Python<'_>, message: Option<&str>) {
        py.allow_threads(|| {
            if let Some(message) = message {
                self.message(message);
            }
            vcmp_func().ban_player(self.id);
        })
    }

    #[getter]
    pub fn get_camera_locked(&self, py: Python<'_>) -> bool {
        py.allow_threads(|| vcmp_func().is_camera_locked(self.id))
    }

    #[getter]
    pub fn get_can_attack(&self, py: Python<'_>) -> bool {
        py.allow_threads(|| vcmp_func().get_player_option(self.id, VcmpPlayerOption::CanAttack))
    }

    #[setter]
    pub fn set_can_attack(&self, py: Python<'_>, can_attack: bool) {
        py.allow_threads(|| {
            vcmp_func().set_player_option(self.id, VcmpPlayerOption::CanAttack, can_attack)
        })
    }

    #[getter]
    pub fn get_cash(&self, py: Python<'_>) -> i32 {
        py.allow_threads(|| vcmp_func().get_player_money(self.id))
    }

    #[setter]
    pub fn set_cash(&self, py: Python<'_>, cash: i32) {
        py.allow_threads(|| {
            let _ = vcmp_func().set_player_money(self.id, cash);
        });
    }

    #[setter]
    pub fn set_chat_tags_enabled(&self, py: Python<'_>, chat_tags_enabled: bool) {
        py.allow_threads(|| {
            vcmp_func().set_player_option(
                self.id,
                VcmpPlayerOption::ChatTagsEnabled,
                chat_tags_enabled,
            )
        })
    }

    #[getter]
    pub fn get_class_id(&self, py: Python<'_>) -> i32 {
        py.allow_threads(|| vcmp_func().get_player_class(self.id))
    }

    pub fn clear_weapons(&self, py: Python<'_>) {
        py.allow_threads(|| {
            let _ = vcmp_func().remove_all_weapons(self.id);
        });
    }

    pub fn disarm(&self, py: Python<'_>) {
        self.clear_weapons(py)
    }

    #[getter]
    pub fn get_color(&self, py: Python<'_>) -> RGBPy {
        py.allow_threads(|| RGBPy::from(vcmp_func().get_player_color(self.id)))
    }

    #[setter]
    pub fn set_color(&self, py: Python<'_>, value: RGBPy) {
        py.allow_threads(|| {
            let _ = vcmp_func().set_player_color(self.id, value.into());
        });
    }

    #[getter]
    pub fn get_controllable(&self, py: Python<'_>) -> bool {
        py.allow_threads(|| vcmp_func().get_player_option(self.id, VcmpPlayerOption::Controllable))
    }

    #[setter]
    pub fn set_controllable(&self, py: Python<'_>, controllable: bool) {
        py.allow_threads(|| {
            vcmp_func().set_player_option(self.id, VcmpPlayerOption::Controllable, controllable)
        })
    }

    #[getter]
    pub fn get_frozen(&self, py: Python<'_>) -> bool {
        !self.get_controllable(py)
    }

    #[setter]
    pub fn set_frozen(&self, py: Python<'_>, frozen: bool) {
        self.set_controllable(py, !frozen)
    }

    #[getter]
    pub fn crouching(&self, py: Python<'_>) -> bool {
        py.allow_threads(|| vcmp_func().is_player_crouching(self.id))
    }

    #[getter]
    pub fn get_drive_by(&self, py: Python<'_>) -> bool {
        py.allow_threads(|| vcmp_func().get_player_option(self.id, VcmpPlayerOption::DriveBy))
    }

    #[setter]
    pub fn set_drive_by(&self, py: Python<'_>, drive_by: bool) {
        py.allow_threads(|| {
            vcmp_func().set_player_option(self.id, VcmpPlayerOption::DriveBy, drive_by)
        })
    }

    #[getter]
    pub fn get_drunk_effects(&self, py: Python<'_>) -> bool {
        py.allow_threads(|| {
            vcmp_func().get_player_option(self.id, VcmpPlayerOption::DrunkEffectsDeprecated)
        })
    }

    #[setter]
    pub fn set_drunk_effects(&self, py: Python<'_>, drunk_effects: bool) {
        py.allow_threads(|| {
            vcmp_func().set_player_option(
                self.id,
                VcmpPlayerOption::DrunkEffectsDeprecated,
                drunk_effects,
            )
        })
    }

    #[getter]
    pub fn get_fps(&self, py: Python<'_>) -> f64 {
        py.allow_threads(|| vcmp_func().get_player_fps(self.id))
    }

    #[getter]
    pub fn get_game_keys(&self, py: Python<'_>) -> u32 {
        py.allow_threads(|| vcmp_func().get_player_game_keys(self.id))
    }

    #[getter]
    pub fn get_weapon(&self, py: Python<'_>) -> i32 {
        py.allow_threads(|| vcmp_func().get_player_weapon(self.id))
    }

    #[getter]
    pub fn get_weapon_ammo(&self, py: Python<'_>) -> i32 {
        py.allow_threads(|| vcmp_func().get_player_weapon_ammo(self.id))
    }

    pub fn get_weapon_ammo_at_slot(&self, py: Python<'_>, slot: i32) -> i32 {
        py.allow_threads(|| vcmp_func().get_player_ammo_at_slot(self.id, slot))
    }

    pub fn get_weapon_at_slot(&self, py: Python<'_>, slot: i32) -> i32 {
        py.allow_threads(|| vcmp_func().get_player_weapon_at_slot(self.id, slot))
    }

    #[getter]
    pub fn get_weapon_slot(&self, py: Python<'_>) -> i32 {
        py.allow_threads(|| vcmp_func().get_player_weapon_slot(self.id))
    }

    pub fn give_weapon(&self, py: Python<'_>, weapon: i32, ammo: i32) {
        py.allow_threads(|| {
            let _ = vcmp_func().give_player_weapon(self.id, weapon, ammo);
        });
    }

    #[getter]
    pub fn get_green_scanlines(&self, py: Python<'_>) -> bool {
        py.allow_threads(|| {
            vcmp_func().get_player_option(self.id, VcmpPlayerOption::GreenScanlines)
        })
    }

    #[setter]
    pub fn set_green_scanlines(&self, py: Python<'_>, green_scanlines: bool) {
        py.allow_threads(|| {
            vcmp_func().set_player_option(
                self.id,
                VcmpPlayerOption::GreenScanlines,
                green_scanlines,
            )
        })
    }

    #[getter]
    pub fn get_has_marker(&self, py: Python<'_>) -> bool {
        py.allow_threads(|| vcmp_func().get_player_option(self.id, VcmpPlayerOption::HasMarker))
    }

    #[setter]
    pub fn set_has_marker(&self, py: Python<'_>, has_marker: bool) {
        py.allow_threads(|| {
            vcmp_func().set_player_option(self.id, VcmpPlayerOption::HasMarker, has_marker)
        })
    }

    #[getter]
    pub fn get_health(&self, py: Python<'_>) -> f32 {
        py.allow_threads(|| vcmp_func().get_player_health(self.id))
    }

    #[setter]
    pub fn set_health(&self, py: Python<'_>, health: f32) {
        py.allow_threads(|| {
            let _ = vcmp_func().set_player_health(self.id, health);
        })
    }

    #[getter]
    pub fn get_id(&self) -> i32 {
        self.id
    }

    #[getter]
    pub fn get_immunity(&self, py: Python<'_>) -> u32 {
        py.allow_threads(|| vcmp_func().get_player_immunity(self.id))
    }

    #[setter]
    pub fn set_immunity(&self, py: Python<'_>, flags: u32) {
        py.allow_threads(|| {
            let _ = vcmp_func().set_player_immunity(self.id, flags);
        });
    }

    #[getter]
    pub fn get_ip(&self, py: Python<'_>) -> String {
        py.allow_threads(|| vcmp_func().get_player_ip(self.id))
    }

    #[getter]
    pub fn is_alive(&self, py: Python<'_>) -> bool {
        py.allow_threads(|| vcmp_func().is_player_connected(self.id))
    }

    pub fn is_streamed_for_target(&self, py: Python<'_>, player: i32) -> bool {
        py.allow_threads(|| vcmp_func().is_player_streamed_for_target(self.id, player))
    }

    #[getter]
    pub fn get_key(&self, py: Python<'_>) -> u32 {
        py.allow_threads(|| vcmp_func().get_player_key(self.id))
    }

    #[pyo3(signature = (message = None))]
    pub fn kick(&self, py: Python<'_>, message: Option<&str>) {
        py.allow_threads(|| {
            if let Some(message) = message {
                self.message(message);
            }
            vcmp_func().kick_player(self.id);
        })
    }

    pub fn kill(&self, py: Python<'_>) {
        py.allow_threads(|| {
            let _ = vcmp_func().kill_player(self.id);
        });
    }

    #[getter]
    pub fn get_name(&self, py: Python<'_>) -> String {
        py.allow_threads(|| vcmp_func().get_player_name(self.id))
    }

    #[setter]
    pub fn set_name(&self, py: Python<'_>, name: String) {
        py.allow_threads(|| vcmp_func().set_player_name(self.id, name.as_str()))
    }

    #[getter]
    pub fn on_fire(&self, py: Python<'_>) -> bool {
        py.allow_threads(|| vcmp_func().is_player_on_fire(self.id))
    }

    #[getter]
    pub fn get_ping(&self, py: Python<'_>) -> i32 {
        py.allow_threads(|| vcmp_func().get_player_ping(self.id))
    }

    pub fn play_animation(&self, py: Python<'_>, group_id: i32, animation_id: i32) {
        py.allow_threads(|| {
            let _ = vcmp_func().set_player_animation(self.id, group_id, animation_id);
        });
    }

    #[pyo3(signature = (sound, position = None))]
    pub fn play_sound(&self, py: Python<'_>, sound: i32, position: Option<VectorPy>) {
        py.allow_threads(|| {
            let pos = position
                .map(Vectorf32::from)
                .unwrap_or_else(|| Vectorf32::from((f32::NAN, f32::NAN, f32::NAN)));
            vcmp_func().play_sound_for_player(self.id, sound, Some(pos));
        })
    }

    #[getter]
    pub fn get_position(&self) -> VectorPy {
        self.position()
    }

    #[setter]
    #[pyo3(name = "position")]
    pub fn py_set_position(&self, py: Python<'_>, position: VectorPy) {
        py.allow_threads(|| {
            self.set_position(position.into());
        })
    }

    pub fn redirect(
        &self,
        py: Python<'_>,
        ip: &str,
        port: u32,
        nick: &str,
        password: &str,
        user_password: &str,
    ) {
        py.allow_threads(|| {
            let _ = vcmp_func().redirect_player_to_server(
                self.id,
                ip,
                port,
                nick,
                password,
                user_password,
            );
        })
    }

    pub fn remove_weapon(&self, py: Python<'_>, weapon: i32) {
        py.allow_threads(|| {
            let _ = vcmp_func().remove_player_weapon(self.id, weapon);
        });
    }

    pub fn request_module_list(&self, py: Python<'_>) {
        py.allow_threads(|| {
            let _ = vcmp_func().get_player_module_list(self.id);
        });
    }

    pub fn restore_camera(&self, py: Python<'_>) {
        py.allow_threads(|| {
            let _ = vcmp_func().restore_camera(self.id);
        });
    }

    #[getter]
    pub fn get_score(&self, py: Python<'_>) -> i32 {
        py.allow_threads(|| vcmp_func().get_player_score(self.id))
    }

    #[setter]
    pub fn set_score(&self, py: Python<'_>, score: i32) {
        py.allow_threads(|| {
            let _ = vcmp_func().set_player_score(self.id, score);
        });
    }

    #[getter]
    pub fn get_sec_world(&self, py: Python<'_>) -> i32 {
        py.allow_threads(|| vcmp_func().get_player_secondary_world(self.id))
    }

    #[setter]
    pub fn set_sec_world(&self, py: Python<'_>, sec_world: i32) {
        py.allow_threads(|| {
            let _ = vcmp_func().set_player_secondary_world(self.id, sec_world);
        });
    }

    #[getter]
    pub fn get_world(&self, py: Python<'_>) -> i32 {
        py.allow_threads(|| vcmp_func().get_player_world(self.id))
    }

    #[setter]
    pub fn set_world(&self, py: Python<'_>, world: i32) {
        py.allow_threads(|| {
            let _ = vcmp_func().set_player_world(self.id, world);
        });
    }

    pub fn select(&self, py: Python<'_>) {
        py.allow_threads(|| {
            let _ = vcmp_func().force_player_select(self.id);
        });
    }

    pub fn send_data(&self, py: Python<'_>, data: WriteStream) {
        py.allow_threads(|| {
            let _ = vcmp_func().send_client_script_data(self.id, data.raw_buffer().as_slice());
        });
    }

    pub fn send_raw_message(&self, py: Python<'_>, color: RGBPy, message: &str) {
        py.allow_threads(|| {
            let _ = vcmp_func().send_client_message(self.id, color.into(), message);
        })
    }

    pub fn send_message(&self, py: Python<'_>, message: &str) {
        self.send_raw_message(py, RGBPy::from_rgb(0xFFFFFF, None), message)
    }

    pub fn send_announce(&self, py: Python<'_>, announce_type: i32, message: &str) {
        py.allow_threads(|| {
            let _ = vcmp_func().send_announce(self.id, announce_type, message);
        });
    }

    #[getter]
    pub fn get_unique_world(&self, py: Python<'_>) -> i32 {
        py.allow_threads(|| vcmp_func().get_player_unique_world(self.id))
    }

    pub fn set_camera_position(&self, py: Python<'_>, position: VectorPy, look_at: VectorPy) {
        py.allow_threads(|| {
            let _ = vcmp_func().set_camera_position(
                self.id,
                position.get_entity_pos(),
                look_at.get_entity_pos(),
            );
        })
    }

    #[pyo3(signature = (position, look_yaw, look_pitch, range = 0.5))]
    pub fn set_camera(
        &self,
        py: Python<'_>,
        position: VectorPy,
        look_yaw: f32,
        look_pitch: f32,
        range: Option<f32>,
    ) {
        py.allow_threads(|| {
            let mut look = Vectorf32::default();
            let range = range.unwrap_or(0.5);
            look.x = look_yaw.cos() * range;
            look.y = look_yaw.sin() * range;
            look.z = look_pitch.sin() * range;
            let py_look = VectorPy::from(look);
            let origin = position;
            let camera_position = position;
            let _ = vcmp_func().set_camera_position(
                self.id,
                origin.get_entity_pos(),
                (py_look + camera_position).get_entity_pos(),
            );
        });
    }

    pub fn set_vehicle_slot(&self, py: Python<'_>, vehicle: Option<VehiclePy>, slot: i32) {
        py.allow_threads(|| {
            if vehicle.is_none() {
                let _ = vcmp_func().remove_player_from_vehicle(self.id);
                return;
            }

            let _ =
                vcmp_func().put_player_in_vehicle(self.id, vehicle.unwrap().get_id(), slot, 1, 0);
        })
    }

    pub fn set_weapon(&self, py: Python<'_>, weapon: i32, ammo: i32) {
        py.allow_threads(|| {
            let _ = vcmp_func().set_player_weapon(self.id, weapon, ammo);
        });
    }

    pub fn set_weapon_slot(&self, py: Python<'_>, slot: i32) {
        py.allow_threads(|| {
            let _ = vcmp_func().set_player_weapon_slot(self.id, slot);
        });
    }

    #[getter]
    pub fn get_show_markers(&self, py: Python<'_>) -> bool {
        py.allow_threads(|| vcmp_func().get_player_option(self.id, VcmpPlayerOption::ShowMarkers))
    }

    #[setter]
    pub fn set_show_markers(&self, py: Python<'_>, show_markers: bool) {
        py.allow_threads(|| {
            vcmp_func().set_player_option(self.id, VcmpPlayerOption::ShowMarkers, show_markers)
        })
    }

    #[getter]
    pub fn get_skin(&self, py: Python<'_>) -> i32 {
        py.allow_threads(|| vcmp_func().get_player_skin(self.id))
    }

    #[setter]
    pub fn set_skin(&self, py: Python<'_>, skin: i32) {
        py.allow_threads(|| {
            let _ = vcmp_func().set_player_skin(self.id, skin);
        });
    }

    pub fn spawn(&self, py: Python<'_>) {
        py.allow_threads(|| {
            let _ = vcmp_func().spawn_player(self.id);
        });
    }

    #[getter]
    pub fn get_spawned(&self) -> bool {
        vcmp_func().is_player_spawned(self.id)
    }

    #[getter]
    pub fn get_spectate_target(&self, py: Python<'_>) -> Option<PlayerPy> {
        py.allow_threads(|| {
            let pool = ENTITY_POOL.lock().unwrap();
            let id = vcmp_func().get_player_spectate_target(self.id);
            pool.get_player(id).copied()
        })
    }

    #[setter]
    pub fn set_spectate_target(&self, py: Python<'_>, player: Option<PlayerPy>) {
        py.allow_threads(|| {
            let _ = vcmp_func()
                .set_player_spectate_target(self.id, player.map(|p| p.get_id()).unwrap_or(-1));
        })
    }

    #[getter]
    pub fn get_speed(&self, py: Python<'_>) -> VectorPy {
        py.allow_threads(|| self._speed())
    }

    #[setter]
    pub fn set_speed(&self, py: Python<'_>, speed: VectorPy) {
        py.allow_threads(|| {
            let _ = vcmp_func().set_player_speed(self.id, speed.get_entity_pos());
        });
    }

    #[getter]
    pub fn get_standing_on_object(&self, py: Python<'_>) -> Option<ObjectPy> {
        py.allow_threads(|| {
            let id = vcmp_func().get_player_standing_on_object(self.id);
            let pool = ENTITY_POOL.lock().unwrap();
            pool.get_object(id).copied()
        })
    }

    #[getter]
    pub fn get_standing_vehicle(&self, py: Python<'_>) -> Option<VehiclePy> {
        py.allow_threads(|| {
            let id = vcmp_func().get_player_standing_on_vehicle(self.id);
            let pool = ENTITY_POOL.lock().unwrap();
            pool.get_vehicle(id).copied()
        })
    }

    #[getter]
    pub fn get_state(&self, py: Python<'_>) -> i32 {
        py.allow_threads(|| vcmp_func().get_player_state(self.id) as i32)
    }

    #[getter]
    pub fn get_team(&self, py: Python<'_>) -> i32 {
        py.allow_threads(|| vcmp_func().get_player_team(self.id))
    }

    #[setter]
    pub fn set_team(&self, py: Python<'_>, team: i32) {
        py.allow_threads(|| {
            let _ = vcmp_func().set_player_team(self.id, team);
        });
    }

    #[getter]
    pub fn is_typing(&self, py: Python<'_>) -> bool {
        py.allow_threads(|| vcmp_func().is_player_typing(self.id))
    }

    #[getter]
    pub fn get_unique_id(&self, py: Python<'_>) -> String {
        py.allow_threads(|| vcmp_func().get_player_uid(self.id))
    }

    #[getter]
    pub fn get_unique_id2(&self, py: Python<'_>) -> String {
        py.allow_threads(|| vcmp_func().get_player_uid2(self.id))
    }

    #[getter]
    pub fn get_vehicle(&self, py: Python<'_>) -> Option<VehiclePy> {
        py.allow_threads(|| {
            let vehicle_id = vcmp_func().get_player_vehicle_id(self.id);
            let pool = ENTITY_POOL.lock().unwrap();
            pool.get_vehicle(vehicle_id).copied()
        })
    }

    #[setter]
    pub fn set_vehicle(&self, py: Python<'_>, vehicle: Option<VehiclePy>) {
        py.allow_threads(|| {
            let mut id = None;
            if let Some(vehicle) = vehicle {
                id = Some(vehicle.get_id());
            }
            if let Some(id) = id
                && vcmp_func().is_vehicle_alive(id)
            {
                let origin = vcmp_func().get_player_vehicle_id(self.id);
                if origin != id {
                    let _ = vcmp_func().remove_player_from_vehicle(self.id);
                    let _ = vcmp_func().put_player_in_vehicle(self.id, id, 0, 0, 1);
                }
            } else {
                let _ = vcmp_func().remove_player_from_vehicle(self.id);
            }
        })
    }

    #[getter]
    pub fn get_vehicle_status(&self, py: Python<'_>) -> i32 {
        py.allow_threads(|| vcmp_func().get_player_in_vehicle_status(self.id) as i32)
    }

    #[getter]
    pub fn get_wanted_level(&self, py: Python<'_>) -> i32 {
        py.allow_threads(|| vcmp_func().get_player_wanted_level(self.id))
    }

    #[setter]
    pub fn set_wanted_level(&self, py: Python<'_>, wanted_level: i32) {
        py.allow_threads(|| {
            let _ = vcmp_func().set_player_wanted_level(self.id, wanted_level);
        });
    }

    #[getter]
    pub fn get_white_scanlines(&self, py: Python<'_>) -> bool {
        py.allow_threads(|| {
            vcmp_func().get_player_option(self.id, VcmpPlayerOption::WhiteScanlines)
        })
    }

    #[setter]
    pub fn set_white_scanlines(&self, py: Python<'_>, white_scanlines: bool) {
        py.allow_threads(|| {
            vcmp_func().set_player_option(
                self.id,
                VcmpPlayerOption::WhiteScanlines,
                white_scanlines,
            )
        })
    }

    #[getter]
    pub fn get_widescreen(&self, py: Python<'_>) -> bool {
        py.allow_threads(|| vcmp_func().get_player_option(self.id, VcmpPlayerOption::Widescreen))
    }

    #[setter]
    pub fn set_widescreen(&self, py: Python<'_>, widescreen: bool) {
        py.allow_threads(|| {
            vcmp_func().set_player_option(self.id, VcmpPlayerOption::Widescreen, widescreen)
        })
    }

    #[getter]
    pub fn get_chat_tags_enabled(&self, py: Python<'_>) -> bool {
        py.allow_threads(|| {
            vcmp_func().get_player_option(self.id, VcmpPlayerOption::ChatTagsEnabled)
        })
    }

    #[getter]
    pub fn get_bleeding(&self, py: Python<'_>) -> bool {
        py.allow_threads(|| vcmp_func().get_player_option(self.id, VcmpPlayerOption::Bleeding))
    }

    #[setter]
    pub fn set_bleeding(&self, py: Python<'_>, bleeding: bool) {
        py.allow_threads(|| {
            vcmp_func().set_player_option(self.id, VcmpPlayerOption::Bleeding, bleeding)
        })
    }

    pub fn is_world_compatible(&self, py: Python<'_>, world: i32) -> bool {
        py.allow_threads(|| vcmp_func().is_player_world_compatible(self.id, world))
    }

    #[getter]
    pub fn drunk_handling(&self, py: Python<'_>) -> u32 {
        py.allow_threads(|| vcmp_func().get_player_drunk_handling(self.id))
    }

    #[setter]
    pub fn set_drunk_handling(&self, py: Python<'_>, drunk_level: u32) {
        py.allow_threads(|| {
            let _ = vcmp_func().set_player_drunk_handling(self.id, drunk_level);
        });
    }

    #[getter]
    pub fn drunk_visuals(&self, py: Python<'_>) -> bool {
        py.allow_threads(|| vcmp_func().get_player_drunk_visuals(self.id))
    }

    #[setter]
    pub fn set_drunk_visuals(&self, py: Python<'_>, drunk: bool) {
        py.allow_threads(|| {
            let _ = vcmp_func().set_player_drunk_visuals(self.id, drunk);
        });
    }

    pub fn set_3d_arrow_for_target(&self, py: Python<'_>, target: PlayerPy, show: bool) {
        py.allow_threads(|| {
            let _ = vcmp_func().set_player_3d_arrow_for_target(self.id, target.get_id(), show);
        });
    }

    pub fn is_3d_arrow_show_for_target(&self, py: Python<'_>, target: PlayerPy) -> bool {
        py.allow_threads(|| vcmp_func().is_player_3d_arrow_for_target(self.id, target.get_id()))
    }

    pub fn interpolate_camera_look_at(&self, py: Python<'_>, look: VectorPy, time: u32) {
        py.allow_threads(|| {
            let _ = vcmp_func().interpolate_camera_look_at(self.id, look.into(), time);
        });
    }
}
pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PlayerPy>()?;
    Ok(())
}
