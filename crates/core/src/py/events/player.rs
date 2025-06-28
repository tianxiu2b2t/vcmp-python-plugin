use std::ffi::c_char;

use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{Bound, PyResult, Python, pyclass, pymethods};
use vcmp_bindings::events::player;

use crate::py::events::{BaseEvent, PyBaseEvent};
// rewrite this to use pyclass

#[pyclass(extends=BaseEvent, subclass)]
#[derive(Debug, Clone, Default)]
#[pyo3(name = "PlayerEvent")]
pub struct PlayerEvent;

#[pymethods]
impl PlayerEvent {
    #[new]
    pub fn new(name: &str) -> (Self, BaseEvent) {
        (Self, BaseEvent::new(name))
    }
}

impl PyBaseEvent for PlayerEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(py, PlayerEvent::new("PlayerEvent"))?;
        Ok(value.into())
    }
}

#[pyclass(extends=PlayerEvent, unsendable)]
#[derive(Debug, Clone)]
#[pyo3(name = "IncomingConnectionEvent")]
pub struct IncomingConnectionEvent {
    inner: player::IncomingConnectionEvent,
}

impl From<(*mut c_char, usize, *const c_char, *const c_char)> for IncomingConnectionEvent {
    fn from(value: (*mut c_char, usize, *const c_char, *const c_char)) -> Self {
        Self {
            inner: player::IncomingConnectionEvent::from(value),
        }
    }
}

impl IncomingConnectionEvent {
    pub fn new(inner: player::IncomingConnectionEvent) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("IncomingConnectionEvent"))
            .add_subclass(Self { inner })
    }
}

#[pymethods]
impl IncomingConnectionEvent {
    #[new]
    pub fn py_new(player_name: String, password: String, ip: String) -> PyClassInitializer<Self> {
        // (*mut c_char, usize, *const c_char, *const c_char)
        Self::new(player::IncomingConnectionEvent::from((
            player_name.as_str().as_ptr() as *mut c_char,
            player_name.len(),
            password.as_str().as_ptr() as *const c_char,
            ip.as_str().as_ptr() as *const c_char,
        )))
    }
}

impl PyBaseEvent for IncomingConnectionEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(py, IncomingConnectionEvent::new(self.inner.clone()))?;
        Ok(value.into())
    }
}

#[pyclass(extends=PlayerEvent, unsendable)]
#[derive(Debug, Clone)]
#[pyo3(name = "PlayerConnectEvent")]
pub struct PlayerConnectEvent {
    player_id: i32,
}

#[pymethods]
impl PlayerConnectEvent {
    #[new]
    pub fn new(player_id: i32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerConnectEvent"))
            .add_subclass(Self { player_id })
    }
}

impl From<i32> for PlayerConnectEvent {
    fn from(value: i32) -> Self {
        Self { player_id: value }
    }
}

impl PyBaseEvent for PlayerConnectEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(py, PlayerConnectEvent::new(self.player_id))?;
        Ok(value.into())
    }
}

#[pyclass(extends=PlayerEvent, unsendable)]
#[derive(Debug, Clone)]
#[pyo3(name = "PlayerDisconnectEvent")]
pub struct PlayerDisconnectEvent {
    player_id: i32,
    reason: i32,
}

#[pymethods]
impl PlayerDisconnectEvent {
    #[new]
    pub fn new(player_id: i32, reason: i32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerDisconnectEvent"))
            .add_subclass(Self { player_id, reason })
    }
}

impl From<(i32, i32)> for PlayerDisconnectEvent {
    fn from(value: (i32, i32)) -> Self {
        Self {
            player_id: value.0,
            reason: value.1,
        }
    }
}

impl PyBaseEvent for PlayerDisconnectEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(py, PlayerDisconnectEvent::new(self.player_id, self.reason))?;
        Ok(value.into())
    }
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PlayerEvent>()?;
    m.add_class::<IncomingConnectionEvent>()?;
    m.add_class::<PlayerConnectEvent>()?;
    m.add_class::<PlayerDisconnectEvent>()?;
    Ok(())
}
