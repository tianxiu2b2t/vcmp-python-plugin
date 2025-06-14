use pyo3::prelude::*;
use pyo3::{
    pymethods,
    pyclass,
};

use vcmp_bindings::vcmp_func;
use vcmp_bindings::func::{PlayerMethods};


#[pyclass]
#[derive(Debug, Clone, Copy)]
pub struct RustPlayer {
    id: i32
}

#[pymethods]
impl RustPlayer {
    
    // rust constructor, py can't call it
    #[new]
    fn new(id: i32) -> Self {
        Self {
            id
        }
    }

    #[getter]
    fn get_name(&self) -> String {
        vcmp_func().get_player_name(self.id)
    }

    #[setter]
    fn set_name(&self, name: String) {
        vcmp_func().set_player_name(self.id, name.as_str());
    }

    #[getter]
    pub fn get_id(&self) -> i32 {
        self.id
    }
}