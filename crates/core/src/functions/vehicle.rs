use std::ops::Add as _;

use pyo3::{
    Bound, PyResult, Python, pyclass, pyfunction, pymethods,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction,
};
use vcmp_bindings::{
    func::{
        QueryVehicle, QueryVehicleOptions, SetVehicle, SetVehicleOptions, VehicleHandlingMethods,
        VehicleMethods,
    },
    vcmp_func,
};

use crate::{
    functions::player::PlayerPy,
    pool::{ENTITY_POOL, EntityPoolTrait},
    py::types::{EntityQuaternionType, EntityVectorType, QuaternionPy, VectorPy},
};

#[pyclass]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "Vehicle")]
pub struct VehiclePy {
    id: i32,
}

impl VehiclePy {
    pub fn new(id: i32) -> Self {
        Self { id }
    }

    pub fn _position(&self) -> VectorPy {
        VectorPy::from((EntityVectorType::VehiclePosition, self.id))
    }

    pub fn _speed(&self) -> VectorPy {
        VectorPy::from((EntityVectorType::VehicleSpeed, self.id))
    }

    pub fn _rel_speed(&self) -> VectorPy {
        VectorPy::from((EntityVectorType::VehicleRelSpeed, self.id))
    }

    pub fn _turn_speed(&self) -> VectorPy {
        VectorPy::from((EntityVectorType::VehicleTurnSpeed, self.id))
    }

    pub fn _rel_turn_speed(&self) -> VectorPy {
        VectorPy::from((EntityVectorType::VehicleRelTurnSpeed, self.id))
    }

    pub fn _rotation(&self) -> QuaternionPy {
        QuaternionPy::from((EntityQuaternionType::VehicleRotation, self.id))
    }

    pub fn _rotation_euler(&self) -> VectorPy {
        VectorPy::from((EntityVectorType::VehicleRotationEuler, self.id))
    }

    pub fn _spawn_position(&self) -> VectorPy {
        VectorPy::from((EntityVectorType::VehicleSpawnPosition, self.id))
    }

    pub fn _spawn_rotation(&self) -> QuaternionPy {
        QuaternionPy::from((EntityQuaternionType::VehicleSpawnRotation, self.id))
    }

    pub fn _spawn_rotation_euler(&self) -> VectorPy {
        VectorPy::from((EntityVectorType::VehicleSpawnRotationEuler, self.id))
    }
}

impl EntityPoolTrait for VehiclePy {
    fn entity_id(&self) -> crate::consts::EntityId {
        self.id
    }
    fn entity_pool_type() -> vcmp_bindings::options::VcmpEntityPool {
        vcmp_bindings::options::VcmpEntityPool::Vehicle
    }
}

impl From<i32> for VehiclePy {
    fn from(val: i32) -> Self {
        VehiclePy::new(val)
    }
}

#[pymethods]
impl VehiclePy {
    fn __hash__(&self) -> i32 {
        self.id
    }

    fn __eq__(&self, other: &VehiclePy) -> bool {
        self.id == other.id
    }

    #[getter]
    pub fn get_id(&self) -> i32 {
        self.id
    }

    fn add_position(&mut self, pos: VectorPy) {
        let origin = self._position();
        let _ = origin.add(pos);
    }

    fn add_relative_speed(&mut self, speed: VectorPy) {
        let origin = self._rel_speed();
        let _ = origin.add(speed);
    }

    fn add_relative_turn_speed(&mut self, speed: VectorPy) {
        let origin = self._rel_turn_speed();
        let _ = origin.add(speed);
    }

    fn add_rotation_euler(&mut self, rotation: VectorPy) {
        let origin = self._rotation_euler();
        let _ = origin.add(rotation);
    }

    fn add_rotation(&mut self, rotation: QuaternionPy) {
        let origin = self._rotation();
        let _ = origin.add(rotation);
    }

    fn add_spawn_position(&mut self, pos: VectorPy) {
        let origin = self._spawn_position();
        let _ = origin.add(pos);
    }

    fn add_spawn_rotation_euler(&mut self, rotation: VectorPy) {
        let origin = self._spawn_rotation_euler();
        let _ = origin.add(rotation);
    }

    fn add_spawn_rotation(&mut self, rotation: QuaternionPy) {
        let origin = self._spawn_rotation();
        let _ = origin.add(rotation);
    }

