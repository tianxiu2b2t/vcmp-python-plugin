use pyo3::{pyclass, pymethods};

use crate::pool::EntityPoolTrait;

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

#[pymethods]
impl PickupPy {
    #[getter]
    pub fn get_id(&self) -> i32 {
        self.id
    }
}
