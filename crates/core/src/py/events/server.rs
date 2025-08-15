use pyo3::{
    Bound, Py, PyAny, PyClassInitializer, PyResult, Python, pyclass, pymethods,
    types::{PyModule, PyModuleMethods},
};
use vcmp_bindings::events::server;

use crate::py::events::abc::{BaseEvent, PyEvent};

#[derive(Debug, Clone)]
#[pyclass(extends=BaseEvent, subclass)]
pub struct ServerEvent {}
impl ServerEvent {
    pub fn new() -> (Self, BaseEvent) {
        (Self {}, BaseEvent::default())
    }
}
impl PyEvent for ServerEvent {
    fn event_name(&self) -> String {
        "ServerEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(BaseEvent::default()).add_subclass(self.clone()),
        )
        .expect("Failed to create ServerEvent")
        .into_any()
    }
}

#[derive(Debug, Clone, Default)]
#[pyclass(extends=ServerEvent, subclass)]
pub struct ServerInitialiseEvent {}
impl PyEvent for ServerInitialiseEvent {
    fn event_name(&self) -> String {
        "ServerInitialiseEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(ServerEvent::new()).add_subclass(self.clone()),
        )
        .expect("Failed to create ServerInitialiseEvent")
        .into_any()
    }
}

#[derive(Debug, Clone, Default)]
#[pyclass(extends=ServerEvent, subclass)]
pub struct ServerShutdownEvent {}
impl PyEvent for ServerShutdownEvent {
    fn event_name(&self) -> String {
        "ServerShutdownEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(ServerEvent::new()).add_subclass(self.clone()),
        )
        .expect("Failed to create ServerShutdownEvent")
        .into_any()
    }
}

#[derive(Debug, Clone)]
#[pyclass(extends=ServerEvent, subclass)]
pub struct ServerFrameEvent {
    pub inner: server::ServerFrameEvent,
}
#[pymethods]
impl ServerFrameEvent {
    #[getter]
    fn elapsed_time(&self) -> f32 {
        self.inner.elapsed_time
    }
    fn __repr__(&self) -> String {
        format!("ServerFrameEvent(elapsed_time={})", self.elapsed_time())
    }
}
impl From<server::ServerFrameEvent> for ServerFrameEvent {
    fn from(event: server::ServerFrameEvent) -> Self {
        Self { inner: event }
    }
}
impl ServerFrameEvent {
    pub fn new(elapsed_time: f32) -> Self {
        Self {
            inner: server::ServerFrameEvent { elapsed_time },
        }
    }
}
impl PyEvent for ServerFrameEvent {
    fn event_name(&self) -> String {
        "ServerFrameEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(ServerEvent::new()).add_subclass(self.clone()),
        )
        .expect("Failed to create ServerFrameEvent")
        .into_any()
    }
}

#[derive(Debug, Clone)]
#[pyclass(extends=ServerEvent, subclass)]
pub struct ServerPerformanceReportEvent {
    pub inner: server::ServerPerformanceReportEvent,
}
#[pymethods]
impl ServerPerformanceReportEvent {
    #[getter]
    fn descriptions(&self) -> Vec<String> {
        self.inner.descriptions.clone()
    }

    #[getter]
    fn times(&self) -> Vec<u64> {
        self.inner.times.clone()
    }

    #[getter]
    fn entry_count(&self) -> usize {
        self.inner.entry_count
    }

    fn __repr__(&self) -> String {
        format!(
            "ServerPerformanceReportEvent(descriptions={:?}, times={:?}, entry_count={})",
            self.descriptions(),
            self.times(),
            self.entry_count()
        )
    }
}
impl From<server::ServerPerformanceReportEvent> for ServerPerformanceReportEvent {
    fn from(event: server::ServerPerformanceReportEvent) -> Self {
        Self { inner: event }
    }
}
impl ServerPerformanceReportEvent {
    pub fn new(descriptions: Vec<String>, times: Vec<u64>, entry_count: usize) -> Self {
        Self {
            inner: server::ServerPerformanceReportEvent {
                entry_count,
                descriptions,
                times,
            },
        }
    }
}
impl PyEvent for ServerPerformanceReportEvent {
    fn event_name(&self) -> String {
        "ServerPerformanceReportEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(ServerEvent::new()).add_subclass(self.clone()),
        )
        .expect("Failed to create ServerPerformanceReportEvent")
        .into_any()
    }
}

// Extra
#[derive(Debug, Clone)]
#[pyclass(extends=ServerEvent, subclass)]
pub struct ServerReloadedEvent {
    /// 花费多长时间
    pub elapsed_time: f64,
}
#[pymethods]
impl ServerReloadedEvent {
    #[getter]
    fn elapsed_time(&self) -> f64 {
        self.elapsed_time
    }
    fn __repr__(&self) -> String {
        format!("ServerReloadedEvent(elapsed_time={})", self.elapsed_time())
    }
}
impl From<f64> for ServerReloadedEvent {
    fn from(value: f64) -> Self {
        Self {
            elapsed_time: value,
        }
    }
}
impl ServerReloadedEvent {
    pub fn new(elapsed_time: f64) -> Self {
        Self { elapsed_time }
    }
}
impl PyEvent for ServerReloadedEvent {
    fn event_name(&self) -> String {
        "ServerReloadedEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(ServerEvent::new()).add_subclass(self.clone()),
        )
        .expect("Failed to create ServerReloadedEvent")
        .into_any()
    }
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ServerEvent>()?;
    m.add_class::<ServerInitialiseEvent>()?;
    m.add_class::<ServerShutdownEvent>()?;
    m.add_class::<ServerFrameEvent>()?;
    m.add_class::<ServerPerformanceReportEvent>()?;
    m.add_class::<ServerReloadedEvent>()?;
    Ok(())
}
