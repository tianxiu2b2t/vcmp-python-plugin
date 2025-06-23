use pyo3::{pyclass, pymethods};

use crate::pool::EntityPoolTrait;

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
    #[getter]
    pub fn get_id(&self) -> i32 {
        self.id
    }
}
