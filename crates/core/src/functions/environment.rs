use pyo3::{
    Bound, PyResult, Python, pyfunction,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction,
};
use vcmp_bindings::{
    func::{
        EnvironmentMethods, QueryEnvironmentOption, SetEnvironmentOption,
        environment::{QueryEnvironmentWorld, SetEnvironmentWorld},
    },
    utils::Vectorf32,
    vcmp_func,
};

use crate::py::types::{RGBPy, VectorPy, WastedSettingsPy};

#[pyfunction]
pub fn set_taxi_boost_jump(toggle: bool) {
    vcmp_func().set_taxi_boost_jump(toggle);
}

#[pyfunction]
pub fn get_taxi_boost_jump() -> bool {
    vcmp_func().get_taxi_boost_jump()
}

#[pyfunction]
pub fn set_drive_on_water(toggle: bool) {
    vcmp_func().set_drive_on_water(toggle);
}

#[pyfunction]
pub fn get_drive_on_water() -> bool {
    vcmp_func().get_drive_on_water()
}

#[pyfunction]
pub fn set_fast_switch(toggle: bool) {
    vcmp_func().set_fast_switch(toggle);
}

#[pyfunction]
pub fn get_fast_switch() -> bool {
    vcmp_func().get_fast_switch()
}

#[pyfunction]
pub fn set_friendly_fire(toggle: bool) {
    vcmp_func().set_friendly_fire(toggle);
}

#[pyfunction]
pub fn get_friendly_fire() -> bool {
    vcmp_func().get_friendly_fire()
}

#[pyfunction]
pub fn set_disable_drive_by(toggle: bool) {
    vcmp_func().set_disable_drive_by(toggle);
}

#[pyfunction]
pub fn get_disable_drive_by() -> bool {
    vcmp_func().get_disable_drive_by()
}

#[pyfunction]
pub fn set_perfect_handling(toggle: bool) {
    vcmp_func().set_perfect_handling(toggle);
}

#[pyfunction]
pub fn get_perfect_handling() -> bool {
    vcmp_func().get_perfect_handling()
}

#[pyfunction]
pub fn set_flying_cars(toggle: bool) {
    vcmp_func().set_flying_cars(toggle);
}

#[pyfunction]
pub fn get_flying_cars() -> bool {
    vcmp_func().get_flying_cars()
}

#[pyfunction]
pub fn set_jump_switch(toggle: bool) {
    vcmp_func().set_jump_switch(toggle);
}

#[pyfunction]
pub fn get_jump_switch() -> bool {
    vcmp_func().get_jump_switch()
}

#[pyfunction]
pub fn set_show_markers(toggle: bool) {
    vcmp_func().set_show_markers(toggle);
}

#[pyfunction]
pub fn get_show_markers() -> bool {
    vcmp_func().get_show_markers()
}

#[pyfunction]
pub fn set_only_show_team_markers(toggle: bool) {
    vcmp_func().set_only_show_team_markers(toggle);
}

#[pyfunction]
pub fn get_only_show_team_markers() -> bool {
    vcmp_func().get_only_show_team_markers()
}

#[pyfunction]
pub fn set_stunt_bike(toggle: bool) {
    vcmp_func().set_stunt_bike(toggle);
}

#[pyfunction]
pub fn get_stunt_bike() -> bool {
    vcmp_func().get_stunt_bike()
}

#[pyfunction]
pub fn set_shoot_in_air(toggle: bool) {
    vcmp_func().set_shoot_in_air(toggle);
}

#[pyfunction]
pub fn get_shoot_in_air() -> bool {
    vcmp_func().get_shoot_in_air()
}

#[pyfunction]
pub fn set_show_name_tags(toggle: bool) {
    vcmp_func().set_show_name_tags(toggle);
}

#[pyfunction]
pub fn get_show_name_tags() -> bool {
    vcmp_func().get_show_name_tags()
}

#[pyfunction]
pub fn set_join_messages(toggle: bool) {
    vcmp_func().set_join_messages(toggle);
}

#[pyfunction]
pub fn get_join_messages() -> bool {
    vcmp_func().get_join_messages()
}

#[pyfunction]
pub fn set_death_messages(toggle: bool) {
    vcmp_func().set_death_messages(toggle);
}

#[pyfunction]
pub fn get_death_messages() -> bool {
    vcmp_func().get_death_messages()
}

