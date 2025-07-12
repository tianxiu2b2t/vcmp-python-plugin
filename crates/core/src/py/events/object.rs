use pyo3::{
    Bound, Py, PyAny, PyClassInitializer, PyResult, Python, pyclass, pymethods,
    types::{PyModule, PyModuleMethods},
};

use crate::py::events::abc::{BaseEvent, PyEvent};

use vcmp_bindings::events::object;

#[derive(Debug, Clone)]
#[pyclass(extends=BaseEvent, subclass)]
pub struct ObjectEvent {}

impl ObjectEvent {
    pub fn new() -> (Self, BaseEvent) {
        (Self {}, BaseEvent::default())
    }
}

impl PyEvent for ObjectEvent {
    fn event_name(&self) -> String {
        "ObjectEvent".to_string()
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
#[pyclass(extends=ObjectEvent, subclass)]
pub struct ObjectShotEvent {
    pub inner: object::ObjectShotEvent,
}

#[pymethods]
impl ObjectShotEvent {
    #[getter]
    fn object_id(&self) -> i32 {
        self.inner.object_id
    }

    #[getter]
    fn player_id(&self) -> i32 {
        self.inner.player_id.into()
    }

    #[getter]
    fn weapon_id(&self) -> i32 {
        self.inner.weapon_id
    }

    fn __repr__(&self) -> String {
        format!(
            "ObjectShotEvent(object_id={}, player_id={}, weapon_id={})",
            self.object_id(),
            self.player_id(),
            self.weapon_id()
        )
    }
}

impl From<object::ObjectShotEvent> for ObjectShotEvent {
    fn from(event: object::ObjectShotEvent) -> Self {
        Self { inner: event }
    }
}

impl PyEvent for ObjectShotEvent {
    fn event_name(&self) -> String {
        "ObjectShotEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(ObjectEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

#[derive(Debug, Clone)]
#[pyclass(extends=ObjectEvent, subclass)]
pub struct ObjectTouchedEvent {
    pub inner: object::ObjectTouchedEvent,
}

#[pymethods]
impl ObjectTouchedEvent {
    #[getter]
    fn object_id(&self) -> i32 {
        self.inner.object_id
    }

    #[getter]
    fn player_id(&self) -> i32 {
        self.inner.player_id.into()
    }

    fn __repr__(&self) -> String {
        format!(
            "ObjectTouchedEvent(object_id={}, player_id={})",
            self.object_id(),
            self.player_id()
        )
    }
}

impl From<object::ObjectTouchedEvent> for ObjectTouchedEvent {
    fn from(event: object::ObjectTouchedEvent) -> Self {
        Self { inner: event }
    }
}

impl PyEvent for ObjectTouchedEvent {
    fn event_name(&self) -> String {
        "ObjectTouchedEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(ObjectEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ObjectEvent>()?;
    m.add_class::<ObjectShotEvent>()?;
    m.add_class::<ObjectTouchedEvent>()?;
    Ok(())
}