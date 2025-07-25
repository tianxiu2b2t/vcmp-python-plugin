use std::{fmt::Display, ops::Add};

use pyo3::{
    Bound, PyResult, Python, pyclass, pyfunction, pymethods,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction,
};
use vcmp_bindings::{func::ObjectMethods, vcmp_func};

use crate::{
    functions::player::PlayerPy,
    pool::{ENTITY_POOL, EntityPoolTrait},
    py::types::{EntityQuaternionType, EntityVectorType, QuaternionPy, VectorPy},
};

#[pyclass]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "Object")]
pub struct ObjectPy {
    id: i32,
}

impl Display for ObjectPy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object({})", self.id)
    }
}

impl ObjectPy {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

impl EntityPoolTrait for ObjectPy {
    fn entity_id(&self) -> crate::consts::EntityId {
        self.id
    }
    fn entity_pool_type() -> vcmp_bindings::options::VcmpEntityPool {
        vcmp_bindings::options::VcmpEntityPool::Object
    }
}

impl From<i32> for ObjectPy {
    fn from(val: i32) -> Self {
        ObjectPy::new(val)
    }
}

impl ObjectPy {
    pub fn _position(&self) -> VectorPy {
        VectorPy::from((EntityVectorType::ObjectPosition, self.id))
    }

    pub fn _rotation(&self) -> QuaternionPy {
        QuaternionPy::from((EntityQuaternionType::ObjectRotation, self.id))
    }

    pub fn _rotation_euler(&self) -> VectorPy {
        VectorPy::from((EntityVectorType::ObjectRotationEuler, self.id))
    }
}

#[pymethods]
impl ObjectPy {
    #[getter]
    pub fn get_id(&self) -> i32 {
        self.id
    }

    fn __hash__(&self) -> i32 {
        self.id
    }

    fn __eq__(&self, other: &ObjectPy) -> bool {
        self.id == other.id
    }

    fn add_position(&self, vector: VectorPy) {
        let origin = self._position();
        let _ = origin.add(vector);
    }

    #[getter]
    fn alpha(&self) -> i32 {
        vcmp_func().get_object_alpha(self.id)
    }

    fn set_alpha(&self, alpha: i32, duration: Option<u32>) {
        let _ = vcmp_func().set_object_alpha(self.id, alpha, duration.unwrap_or_default());
    }

    pub fn delete(&self) {
        let _ = vcmp_func().delete_object(self.id);
    }

    #[getter]
    fn is_alive(&self) -> bool {
        vcmp_func().is_object_alive(self.id)
    }

    fn is_streamed_for_player(&self, player: PlayerPy) -> bool {
        vcmp_func().is_object_streamed_for_player(self.id, player.get_id())
    }

    #[getter]
    fn model(&self) -> i32 {
        vcmp_func().get_object_model(self.id)
    }

    fn move_by(&self, vector: VectorPy, duration: Option<u32>) {
        let _ = vcmp_func().move_object_by(
            self.id,
            vector.get_entity_pos(),
            duration.unwrap_or_default(),
        );
    }

    fn move_to(&self, vector: VectorPy, duration: Option<u32>) {
        let _ = vcmp_func().move_object_to(
            self.id,
            vector.get_entity_pos(),
            duration.unwrap_or_default(),
        );
    }

    #[getter]
    fn get_position(&self) -> VectorPy {
        self._position()
    }

    #[setter]
    fn set_position(&self, vector: VectorPy) {
        let _ = vcmp_func().set_object_position(self.id, vector.get_entity_pos());
    }

    fn rotate_by(&self, quaternion: QuaternionPy, duration: Option<u32>) {
        let _ = vcmp_func().rotate_object_by(
            self.id,
            quaternion.get_entity_quaternion(),
            duration.unwrap_or_default(),
        );
    }

    fn rotate_to(&self, quaternion: QuaternionPy, duration: Option<u32>) {
        let _ = vcmp_func().rotate_object_to(
            self.id,
            quaternion.get_entity_quaternion(),
            duration.unwrap_or_default(),
        );
    }

    fn rotate_by_euler(&self, vector: VectorPy, duration: Option<u32>) {
        let _ = vcmp_func().rotate_object_by_euler(
            self.id,
            vector.get_entity_pos(),
            duration.unwrap_or_default(),
        );
    }

    fn rotate_to_euler(&self, vector: VectorPy, duration: Option<u32>) {
        let _ = vcmp_func().rotate_object_to_euler(
            self.id,
            vector.get_entity_pos(),
            duration.unwrap_or_default(),
        );
    }

    #[getter]
    fn get_shot_report(&self) -> bool {
        vcmp_func().is_object_shot_report_enabled(self.id)
    }

    #[setter]
    fn set_shot_report(&self, enabled: bool) {
        let _ = vcmp_func().set_object_shot_report_enabled(self.id, enabled);
    }

    #[getter]
    fn get_touched_report(&self) -> bool {
        vcmp_func().is_object_touched_report_enabled(self.id)
    }

    #[setter]
    fn set_touched_report(&self, enabled: bool) {
        let _ = vcmp_func().set_object_touched_report_enabled(self.id, enabled);
    }

    #[getter]
    fn get_world(&self) -> i32 {
        vcmp_func().get_object_world(self.id)
    }

    #[setter]
    fn set_world(&self, world: i32) {
        let _ = vcmp_func().set_object_world(self.id, world);
    }

    #[getter]
    fn get_rotation(&self) -> QuaternionPy {
        self._rotation()
    }

    #[setter]
    fn set_rotation(&self, quaternion: QuaternionPy) {
        self.rotate_to(quaternion, Some(0));
    }

    #[getter]
    fn get_rotation_euler(&self) -> VectorPy {
        self._rotation_euler()
    }

    #[setter]
    fn set_rotation_euler(&self, vector: VectorPy) {
        self.rotate_to_euler(vector, Some(0))
    }

    fn __repr__(&self) -> String {
        format!("Object(id={})", self.id)
    }
}

#[pyfunction]
pub fn create_object(model: i32, world: i32, pos: VectorPy, alpha: i32) -> ObjectPy {
    let id = vcmp_func().create_object(model, world, pos.into(), alpha);

    let pool = ENTITY_POOL.lock().unwrap();

    pool.get_object(id).copied().unwrap_or(ObjectPy::new(id))
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ObjectPy>()?;
    m.add_function(wrap_pyfunction!(create_object, m)?)?;
    Ok(())
}
