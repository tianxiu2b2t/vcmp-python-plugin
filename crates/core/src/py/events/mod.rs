// https://bitbucket.org/stormeus/0.4-squirrel/src/master/CallbackHandler.cpp

use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{Bound, PyResult, Python};
use vcmp_bindings::events::{EntityStreamingChangeEvent, PluginCommandEvent, VcmpEvent};

use crate::py::fix_module_name;

pub mod player;
pub mod server;

#[pyclass(name = "VcmpEvent")]
pub struct PyVcmpEvnet {
    pub event_enum: VcmpEvent,
}

impl PyVcmpEvnet {
    pub fn new(event: VcmpEvent) -> Self {
        Self { event_enum: event }
    }
}

#[pymethods]
impl PyVcmpEvnet {
    #[staticmethod]
    pub fn plugin_command(identifer: u32, message: String) -> Self {
        Self { event_enum: VcmpEvent::PluginCommand(PluginCommandEvent::new(identifer, message)) }
    }

    #[staticmethod]
    pub fn entity_streaming(player_id: i32, entity_id: i32, entity_type: i32, deleted: bool) -> Self {
        Self { event_enum: VcmpEvent::EntityStreaming(EntityStreamingChangeEvent::new(player_id, entity_id, entity_type.into(), deleted))}
    }
    /*
pub enum VcmpEvent {
    PluginCommand(PluginCommandEvent),
    EntityStreaming(EntityStreamingChangeEvent),
    EntityPool(EntityPoolChangeEvent),


#[derive(Debug, Clone)]
pub struct EntityStreamingChangeEvent {
    pub player_id: PlayerId,
    pub entity_id: i32,
    pub entity_type: VcmpEntityPool,
    pub deleted: bool,
}

impl From<(i32, i32, i32, u8)> for EntityStreamingChangeEvent {
    fn from(value: (i32, i32, i32, u8)) -> Self {
        Self {
            player_id: value.0,
            entity_id: value.1,
            entity_type: VcmpEntityPool::from(value.2),
            deleted: value.3 != 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntityPoolChangeEvent {
    pub entity_type: VcmpEntityPool,
    pub entity_id: i32,
    pub deleted: bool,
}

impl From<(i32, i32, u8)> for EntityPoolChangeEvent {
    fn from(value: (i32, i32, u8)) -> Self {
        Self {
            entity_type: VcmpEntityPool::from(value.0),
            entity_id: value.1,
            deleted: value.2 != 0,
        }
    }
}


     */


}

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