    fn add_speed(&mut self, speed: VectorPy) {
        let origin = self._speed();
        let _ = origin.add(speed);
    }

    fn add_turn_speed(&mut self, speed: VectorPy) {
        let origin = self._turn_speed();
        let _ = origin.add(speed);
    }

    #[getter]
    fn get_alarm(&self) -> bool {
        vcmp_func().get_vehicle_option_alarm(self.id)
    }

    #[setter]
    fn set_alarm(&mut self, alarm: bool) {
        let _ = vcmp_func().set_vehicle_option_alarm(self.id, alarm);
    }

    #[getter]
    fn get_color(&self) -> (i32, i32) {
        vcmp_func().get_vehicle_color(self.id)
    }

    #[setter]
    fn set_color(&mut self, color: (i32, i32)) {
        let _ = vcmp_func().set_vehicle_color(self.id, color.0, color.1);
    }

    #[getter]
    fn get_primary_color(&self) -> i32 {
        self.get_color().0
    }

    #[getter]
    fn get_secondary_color(&self) -> i32 {
        self.get_color().1
    }

    #[setter]
    fn set_primary_color(&mut self, color: i32) {
        self.set_color((color, self.get_secondary_color()));
    }

    #[setter]
    fn set_secondary_color(&mut self, color: i32) {
        self.set_color((self.get_primary_color(), color));
    }

    #[getter]
    fn get_damage(&self) -> u32 {
        vcmp_func().get_vehicle_damage_data(self.id)
    }

    #[setter]
    fn set_damage(&mut self, damage: u32) {
        let _ = vcmp_func().set_vehicle_damage_data(self.id, damage);
    }

    fn delete(&self) {
        #[cfg(debug_assertions)]
        println!("Deleting vehicle {}", self.id);
        let _ = vcmp_func().delete_vehicle(self.id);
    }

    #[getter]
    fn get_doors_locked(&self) -> bool {
        vcmp_func().get_vehicle_option_doors_locked(self.id)
    }

    #[setter]
    fn set_doors_locked(&mut self, doors_locked: bool) {
        let _ = vcmp_func().set_vehicle_option_doors_locked(self.id, doors_locked);
    }

    fn exists_handling_rule(&self, rule_index: i32) -> bool {
        vcmp_func().exists_inst_handling_rule(self.id, rule_index)
    }

    fn explode(&self) {
        let _ = vcmp_func().explode_vehicle(self.id);
    }

    fn fix(&mut self) {
        self.set_health(1000.0);
        self.set_damage(0);
        let mut lights_data = self.get_lights_data();
        lights_data &= 0xFFFFFF00;
        self.set_lights_data(lights_data);
    }

    fn get_handling_rule(&self, rule_index: i32) -> f64 {
        vcmp_func().get_inst_handling_rule(self.id, rule_index)
    }

    fn get_occupant(&self, seat: i32) -> i32 {
        vcmp_func().get_vehicle_occupant(self.id, seat)
    }

    fn get_part_status(&self, part: i32) -> bool {
        vcmp_func().get_vehicle_part_status(self.id, part)
    }

    fn get_tyre_status(&self, tyre: i32) -> bool {
        vcmp_func().get_vehicle_tyre_status(self.id, tyre)
    }

    #[getter]
    fn get_ghost(&self) -> bool {
        vcmp_func().get_vehicle_option_ghost(self.id)
    }

    #[setter]
    fn set_ghost(&mut self, ghost: bool) {
        let _ = vcmp_func().set_vehicle_option_ghost(self.id, ghost);
    }

    #[getter]
    fn get_health(&self) -> f32 {
        vcmp_func().get_vehicle_health(self.id)
    }

    #[setter]
    fn set_health(&mut self, health: f32) {
        let _ = vcmp_func().set_vehicle_health(self.id, health);
    }

    #[getter]
    fn get_idle_respawn_timer(&self) -> u32 {
        vcmp_func().get_vehicle_idle_respawn_timer(self.id)
    }

    #[setter]
    fn set_idle_respawn_timer(&mut self, idle_respawn_timer: u32) {
        let _ = vcmp_func().set_vehicle_idle_respawn_timer(self.id, idle_respawn_timer);
    }

