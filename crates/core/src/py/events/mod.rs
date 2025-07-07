// https://bitbucket.org/stormeus/0.4-squirrel/src/master/CallbackHandler.cpp

use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{Bound, PyResult, Python};

use crate::py::fix_module_name;

pub mod player;
pub mod server;

#[pyclass(subclass)]
#[pyo3(name = "Event")]
#[derive(Debug, Clone)]
pub struct BaseEvent {
    pub name: String,
}

pub trait PyBaseEvent: std::fmt::Debug + Clone {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>>;
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
    fix_module_name(py, &server_module, "events.server");
    m.add_submodule(&server_module)?;

    let player_module = PyModule::new(py, "player")?;
    player::module_define(py, &player_module)?;
    fix_module_name(py, &player_module, "events.player");
    m.add_submodule(&player_module)?;

    // abc
    let abc_module = PyModule::new(py, "abc")?;
    {
        let abc = &abc_module;
        abc.add_class::<BaseEvent>()?;
    }
    fix_module_name(py, &abc_module, "events.abc");
    m.add_submodule(&abc_module)?;
    Ok(())
}
