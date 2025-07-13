use pyo3::{
    Bound, Py, PyAny, PyClassInitializer, PyResult, Python, pyclass, pymethods,
    types::{PyModule, PyModuleMethods},
};

use crate::{
    functions::{pickup::PickupPy, player::PlayerPy},
    pool::ENTITY_POOL,
    py::events::abc::{BaseEvent, PyEvent},
};

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
    fn pickup(&self) -> PickupPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_pickup(self.inner.pickup_id).unwrap()
    }

    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }

    fn __repr__(&self) -> String {
        format!(
            "PickupPickAttemptEvent(pickup={:?}, player={:?})",
            self.pickup(),
            self.player(),
        )
    }
}
impl From<pickup::PickupPickAttemptEvent> for PickupPickAttemptEvent {
    fn from(event: pickup::PickupPickAttemptEvent) -> Self {
        Self { inner: event }
    }
}
impl PickupPickAttemptEvent {
    pub fn new(pickup: PickupPy, player: PlayerPy) -> Self {
        Self {
            inner: pickup::PickupPickAttemptEvent {
                pickup_id: pickup.get_id(),
                player_id: player.get_id(),
            },
        }
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
    fn pickup(&self) -> PickupPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_pickup(self.inner.pickup_id).unwrap()
    }

    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }

    fn __repr__(&self) -> String {
        format!(
            "PickupPickedEvent(pickup={:?}, player={:?})",
            self.pickup(),
            self.player()
        )
    }
}
impl From<pickup::PickupPickedEvent> for PickupPickedEvent {
    fn from(event: pickup::PickupPickedEvent) -> Self {
        Self { inner: event }
    }
}
impl PickupPickedEvent {
    pub fn new(pickup: PickupPy, player: PlayerPy) -> Self {
        Self {
            inner: pickup::PickupPickedEvent {
                pickup_id: pickup.get_id(),
                player_id: player.get_id(),
            },
        }
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
    fn pickup(&self) -> PickupPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_pickup(self.inner.pickup_id).unwrap()
    }

    fn __repr__(&self) -> String {
        format!("PickupRespawnEvent(pickup={:?})", self.pickup())
    }
}
impl From<pickup::PickupRespawnEvent> for PickupRespawnEvent {
    fn from(event: pickup::PickupRespawnEvent) -> Self {
        Self { inner: event }
    }
}
impl PickupRespawnEvent {
    pub fn new(pickup: PickupPy) -> Self {
        Self {
            inner: pickup::PickupRespawnEvent {
                pickup_id: pickup.get_id(),
            },
        }
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
