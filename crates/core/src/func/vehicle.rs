use pyo3::{pyclass, pymethods};

use vcmp_bindings::func::PlayerMethods;
use vcmp_bindings::vcmp_func;

#[pyclass]
#[derive(Debug, Clone, Copy)]
pub struct RustVehicle {
    id: i32,
}

impl RustVehicle {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

#[pymethods]
impl RustVehicle {
    #[getter]
    pub fn get_id(&self) -> i32 {
        self.id
    }
}