    #[getter]
    fn get_immunity(&self) -> u32 {
        vcmp_func().get_vehicle_immunity(self.id)
    }

    #[setter]
    fn set_immunity(&mut self, immunity: u32) {
        let _ = vcmp_func().set_vehicle_immunity(self.id, immunity);
    }

    #[getter]
    fn is_alive(&self) -> bool {
        vcmp_func().is_vehicle_alive(self.id)
    }

    fn is_streamed_for_player(&self, player: &PlayerPy) -> bool {
        vcmp_func().is_vehicle_streamed_for_player(self.id, player.get_id())
    }

    fn kill(&self) {
        self.explode();
    }

    #[getter]
    fn get_lights(&self) -> bool {
        vcmp_func().get_vehicle_option_lights(self.id)
    }

    #[setter]
    fn set_lights(&mut self, lights: bool) {
        let _ = vcmp_func().set_vehicle_option_lights(self.id, lights);
    }

    #[getter]
    fn get_lights_data(&self) -> u32 {
        vcmp_func().get_vehicle_lights_data(self.id)
    }

    #[setter]
    fn set_lights_data(&mut self, lights_data: u32) {
        let _ = vcmp_func().set_vehicle_lights_data(self.id, lights_data);
    }

    #[getter]
    fn get_model(&self) -> i32 {
        vcmp_func().get_vehicle_model(self.id)
    }

    #[getter]
    fn get_position(&self) -> VectorPy {
        self._position()
    }

    #[setter]
    #[pyo3(name = "position")]
    fn setter_position(&mut self, position: VectorPy) {
        let _ = vcmp_func().set_vehicle_position(self.id, position.get_entity_pos(), Some(false));
    }

    fn set_position(&self, position: VectorPy, remove_occupants: bool) {
        let _ = vcmp_func().set_vehicle_position(
            self.id,
            position.get_entity_pos(),
            Some(remove_occupants),
        );
    }

    #[getter]
    fn get_radio(&self) -> i32 {
        vcmp_func().get_vehicle_radio(self.id)
    }

    #[setter]
    fn set_radio(&mut self, radio: i32) {
        let _ = vcmp_func().set_vehicle_radio(self.id, radio);
    }

    #[getter]
    fn get_radio_locked(&self) -> bool {
        vcmp_func().get_vehicle_option_radio_locked(self.id)
    }
    #[setter]
    fn set_radio_locked(&mut self, radio_locked: bool) {
        let _ = vcmp_func().set_vehicle_option_radio_locked(self.id, radio_locked);
    }

    #[getter]
    fn get_relative_speed(&self) -> VectorPy {
        self._rel_speed()
    }

    #[setter]
    fn set_relative_speed(&mut self, relative_speed: VectorPy) {
        let _ = vcmp_func().set_vehicle_rel_speed(self.id, relative_speed.get_entity_pos());
    }

    #[getter]
    fn get_relative_turn_speed(&self) -> VectorPy {
        self._rel_turn_speed()
    }

    #[setter]
    fn set_relative_turn_speed(&mut self, relative_turn_speed: VectorPy) {
        let _ =
            vcmp_func().set_vehicle_rel_turn_speed(self.id, relative_turn_speed.get_entity_pos());
    }

    fn reset_handling(&self) {
        let _ = vcmp_func().reset_inst_handling(self.id);
    }

    fn reset_handling_rule(&self, rule_index: i32) {
        let _ = vcmp_func().reset_inst_handling_rule(self.id, rule_index);
    }

    fn respawn(&self) {
        let _ = vcmp_func().respawn_vehicle(self.id);
    }

    #[getter]
    fn get_rotation(&self) -> QuaternionPy {
        self._rotation()
    }

    #[setter]
    fn set_rotation(&mut self, rotation: QuaternionPy) {
        let _ = vcmp_func().set_vehicle_rotation(self.id, rotation.get_entity_quaternion());
    }

    #[getter]
    fn get_rotation_euler(&self) -> VectorPy {
        self._rotation_euler()
    }

    #[setter]
    fn set_rotation_euler(&mut self, rotation_euler: VectorPy) {
        let _ = vcmp_func().set_vehicle_rotation_euler(self.id, rotation_euler.get_entity_pos());
    }

    fn set_handling_rule(&self, rule_index: i32, rule_value: f64) {
        let _ = vcmp_func().set_inst_handling_rule(self.id, rule_index, rule_value);
    }

