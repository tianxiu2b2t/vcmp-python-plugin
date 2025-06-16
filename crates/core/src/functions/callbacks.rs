/*use pyo3::types::PyCFunction;
use pyo3::{pymethods, pyclass, types::PyFunction, Py, PyAny};
use pyo3::prelude::*;


pub struct CallbackParameter {
    pub name: String,
    pub annontations: Vec<Py<PyAny>>,
    pub default: Option<PyAny>,
    pub required: bool,
}

pub struct CallbackFunction {
    pub func: Py<PyFunction>,
    pub args: Vec<CallbackParameter>,
    pub poriority: i32,
}

#[pyclass]
#[derive(Debug, Clone, Copy)]
pub struct CallbackManager {
    callbacks: Vec<CallbackFunction>
}

impl CallbackManager {
    pub fn new() -> Self {
        Self {
            callbacks: Vec::new()
        }
    }
}

#[pymethods]
impl CallbackManager {

}*/
