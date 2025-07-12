use pyo3::{
    Bound, Py, PyAny, PyClassInitializer, PyResult, Python, pyclass, pymethods,
    types::{PyModule, PyModuleMethods},
};

use crate::py::events::abc::{BaseEvent, PyEvent};

use vcmp_bindings::events::vehicle;

#[derive(Debug, Clone)]
#[pyclass(extends=BaseEvent, subclass)]
pub struct VehicleEvent {}

impl VehicleEvent {
    pub fn new() -> (Self, BaseEvent) {
        (Self {}, BaseEvent::default())
    }
}

impl PyEvent for VehicleEvent {
    fn event_name(&self) -> String {
        "VehicleEvent".to_string()
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
#[pyclass(extends=VehicleEvent, subclass)]
pub struct VehicleUpdateEvent {
    pub inner: vehicle::VehicleUpdateEvent,
}

#[pymethods]
impl VehicleUpdateEvent {
    #[getter]
    fn vehicle_id(&self) -> i32 {
        self.inner.vehicle_id.into()
    }

    #[getter]
    fn update_type(&self) -> i32 {
        self.inner.update_type
    }

    fn __repr__(&self) -> String {
        format!(
            "VehicleUpdateEvent(vehicle_id={}, update_type={})",
            self.vehicle_id(),
            self.update_type()
        )
    }
}

impl From<vehicle::VehicleUpdateEvent> for VehicleUpdateEvent {
    fn from(event: vehicle::VehicleUpdateEvent) -> Self {
        Self { inner: event }
    }
}

impl PyEvent for VehicleUpdateEvent {
    fn event_name(&self) -> String {
        "VehicleUpdateEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(VehicleEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

#[derive(Debug, Clone)]
#[pyclass(extends=VehicleEvent, subclass)]
pub struct VehicleExplodeEvent {
    pub inner: vehicle::VehicleExplodeEvent,
}

#[pymethods]
impl VehicleExplodeEvent {
    #[getter]
    fn vehicle_id(&self) -> i32 {
        self.inner.vehicle_id.into()
    }

    fn __repr__(&self) -> String {
        format!(
            "VehicleExplodeEvent(vehicle_id={})",
            self.vehicle_id()
        )
    }
}

impl From<vehicle::VehicleExplodeEvent> for VehicleExplodeEvent {
    fn from(event: vehicle::VehicleExplodeEvent) -> Self {
        Self { inner: event }
    }
}

impl PyEvent for VehicleExplodeEvent {
    fn event_name(&self) -> String {
        "VehicleExplodeEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(VehicleEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

#[derive(Debug, Clone)]
#[pyclass(extends=VehicleEvent, subclass)]
pub struct VehicleRespawnEvent {
    pub inner: vehicle::VehicleRespawnEvent,
}

#[pymethods]
impl VehicleRespawnEvent {
    #[getter]
    fn vehicle_id(&self) -> i32 {
        self.inner.vehicle_id.into()
    }

    fn __repr__(&self) -> String {
        format!(
            "VehicleRespawnEvent(vehicle_id={})",
            self.vehicle_id()
        )
    }
}

impl From<vehicle::VehicleRespawnEvent> for VehicleRespawnEvent {
    fn from(event: vehicle::VehicleRespawnEvent) -> Self {
        Self { inner: event }
    }
}

impl PyEvent for VehicleRespawnEvent {
    fn event_name(&self) -> String {
        "VehicleRespawnEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(VehicleEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<VehicleEvent>()?;
    m.add_class::<VehicleUpdateEvent>()?;
    m.add_class::<VehicleExplodeEvent>()?;
    m.add_class::<VehicleRespawnEvent>()?;
    Ok(())
}