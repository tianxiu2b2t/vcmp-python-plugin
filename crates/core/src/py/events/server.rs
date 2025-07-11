use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{Bound, PyResult, Python, pyclass, pymethods};
use vcmp_bindings::events::server;

use crate::py::events::{BaseEvent, PyBaseEvent};
// rewrite this to use pyclass

#[pyclass(extends=BaseEvent, subclass)]
#[derive(Debug, Clone, Default)]
#[pyo3(name = "ServerEvent")]
pub struct ServerEvent;

#[pymethods]
impl ServerEvent {
    #[new]
    #[pyo3(signature = (name = "ServerEvent"))]
    pub fn new(name: &str) -> (Self, BaseEvent) {
        (Self, BaseEvent::new(name))
    }
}

#[pyclass(extends=ServerEvent)]
#[derive(Debug, Clone, Default)]
#[pyo3(name = "ServerInitialiseEvent")]
pub struct ServerInitialiseEvent;

#[pymethods]
impl ServerInitialiseEvent {
    #[new]
    pub fn new() -> PyClassInitializer<Self> {
        PyClassInitializer::from(ServerEvent::new("ServerInitialiseEvent")).add_subclass(Self)
    }
}

#[pyclass(extends=ServerEvent)]
#[derive(Debug, Clone, Default)]
#[pyo3(name = "ServerShutdownEvent")]
pub struct ServerShutdownEvent;

#[pymethods]
impl ServerShutdownEvent {
    #[new]
    pub fn new() -> PyClassInitializer<Self> {
        PyClassInitializer::from(ServerEvent::new("ServerShutdownEvent")).add_subclass(Self)
    }
}

#[pyclass(extends=ServerEvent)]
#[derive(Debug, Clone)]
#[pyo3(name = "ServerFrameEvent")]
pub struct ServerFrameEvent {
    pub inner: server::ServerFrameEvent,
}

impl From<f32> for ServerFrameEvent {
    fn from(value: f32) -> Self {
        Self {
            inner: server::ServerFrameEvent::from(value),
        }
    }
}

#[pymethods]
impl ServerFrameEvent {
    #[new]
    #[pyo3(signature = (elapsed_time))]
    pub fn new(elapsed_time: f32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(ServerEvent::new("ServerFrameEvent")).add_subclass(Self {
            inner: server::ServerFrameEvent::from(elapsed_time),
        })
    }

    #[getter]
    pub fn frame(&self) -> f32 {
        self.inner.elapsed_time
    }

    pub fn __repr__(&self) -> String {
        format!("ServerFrameEvent({})", self.frame())
    }
}

#[pyclass(extends=ServerEvent)]
#[derive(Debug, Clone)]
#[pyo3(name = "ServerPerformanceReportEvent")]
pub struct ServerPerformanceReportEvent {
    pub inner: server::ServerPerformanceReportEvent,
}

impl From<(usize, *mut *const ::std::os::raw::c_char, *mut u64)> for ServerPerformanceReportEvent {
    fn from(value: (usize, *mut *const ::std::os::raw::c_char, *mut u64)) -> Self {
        Self {
            inner: server::ServerPerformanceReportEvent::from(value),
        }
    }
}

#[pymethods]
impl ServerPerformanceReportEvent {
    #[new]
    #[pyo3(signature = (entry_count, descriptions, times))]
    pub fn new(
        entry_count: usize,
        descriptions: Vec<String>,
        times: Vec<u64>,
    ) -> PyClassInitializer<Self> {
        PyClassInitializer::from(ServerEvent::new("ServerPerformanceReportEvent")).add_subclass(
            Self {
                inner: server::ServerPerformanceReportEvent {
                    entry_count,
                    descriptions,
                    times,
                },
            },
        )
    }

    #[getter]
    pub fn entry_count(&self) -> usize {
        self.inner.entry_count
    }

    #[getter]
    pub fn descriptions(&self) -> Vec<String> {
        self.inner.descriptions.clone()
    }

    #[getter]
    pub fn times(&self) -> Vec<u64> {
        self.inner.times.clone()
    }

    pub fn __repr__(&self) -> String {
        format!(
            "ServerPerformanceReportEvent(entry_count={}, descriptions={:?}, times={:?})",
            self.entry_count(),
            self.descriptions(),
            self.times()
        )
    }
}

/*
    for python
*/

impl PyBaseEvent for ServerInitialiseEvent {
    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(py, ServerInitialiseEvent::new()).expect("Failed to create ServerInitialiseEvent").into()
    }
}
impl PyBaseEvent for ServerShutdownEvent {
    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(py, ServerShutdownEvent::new()).expect("Failed to create ServerShutdownEvent").into()
    }
}
impl PyBaseEvent for ServerFrameEvent {
    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(py, ServerFrameEvent::new(self.frame())).expect("Failed to create ServerFrameEvent").into()
    }
}
impl PyBaseEvent for ServerPerformanceReportEvent {
    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            ServerPerformanceReportEvent::new(
                self.entry_count(),
                self.descriptions().clone(),
                self.times().clone(),
            ),
        ).expect("Failed to create ServerPerformanceReportEvent").into()
    }
}
impl PyBaseEvent for ServerEvent {
    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(py, ServerEvent::new("ServerEvent")).expect("Failed to create ServerEvent").into()
    }
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ServerEvent>()?;
    m.add_class::<ServerInitialiseEvent>()?;
    m.add_class::<ServerFrameEvent>()?;
    m.add_class::<ServerPerformanceReportEvent>()?;
    m.add_class::<ServerShutdownEvent>()?;
    Ok(())
}
