// https://bitbucket.org/stormeus/0.4-squirrel/src/master/CallbackHandler.cpp

use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{Bound, PyResult, Python};

pub mod server;

#[pyclass(subclass)]
#[pyo3(name = "Event")]
#[derive(Debug, Clone)]
pub struct BaseEvent {
    pub name: String,
}

pub trait PyBaseEvent: std::fmt::Debug + Clone {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>>;
    //pub fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
    //    let typeobj = py.get_type::<Self>();
    //    let sub_sub_class = typeobj.call((), None).unwrap();
    //}
}
impl PyBaseEvent for BaseEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let _ = py;
        todo!()
    }
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

    // abc
    let abc_module = PyModule::new(py, "abc")?;
    {
        let abc = &abc_module;
        abc.add_class::<BaseEvent>()?;
    }
    m.add_submodule(&abc_module)?;
    Ok(())
}
