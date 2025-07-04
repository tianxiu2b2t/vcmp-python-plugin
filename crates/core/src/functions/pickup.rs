use std::ops::Add;

use pyo3::{
    Bound, PyResult, Python, pyclass, pymethods,
    types::{PyModule, PyModuleMethods},
};
use vcmp_bindings::{func::PickupMethods, vcmp_func};

use crate::{
    functions::player::PlayerPy,
    pool::EntityPoolTrait,
    py::types::{EntityVectorType, VectorPy},
};

#[pyclass]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "Pickup")]
pub struct PickupPy {
    id: i32,
}

impl PickupPy {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

impl EntityPoolTrait for PickupPy {
    fn entity_id(&self) -> crate::consts::EntityId {
        self.id
    }
    fn entity_pool_type() -> vcmp_bindings::options::VcmpEntityPool {
        vcmp_bindings::options::VcmpEntityPool::Pickup
    }
}

impl From<i32> for PickupPy {
    fn from(val: i32) -> Self {
        PickupPy::new(val)
    }
}

impl PickupPy {
    pub fn _position(&self) -> VectorPy {
        VectorPy::from((EntityVectorType::PickupPosition, self.id))
    }
}

#[pymethods]
impl PickupPy {
    #[getter]
    pub fn get_id(&self) -> i32 {
        self.id
    }

    fn add_position(&self, position: VectorPy) {
        let origin = self._position();
        let _ = origin.add(position);
    }

    #[getter]
    fn get_alpha(&self) -> i32 {
        vcmp_func().get_pickup_alpha(self.id)
    }

    #[setter]
    fn set_alpha(&self, alpha: i32) {
        let _ = vcmp_func().set_pickup_alpha(self.id, alpha);
    }

    #[getter]
    fn get_automatic(&self) -> bool {
        vcmp_func().is_pickup_automatic(self.id)
    }

    #[setter]
    fn set_automatic(&self, automatic: bool) {
        let _ = vcmp_func().set_pickup_automatic(self.id, automatic);
    }

    fn delete(&self) {
        let _ = vcmp_func().delete_pickup(self.id);
    }

    #[getter]
    fn get_is_alive(&self) -> bool {
        vcmp_func().is_pickup_alive(self.id)
    }

    fn is_streamed_for_player(&self, player: PlayerPy) -> bool {
        vcmp_func().is_pickup_streamed_for_player(self.id, player.get_id())
    }

    #[getter]
    fn model(&self) -> i32 {
        vcmp_func().get_pickup_model(self.id)
    }

    #[getter]
    fn get_position(&self) -> VectorPy {
        self._position()
    }

    #[setter]
    fn set_position(&self, position: VectorPy) {
        let _ = vcmp_func().set_pickup_position(self.id, position.get_entity_pos());
    }

    #[getter]
    fn quantity(&self) -> i32 {
        vcmp_func().get_pickup_quantity(self.id)
    }

    fn refresh(&self) {
        let _ = vcmp_func().refresh_pickup(self.id);
    }

    #[getter]
    fn get_single_use(&self) -> bool {
        vcmp_func().is_pickup_single_use(self.id)
    }

    #[setter]
    fn set_single_use(&self, single_use: bool) {
        let _ = vcmp_func().set_pickup_single_use(self.id, single_use);
    }

    #[getter]
    fn get_timer(&self) -> u32 {
        vcmp_func().get_pickup_auto_timer(self.id)
    }

    #[setter]
    fn set_timer(&self, timer: u32) {
        let _ = vcmp_func().set_pickup_auto_timer(self.id, timer);
    }

    #[getter]
    fn get_world(&self) -> i32 {
        vcmp_func().get_pickup_world(self.id)
    }

    #[setter]
    fn set_world(&self, world: i32) {
        let _ = vcmp_func().set_pickup_world(self.id, world);
    }
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PickupPy>()?;
    Ok(())
}