#[pyfunction]
pub fn set_chat_tags_enabled(toggle: bool) {
    vcmp_func().set_chat_tags_enabled(toggle);
}

#[pyfunction]
pub fn get_chat_tags_enabled() -> bool {
    vcmp_func().get_chat_tags_enabled()
}

#[pyfunction]
pub fn set_use_classes(toggle: bool) {
    vcmp_func().set_use_classes(toggle);
}

#[pyfunction]
pub fn get_use_classes() -> bool {
    vcmp_func().get_use_classes()
}

#[pyfunction]
pub fn set_wall_glitch(toggle: bool) {
    vcmp_func().set_wall_glitch(toggle);
}

#[pyfunction]
pub fn get_wall_glitch() -> bool {
    vcmp_func().get_wall_glitch()
}

#[pyfunction]
pub fn set_disable_backface_culling(toggle: bool) {
    vcmp_func().set_disable_backface_culling(toggle);
}

#[pyfunction]
pub fn get_disable_backface_culling() -> bool {
    vcmp_func().get_disable_backface_culling()
}

#[pyfunction]
pub fn set_disable_heli_blade_damage(toggle: bool) {
    vcmp_func().set_disable_heli_blade_damage(toggle);
}

#[pyfunction]
pub fn get_disable_heli_blade_damage() -> bool {
    vcmp_func().get_disable_heli_blade_damage()
}

#[pyfunction]
pub fn get_time() -> i32 {
    vcmp_func().get_time()
}

#[pyfunction]
pub fn set_time(time: i32) {
    vcmp_func().set_time(time);
}

#[pyfunction]
pub fn get_hour() -> i32 {
    vcmp_func().get_hour()
}

#[pyfunction]
pub fn set_hour(hour: i32) {
    vcmp_func().set_hour(hour);
}

#[pyfunction]
pub fn get_minute() -> i32 {
    vcmp_func().get_minute()
}

#[pyfunction]
pub fn set_minute(minute: i32) {
    vcmp_func().set_minute(minute);
}

#[pyfunction]
pub fn get_weather() -> i32 {
    vcmp_func().get_weather()
}

#[pyfunction]
pub fn set_weather(weather: i32) {
    vcmp_func().set_weather(weather);
}

#[pyfunction]
pub fn get_time_rate() -> i32 {
    vcmp_func().get_time_rate()
}

#[pyfunction]
pub fn set_time_rate(time_rate: i32) {
    vcmp_func().set_time_rate(time_rate);
}

#[pyfunction]
pub fn get_gravity() -> f32 {
    vcmp_func().get_gravity()
}

#[pyfunction]
pub fn set_gravity(gravity: f32) {
    vcmp_func().set_gravity(gravity);
}

#[pyfunction]
pub fn get_gamespeed() -> f32 {
    vcmp_func().get_gamespeed()
}

#[pyfunction]
pub fn set_gamespeed(gamespeed: f32) {
    vcmp_func().set_gamespeed(gamespeed);
}

#[pyfunction]
pub fn get_water_level() -> f32 {
    vcmp_func().get_water_level()
}

#[pyfunction]
pub fn set_water_level(water_level: f32) {
    vcmp_func().set_water_level(water_level);
}

#[pyfunction]
pub fn get_max_flight_altitude() -> f32 {
    vcmp_func().get_maximum_flight_altitude()
}

#[pyfunction]
pub fn set_max_flight_altitude(max_flight_altitude: f32) {
    vcmp_func().set_maximum_flight_altitude(max_flight_altitude);
}

#[pyfunction]
pub fn set_kill_command_delay(delay: i32) {
    vcmp_func().set_kill_command_delay(delay);
}

#[pyfunction]
pub fn get_kill_command_delay() -> i32 {
    vcmp_func().get_kill_command_delay()
}

#[pyfunction]
pub fn disable_kill_command() {
    set_kill_command_delay(0x7FFFFFFF);
}

#[pyfunction]
pub fn enable_kill_command() {
    set_kill_command_delay(0)
}

#[pyfunction]
pub fn get_vehicles_forced_respawn_height() -> f32 {
    vcmp_func().get_vehicles_forced_respawn_height()
}

#[pyfunction]
pub fn set_vehicles_forced_respawn_height(height: f32) {
    vcmp_func().set_vehicles_forced_respawn_height(height);
}

