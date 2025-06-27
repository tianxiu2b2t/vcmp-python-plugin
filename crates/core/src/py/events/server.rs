use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{Bound, PyResult, Python, pyclass, pymethods};
use vcmp_bindings::events::server;

use crate::py::events::BaseEvent;
// rewrite this to use pyclass

#[pyclass(extends=BaseEvent, subclass)]
#[derive(Debug, Default)]
#[pyo3(name = "ServerEvent")]
pub struct ServerEvent;

#[pymethods]
impl ServerEvent {
    #[new]
    pub fn new() -> (Self, BaseEvent) {
        (Self, BaseEvent::new("ServerEvent"))
    }
}

#[pyclass(extends=ServerEvent)]
#[derive(Debug, Default)]
#[pyo3(name = "ServerInitialiseEvent")]
pub struct ServerInitialiseEvent;

#[pymethods]
impl ServerInitialiseEvent {
    #[new]
    pub fn new() -> PyClassInitializer<Self> {
        PyClassInitializer::from(ServerEvent::new()).add_subclass(Self)
    }
}

#[pyclass(extends=ServerEvent)]
#[derive(Debug, Default)]
#[pyo3(name = "ServerShutdownEvent")]
pub struct ServerShutdownEvent;

#[pymethods]
impl ServerShutdownEvent {
    #[new]
    pub fn new() -> PyClassInitializer<Self> {
        PyClassInitializer::from(ServerEvent::new()).add_subclass(Self)
    }
}

#[pyclass(extends=ServerEvent)]
#[derive(Debug)]
#[pyo3(name = "ServerFrameEvent")]
pub struct ServerFrameEvent {
    inner: server::ServerFrameEvent,
}

#[pymethods]
impl ServerFrameEvent {
    #[new]
    #[pyo3(signature = (elapsed_time))]
    pub fn new(elapsed_time: f32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(ServerEvent::new()).add_subclass(Self {
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
#[derive(Debug)]
#[pyo3(name = "ServerPerformanceReportEvent")]
pub struct ServerPerformanceReportEvent {
    inner: server::ServerPerformanceReportEvent,
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
        PyClassInitializer::from(ServerEvent::new()).add_subclass(Self {
            inner: server::ServerPerformanceReportEvent {
                entry_count,
                descriptions,
                times,
            },
        })
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

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ServerInitialiseEvent>()?;
    m.add_class::<ServerFrameEvent>()?;
    m.add_class::<ServerPerformanceReportEvent>()?;
    m.add_class::<ServerShutdownEvent>()?;
    Ok(())
}