    fn set_part_status(&self, part_index: i32, status: i32) {
        let _ = vcmp_func().set_vehicle_part_status(self.id, part_index, status);
    }

    fn set_tyre_status(&self, tyre_index: i32, status: i32) {
        let _ = vcmp_func().set_vehicle_tyre_status(self.id, tyre_index, status);
    }

    #[getter]
    fn get_single_use(&self) -> bool {
        vcmp_func().get_vehicle_option_single_use(self.id)
    }

    #[setter]
    fn set_single_use(&mut self, single_use: bool) {
        let _ = vcmp_func().set_vehicle_option_single_use(self.id, single_use);
    }

    #[getter]
    fn get_siren(&self) -> bool {
        vcmp_func().get_vehicle_option_siren(self.id)
    }

    #[setter]
    fn set_siren(&mut self, siren: bool) {
        let _ = vcmp_func().set_vehicle_option_siren(self.id, siren);
    }

    #[getter]
    fn get_spawn_position(&self) -> VectorPy {
        self._spawn_position()
    }

    #[getter]
    fn get_spawn_rotation(&self) -> QuaternionPy {
        self._spawn_rotation()
    }

    #[getter]
    fn get_spawn_rotation_euler(&self) -> VectorPy {
        self._spawn_rotation_euler()
    }

    #[getter]
    fn get_speed(&self) -> VectorPy {
        self._speed()
    }

    #[setter]
    fn set_spawn_position(&self, pos: VectorPy) {
        let _ = vcmp_func().set_vehicle_spawn_position(self.id, pos.get_entity_pos());
    }

    #[setter]
    fn set_spawn_rotation(&self, rot: QuaternionPy) {
        let _ = vcmp_func().set_vehicle_spawn_rotation(self.id, rot.get_entity_quaternion());
    }

    #[setter]
    fn set_spawn_rotation_euler(&self, rot: VectorPy) {
        let _ = vcmp_func().set_vehicle_spawn_rotation_euler(self.id, rot.get_entity_pos());
    }

    #[setter]
    fn set_speed(&self, speed: VectorPy) {
        let _ = vcmp_func().set_vehicle_speed(self.id, speed.get_entity_pos());
    }

    #[getter]
    fn get_sync_source(&self) -> i32 {
        vcmp_func().get_vehicle_sync_source(self.id)
    }

    #[getter]
    fn get_sync_type(&self) -> i32 {
        vcmp_func().get_vehicle_sync_type(self.id)
    }

    #[getter]
    fn get_turn_speed(&self) -> VectorPy {
        self._turn_speed()
    }

    #[setter]
    fn set_turn_speed(&mut self, turn_speed: VectorPy) {
        let _ = vcmp_func().set_vehicle_turn_speed(self.id, turn_speed.get_entity_pos());
    }

    #[getter]
    fn get_turret_rotation(&self) -> (f32, f32) {
        vcmp_func().get_vehicle_turret_rotation(self.id)
    }

    #[getter]
    fn get_world(&self) -> i32 {
        vcmp_func().get_vehicle_world(self.id)
    }

    #[getter]
    fn get_wrecked(&self) -> bool {
        vcmp_func().is_vehicle_wrecked(self.id)
    }
}

#[pyfunction]
#[pyo3(signature = (model, world, pos, angle = 0.0, primary_color = -1, secondary_color = -1))]
pub fn create_vehicle(
    model: i32,
    world: i32,
    pos: VectorPy,
    angle: f32,
    primary_color: i32,
    secondary_color: i32,
) -> VehiclePy {
    let primary_colour = if primary_color < 0 {
        rand::random_range(0..95)
    } else {
        primary_color
    };
    let secondary_colour = if secondary_color < 0 {
        rand::random_range(0..95)
    } else {
        secondary_color
    };
    let id = vcmp_func().create_vehicle(
        model,
        world,
        pos.into(),
        angle,
        primary_colour,
        secondary_colour,
    );

    let pool = ENTITY_POOL.lock().unwrap();
    pool.get_vehicle(id)
        .map(|v| *v)
        .unwrap_or(VehiclePy::new(id))
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<VehiclePy>()?;
    m.add_function(wrap_pyfunction!(create_vehicle, m)?)?;
    Ok(())
}