#[pyfunction]
pub fn add_player_class(
    team: i32,
    color: RGBPy,
    skin: i32,
    pos: VectorPy,
    angle: f32,
    weapon: Option<i32>,
    ammo: Option<i32>,
    weapon1: Option<i32>,
    ammo1: Option<i32>,
    weapon2: Option<i32>,
    ammo2: Option<i32>,
) {
    vcmp_func().add_player_class(
        team,
        color.into(),
        skin,
        pos.into(),
        angle,
        Some((weapon.unwrap_or_default(), ammo.unwrap_or_default())),
        Some((weapon1.unwrap_or_default(), ammo1.unwrap_or_default())),
        Some((weapon2.unwrap_or_default(), ammo2.unwrap_or_default())),
    );
}

#[pyfunction]
pub fn set_spawn_player_position(position: VectorPy) {
    vcmp_func().set_spawn_player_position(position.into());
}

#[pyfunction]
pub fn set_spawn_camera_position(position: VectorPy) {
    vcmp_func().set_spawn_camera_position(position.into());
}

#[pyfunction]
pub fn set_spawn_camera_look_at(position: VectorPy) {
    vcmp_func().set_spawn_camera_look_at(position.into());
}

#[pyfunction]
pub fn set_spawn_camera(position: VectorPy, look_yaw: f32, look_pitch: f32, range: Option<f32>) {
    let mut look = Vectorf32::default();
    let range = range.unwrap_or(0.5);
    look.x = (look_yaw.cos() * range) as f32;
    look.y = (look_yaw.sin() * range) as f32;
    look.z = (look_pitch.sin() * range) as f32;
    let py_look = VectorPy::from(look);
    let origin = position.clone();
    let camera_position = position.clone();
    set_spawn_camera_position(origin);
    set_spawn_camera_look_at(py_look + camera_position);
}

#[pyfunction]
pub fn set_fall_timer(time: u16) {
    vcmp_func().set_fall_timer(time);
}

#[pyfunction]
pub fn get_fall_timer() -> u16 {
    vcmp_func().get_fall_timer()
}

#[pyfunction]
pub fn get_wasted_settings() -> WastedSettingsPy {
    WastedSettingsPy::from(vcmp_func().get_wasted_settings())
}

