use pyo3::{
    Bound, Py, PyAny, PyClassInitializer, PyResult, Python, pyclass, pymethods,
    types::{PyModule, PyModuleMethods},
};

use crate::py::events::abc::{BaseEvent, PyEvent};

use vcmp_bindings::events::pickup;

#[derive(Debug, Clone)]
#[pyclass(extends=BaseEvent, subclass)]
pub struct PickupEvent {}

impl PickupEvent {
    pub fn new() -> (Self, BaseEvent) {
        (Self {}, BaseEvent::default())
    }
}

impl PyEvent for PickupEvent {
    fn event_name(&self) -> String {
        "PickupEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(BaseEvent::default()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

#[derive(Debug, Clone)]
#[pyclass(extends=PickupEvent, subclass)]
pub struct PickupPickAttemptEvent {
    pub inner: pickup::PickupPickAttemptEvent,
}

#[pymethods]
impl PickupPickAttemptEvent {
    #[getter]
    fn pickup_id(&self) -> i32 {
        self.inner.pickup_id
    }

    #[getter]
    fn player_id(&self) -> i32 {
        self.inner.player_id.into()
    }

    #[getter]
    fn is_allowed(&self) -> bool {
        self.inner.is_allowed
    }

    #[setter]
    fn set_is_allowed(&mut self, value: bool) {
        self.inner.is_allowed = value;
    }

    fn __repr__(&self) -> String {
        format!(
            "PickupPickAttemptEvent(pickup_id={}, player_id={}, is_allowed={})",
            self.pickup_id(),
            self.player_id(),
            self.is_allowed()
        )
    }
}

impl From<pickup::PickupPickAttemptEvent> for PickupPickAttemptEvent {
    fn from(event: pickup::PickupPickAttemptEvent) -> Self {
        Self { inner: event }
    }
}

impl PyEvent for PickupPickAttemptEvent {
    fn event_name(&self) -> String {
        "PickupPickAttemptEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PickupEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

#[derive(Debug, Clone)]
#[pyclass(extends=PickupEvent, subclass)]
pub struct PickupPickedEvent {
    pub inner: pickup::PickupPickedEvent,
}

#[pymethods]
impl PickupPickedEvent {
    #[getter]
    fn pickup_id(&self) -> i32 {
        self.inner.pickup_id
    }

    #[getter]
    fn player_id(&self) -> i32 {
        self.inner.player_id.into()
    }

    fn __repr__(&self) -> String {
        format!(
            "PickupPickedEvent(pickup_id={}, player_id={})",
            self.pickup_id(),
            self.player_id()
        )
    }
}

impl From<pickup::PickupPickedEvent> for PickupPickedEvent {
    fn from(event: pickup::PickupPickedEvent) -> Self {
        Self { inner: event }
    }
}

impl PyEvent for PickupPickedEvent {
    fn event_name(&self) -> String {
        "PickupPickedEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PickupEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

#[derive(Debug, Clone)]
#[pyclass(extends=PickupEvent, subclass)]
pub struct PickupRespawnEvent {
    pub inner: pickup::PickupRespawnEvent,
}

#[pymethods]
impl PickupRespawnEvent {
    #[getter]
    fn pickup_id(&self) -> i32 {
        self.inner.pickup_id
    }

    fn __repr__(&self) -> String {
        format!("PickupRespawnEvent(pickup_id={})", self.pickup_id())
    }
}

impl From<pickup::PickupRespawnEvent> for PickupRespawnEvent {
    fn from(event: pickup::PickupRespawnEvent) -> Self {
        Self { inner: event }
    }
}

impl PyEvent for PickupRespawnEvent {
    fn event_name(&self) -> String {
        "PickupRespawnEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PickupEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PickupEvent>()?;
    m.add_class::<PickupPickAttemptEvent>()?;
    m.add_class::<PickupPickedEvent>()?;
    m.add_class::<PickupRespawnEvent>()?;
    Ok(())
}