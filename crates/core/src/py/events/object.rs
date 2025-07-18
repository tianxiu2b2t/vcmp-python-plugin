use pyo3::{
    Bound, Py, PyAny, PyClassInitializer, PyResult, Python, pyclass, pymethods,
    types::{PyModule, PyModuleMethods},
};

use crate::{
    functions::{object::ObjectPy, player::PlayerPy},
    pool::ENTITY_POOL,
    py::events::abc::{BaseEvent, PyEvent},
};

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
    fn object(&self) -> ObjectPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_object(self.inner.object_id).unwrap()
    }

    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }

    #[getter]
    fn weapon_id(&self) -> i32 {
        self.inner.weapon_id
    }

    fn __repr__(&self) -> String {
        format!(
            "ObjectShotEvent(object={}, player={}, weapon_id={})",
            self.object(),
            self.player(),
            self.weapon_id()
        )
    }
}
impl From<object::ObjectShotEvent> for ObjectShotEvent {
    fn from(event: object::ObjectShotEvent) -> Self {
        Self { inner: event }
    }
}
impl ObjectShotEvent {
    pub fn new(object: ObjectPy, player: PlayerPy, weapon_id: i32) -> Self {
        Self {
            inner: object::ObjectShotEvent {
                object_id: object.get_id(),
                player_id: player.get_id(),
                weapon_id,
            },
        }
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
    fn object(&self) -> ObjectPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_object(self.inner.object_id).unwrap()
    }

    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }

    fn __repr__(&self) -> String {
        format!(
            "ObjectTouchedEvent(object={}, player={})",
            self.object(),
            self.player()
        )
    }
}
impl From<object::ObjectTouchedEvent> for ObjectTouchedEvent {
    fn from(event: object::ObjectTouchedEvent) -> Self {
        Self { inner: event }
    }
}
impl ObjectTouchedEvent {
    pub fn new(object: ObjectPy, player: PlayerPy) -> Self {
        Self {
            inner: object::ObjectTouchedEvent {
                object_id: object.get_id(),
                player_id: player.get_id(),
            },
        }
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
