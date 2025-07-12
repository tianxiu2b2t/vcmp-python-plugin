use pyo3::{
    Bound, Py, PyAny, PyClassInitializer, PyResult, Python, pyclass, pymethods,
    types::{PyModule, PyModuleMethods},
};

use crate::{
    functions::{checkpoint::CheckPointPy, player::PlayerPy},
    pool::ENTITY_POOL,
    py::events::abc::{BaseEvent, PyEvent},
};

use vcmp_bindings::events::checkpoint;

#[derive(Debug, Clone)]
#[pyclass(extends=BaseEvent, subclass)]
pub struct CheckpointEvent {}

impl CheckpointEvent {
    pub fn new() -> (Self, BaseEvent) {
        (Self {}, BaseEvent::default())
    }
}

impl PyEvent for CheckpointEvent {
    fn event_name(&self) -> String {
        "CheckpointEvent".to_string()
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
#[pyclass(extends=CheckpointEvent, subclass)]
pub struct CheckpointEnteredEvent {
    pub inner: checkpoint::CheckpointEnteredEvent,
}

#[pymethods]
impl CheckpointEnteredEvent {
    #[getter]
    fn checkpoint(&self) -> CheckPointPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_checkpoint(self.inner.checkpoint_id).unwrap()
    }

    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }

    fn __repr__(&self) -> String {
        format!(
            "CheckpointEnteredEvent(checkpoint={:?}, player={:?})",
            self.checkpoint(),
            self.player()
        )
    }
}

impl From<checkpoint::CheckpointEnteredEvent> for CheckpointEnteredEvent {
    fn from(event: checkpoint::CheckpointEnteredEvent) -> Self {
        Self { inner: event }
    }
}

impl PyEvent for CheckpointEnteredEvent {
    fn event_name(&self) -> String {
        "CheckpointEnteredEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(CheckpointEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

#[derive(Debug, Clone)]
#[pyclass(extends=CheckpointEvent, subclass)]
pub struct CheckpointExitedEvent {
    pub inner: checkpoint::CheckpointExitedEvent,
}

#[pymethods]
impl CheckpointExitedEvent {
    #[getter]
    fn checkpoint(&self) -> CheckPointPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_checkpoint(self.inner.checkpoint_id).unwrap()
    }

    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }

    fn __repr__(&self) -> String {
        format!(
            "CheckpointExitedEvent(checkpoint={:?}, player={:?})",
            self.checkpoint(),
            self.player()
        )
    }
}

impl From<checkpoint::CheckpointExitedEvent> for CheckpointExitedEvent {
    fn from(event: checkpoint::CheckpointExitedEvent) -> Self {
        Self { inner: event }
    }
}

impl PyEvent for CheckpointExitedEvent {
    fn event_name(&self) -> String {
        "CheckpointExitedEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(CheckpointEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<CheckpointEvent>()?;
    m.add_class::<CheckpointEnteredEvent>()?;
    m.add_class::<CheckpointExitedEvent>()?;
    Ok(())
}