#[pyfunction]
pub fn set_wasted_settings(settings: WastedSettingsPy) {
    vcmp_func().set_wasted_settings(settings.into());
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(set_taxi_boost_jump, m)?)?;
    m.add_function(wrap_pyfunction!(get_taxi_boost_jump, m)?)?;
    m.add_function(wrap_pyfunction!(set_drive_on_water, m)?)?;
    m.add_function(wrap_pyfunction!(get_drive_on_water, m)?)?;
    m.add_function(wrap_pyfunction!(set_fast_switch, m)?)?;
    m.add_function(wrap_pyfunction!(get_fast_switch, m)?)?;
    m.add_function(wrap_pyfunction!(set_friendly_fire, m)?)?;
    m.add_function(wrap_pyfunction!(get_friendly_fire, m)?)?;
    m.add_function(wrap_pyfunction!(set_disable_drive_by, m)?)?;
    m.add_function(wrap_pyfunction!(get_disable_drive_by, m)?)?;
    m.add_function(wrap_pyfunction!(set_perfect_handling, m)?)?;
    m.add_function(wrap_pyfunction!(get_perfect_handling, m)?)?;
    m.add_function(wrap_pyfunction!(set_flying_cars, m)?)?;
    m.add_function(wrap_pyfunction!(get_flying_cars, m)?)?;
    m.add_function(wrap_pyfunction!(set_jump_switch, m)?)?;
    m.add_function(wrap_pyfunction!(get_jump_switch, m)?)?;
    m.add_function(wrap_pyfunction!(set_show_markers, m)?)?;
    m.add_function(wrap_pyfunction!(get_show_markers, m)?)?;
    m.add_function(wrap_pyfunction!(set_only_show_team_markers, m)?)?;
    m.add_function(wrap_pyfunction!(get_only_show_team_markers, m)?)?;
    m.add_function(wrap_pyfunction!(set_stunt_bike, m)?)?;
    m.add_function(wrap_pyfunction!(get_stunt_bike, m)?)?;
    m.add_function(wrap_pyfunction!(set_shoot_in_air, m)?)?;
    m.add_function(wrap_pyfunction!(get_shoot_in_air, m)?)?;
    m.add_function(wrap_pyfunction!(set_show_name_tags, m)?)?;
    m.add_function(wrap_pyfunction!(get_show_name_tags, m)?)?;
    m.add_function(wrap_pyfunction!(set_join_messages, m)?)?;
    m.add_function(wrap_pyfunction!(get_join_messages, m)?)?;
    m.add_function(wrap_pyfunction!(set_death_messages, m)?)?;
    m.add_function(wrap_pyfunction!(get_death_messages, m)?)?;
    m.add_function(wrap_pyfunction!(set_chat_tags_enabled, m)?)?;
    m.add_function(wrap_pyfunction!(get_chat_tags_enabled, m)?)?;
    m.add_function(wrap_pyfunction!(set_use_classes, m)?)?;
    m.add_function(wrap_pyfunction!(get_use_classes, m)?)?;
    m.add_function(wrap_pyfunction!(set_wall_glitch, m)?)?;
    m.add_function(wrap_pyfunction!(get_wall_glitch, m)?)?;
    m.add_function(wrap_pyfunction!(set_disable_backface_culling, m)?)?;
    m.add_function(wrap_pyfunction!(get_disable_backface_culling, m)?)?;
    m.add_function(wrap_pyfunction!(set_disable_heli_blade_damage, m)?)?;
    m.add_function(wrap_pyfunction!(get_disable_heli_blade_damage, m)?)?;
    m.add_function(wrap_pyfunction!(get_time, m)?)?;
    m.add_function(wrap_pyfunction!(set_time, m)?)?;
    m.add_function(wrap_pyfunction!(get_hour, m)?)?;
    m.add_function(wrap_pyfunction!(set_hour, m)?)?;
    m.add_function(wrap_pyfunction!(get_minute, m)?)?;
    m.add_function(wrap_pyfunction!(set_minute, m)?)?;
    m.add_function(wrap_pyfunction!(get_weather, m)?)?;
    m.add_function(wrap_pyfunction!(set_weather, m)?)?;
    m.add_function(wrap_pyfunction!(get_time_rate, m)?)?;
    m.add_function(wrap_pyfunction!(set_time_rate, m)?)?;
    m.add_function(wrap_pyfunction!(get_gravity, m)?)?;
    m.add_function(wrap_pyfunction!(set_gravity, m)?)?;
    m.add_function(wrap_pyfunction!(get_gamespeed, m)?)?;
    m.add_function(wrap_pyfunction!(set_gamespeed, m)?)?;
    m.add_function(wrap_pyfunction!(get_water_level, m)?)?;
    m.add_function(wrap_pyfunction!(set_water_level, m)?)?;
    m.add_function(wrap_pyfunction!(get_max_flight_altitude, m)?)?;
    m.add_function(wrap_pyfunction!(set_max_flight_altitude, m)?)?;
    m.add_function(wrap_pyfunction!(set_kill_command_delay, m)?)?;
    m.add_function(wrap_pyfunction!(get_kill_command_delay, m)?)?;
    m.add_function(wrap_pyfunction!(disable_kill_command, m)?)?;
    m.add_function(wrap_pyfunction!(enable_kill_command, m)?)?;
    m.add_function(wrap_pyfunction!(get_vehicles_forced_respawn_height, m)?)?;
    m.add_function(wrap_pyfunction!(set_vehicles_forced_respawn_height, m)?)?;
    m.add_function(wrap_pyfunction!(add_player_class, m)?)?;
    m.add_function(wrap_pyfunction!(set_spawn_player_position, m)?)?;
    m.add_function(wrap_pyfunction!(set_spawn_camera_position, m)?)?;
    m.add_function(wrap_pyfunction!(set_spawn_camera_look_at, m)?)?;
    m.add_function(wrap_pyfunction!(set_spawn_camera, m)?)?;
    m.add_function(wrap_pyfunction!(set_fall_timer, m)?)?;
    m.add_function(wrap_pyfunction!(get_fall_timer, m)?)?;
    m.add_function(wrap_pyfunction!(get_wasted_settings, m)?)?;
    m.add_function(wrap_pyfunction!(set_wasted_settings, m)?)?;
    Ok(())
}
