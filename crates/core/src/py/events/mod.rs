// https://bitbucket.org/stormeus/0.4-squirrel/src/master/CallbackHandler.cpp

use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{Bound, PyResult, Python};

pub mod server;

#[pyclass(subclass)]
#[pyo3(name = "Event")]
pub struct BaseEvent {
    pub name: String,
}

impl BaseEvent {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

pub fn module_define(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let server_module = PyModule::new(py, "server")?;
    server::module_define(py, &server_module)?;
    m.add_submodule(&server_module)?;
    Ok(())
}
