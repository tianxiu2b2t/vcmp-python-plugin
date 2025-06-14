use pyo3::{pyclass, pymethods};

use crate::pool::EntityPoolTrait;

#[pyclass]
#[derive(Debug, Clone, Copy)]
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

impl Into<VehiclePy> for i32 {
    fn into(self) -> VehiclePy {
        VehiclePy::new(self)
    }
}

#[pymethods]
impl VehiclePy {
    #[getter]
    pub fn get_id(&self) -> i32 {
        self.id
    }
}
