use std::{
    ops::Add,
    fmt::Display
};

use pyo3::{
    Bound, PyResult, Python, pyclass, pyfunction, pymethods,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction,
};
use vcmp_bindings::{func::CheckPointMethods, vcmp_func};

use crate::{
    functions::player::PlayerPy,
    pool::{ENTITY_POOL, EntityPoolTrait},
    py::types::{EntityVectorType, RGBPy, VectorPy},
};

#[pyclass]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "CheckPoint")]
pub struct CheckPointPy {
    id: i32,
}

impl Display for CheckPointPy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CheckPoint({})", self.id)
    }
}

impl CheckPointPy {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

impl EntityPoolTrait for CheckPointPy {
    fn entity_id(&self) -> crate::consts::EntityId {
        self.id
    }
    fn entity_pool_type() -> vcmp_bindings::options::VcmpEntityPool {
        vcmp_bindings::options::VcmpEntityPool::CheckPoint
    }
}

impl From<i32> for CheckPointPy {
    fn from(val: i32) -> Self {
        CheckPointPy::new(val)
    }
}

impl CheckPointPy {
    pub fn _position(&self) -> VectorPy {
        VectorPy::from((EntityVectorType::CheckPointPosition, self.id))
    }
}

#[pymethods]
impl CheckPointPy {
    #[getter]
    pub fn get_id(&self) -> i32 {
        self.id
    }

    fn __hash__(&self) -> i32 {
        self.id
    }

    fn __eq__(&self, other: &CheckPointPy) -> bool {
        self.id == other.id
    }

    fn add_position(&self, pos: VectorPy) {
        let origin = self._position();
        let _ = origin.add(pos);
    }

    #[getter]
    fn get_color(&self) -> RGBPy {
        RGBPy::from(
            vcmp_func()
                .get_checkpoint_colour(self.id)
                .unwrap_or_default(),
        )
    }

    #[setter]
    fn set_color(&self, color: RGBPy) {
        let _ = vcmp_func().set_checkpoint_colour(self.id, color.into());
    }

    pub fn delete(&self) {
        let _ = vcmp_func().delete_checkpoint(self.id);
    }

    #[getter]
    fn is_alive(&self) -> bool {
        vcmp_func().is_checkpoint_alive(self.id)
    }

    fn is_streamed_for_player(&self, player: PlayerPy) -> bool {
        vcmp_func().is_checkpoint_streamed_for_player(self.id, player.get_id())
    }

    #[getter]
    fn get_owner(&self) -> Option<PlayerPy> {
        let id = vcmp_func().get_checkpoint_owner(self.id);
        let pool = ENTITY_POOL.lock().unwrap();
        pool.get_player(id).copied()
    }

    #[getter]
    fn get_position(&self) -> VectorPy {
        self._position()
    }

    #[setter]
    fn set_position(&self, pos: VectorPy) {
        let _ = vcmp_func().set_checkpoint_position(self.id, pos.into());
    }

    #[getter]
    fn get_radius(&self) -> f32 {
        vcmp_func().get_checkpoint_radius(self.id)
    }

    #[setter]
    fn set_radius(&self, radius: f32) {
        let _ = vcmp_func().set_checkpoint_radius(self.id, radius);
    }

    #[getter]
    fn get_sphere(&self) -> bool {
        vcmp_func().is_checkpoint_sphere(self.id)
    }

    #[getter]
    fn get_world(&self) -> i32 {
        vcmp_func().get_checkpoint_world(self.id)
    }

    #[setter]
    fn set_world(&self, world: i32) {
        let _ = vcmp_func().set_checkpoint_world(self.id, world);
    }

    fn __repr__(&self) -> String {
        format!("CheckPoint(id={})", self.id)
    }
}

#[pyfunction]
#[pyo3(signature = (world, sphere, pos, color, radius, player = None))]
pub fn create_checkpoint(
    world: i32,
    sphere: bool,
    pos: VectorPy,
    color: RGBPy,
    radius: f32,
    player: Option<PlayerPy>,
) -> CheckPointPy {
    let id = vcmp_func().create_checkpoint(
        player.map(|p| p.get_id()),
        world,
        sphere,
        pos.into(),
        color.into(),
        radius,
    );

    let pool = ENTITY_POOL.lock().unwrap();

    pool.get_checkpoint(id)
        .map(|c| *c)
        .unwrap_or(CheckPointPy::new(id))
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<CheckPointPy>()?;
    m.add_function(wrap_pyfunction!(create_checkpoint, m)?)?;
    Ok(())
}
