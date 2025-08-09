use pyo3::{
    Bound, Py, PyAny, PyClassInitializer, PyResult, Python, pyclass, pymethods,
    types::{PyModule, PyModuleMethods},
};

use crate::{
    functions::vehicle::VehiclePy,
    pool::ENTITY_POOL,
    py::{
        events::abc::{BaseEvent, PyEvent},
        types::VectorPy,
    },
};

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
    fn vehicle(&self) -> VehiclePy {
        let pool = ENTITY_POOL.lock().expect("Failed to lock entity pool");
        *pool.get_vehicle(self.inner.vehicle_id).unwrap()
    }

    #[getter]
    fn update_type(&self) -> i32 {
        self.inner.update_type
    }

    fn __repr__(&self) -> String {
        format!(
            "VehicleUpdateEvent(vehicle={}, update_type={})",
            self.vehicle(),
            self.update_type()
        )
    }
}
impl From<vehicle::VehicleUpdateEvent> for VehicleUpdateEvent {
    fn from(event: vehicle::VehicleUpdateEvent) -> Self {
        Self { inner: event }
    }
}
impl VehicleUpdateEvent {
    pub fn new(vehicle: VehiclePy, update_type: i32) -> Self {
        Self {
            inner: vehicle::VehicleUpdateEvent {
                vehicle_id: vehicle.get_id(),
                update_type,
            },
        }
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
    fn vehicle(&self) -> VehiclePy {
        let pool = ENTITY_POOL.lock().expect("Failed to lock entity pool");
        *pool.get_vehicle(self.inner.vehicle_id).unwrap()
    }

    fn __repr__(&self) -> String {
        format!("VehicleExplodeEvent(vehicle={})", self.vehicle())
    }
}
impl From<vehicle::VehicleExplodeEvent> for VehicleExplodeEvent {
    fn from(event: vehicle::VehicleExplodeEvent) -> Self {
        Self { inner: event }
    }
}
impl VehicleExplodeEvent {
    pub fn new(vehicle: VehiclePy) -> Self {
        Self {
            inner: vehicle::VehicleExplodeEvent {
                vehicle_id: vehicle.get_id(),
            },
        }
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
    fn vehicle(&self) -> VehiclePy {
        let pool = ENTITY_POOL.lock().expect("Failed to lock entity pool");
        *pool.get_vehicle(self.inner.vehicle_id).unwrap()
    }

    fn __repr__(&self) -> String {
        format!("VehicleRespawnEvent(vehicle={})", self.vehicle())
    }
}
impl From<vehicle::VehicleRespawnEvent> for VehicleRespawnEvent {
    fn from(event: vehicle::VehicleRespawnEvent) -> Self {
        Self { inner: event }
    }
}
impl VehicleRespawnEvent {
    pub fn new(vehicle: VehiclePy) -> Self {
        Self {
            inner: vehicle::VehicleRespawnEvent {
                vehicle_id: vehicle.get_id(),
            },
        }
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

#[derive(Debug, Clone, Copy)]
#[pyclass(extends=VehicleEvent, subclass)]
pub struct VehicleMoveEvent {
    pub vehicle_id: i32,
    pub old_position: VectorPy,
    pub new_position: VectorPy,
    pub current_position: VectorPy,
}
#[pymethods]
impl VehicleMoveEvent {
    #[getter]
    fn vehicle(&self) -> VehiclePy {
        let pool = ENTITY_POOL.lock().expect("Failed to lock entity pool");
        *pool.get_vehicle(self.vehicle_id).unwrap()
    }

    #[getter]
    fn get_old_position(&self) -> VectorPy {
        self.old_position
    }

    #[getter]
    fn get_new_position(&self) -> VectorPy {
        self.new_position
    }

    #[getter]
    fn get_current_position(&self) -> VectorPy {
        self.current_position
    }

    #[setter]
    fn set_current_position(&mut self, position: VectorPy) {
        self.current_position = position;
        {
            let mut pool = ENTITY_POOL.lock().expect("Failed to lock entity pool");
            let vehicle = pool.get_mut_vehicle(self.vehicle_id).unwrap();
            vehicle.set_var_last_position(self.current_position.get_entity_pos());
        };
    }

    fn __repr__(&self) -> String {
        format!(
            "VehicleMoveEvent(vehicle={}, old_position={:?}, new_position={:?})",
            self.vehicle(),
            self.get_old_position(),
            self.get_new_position()
        )
    }
}
impl From<(i32, VectorPy, VectorPy)> for VehicleMoveEvent {
    fn from(value: (i32, VectorPy, VectorPy)) -> Self {
        let mut this = Self {
            vehicle_id: value.0,
            old_position: value.1,
            new_position: value.2,
            current_position: VectorPy::default(),
        };
        this.set_current_position(value.2);
        this
    }
}
impl VehicleMoveEvent {
    pub fn new(vehicle: VehiclePy, old_position: VectorPy, new_position: VectorPy) -> Self {
        let mut this = Self {
            vehicle_id: vehicle.get_id(),
            old_position,
            new_position,
            current_position: VectorPy::default(),
        };
        this.set_current_position(new_position);
        this
    }
}
impl PyEvent for VehicleMoveEvent {
    fn event_name(&self) -> String {
        "VehicleMoveEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(VehicleEvent::new()).add_subclass(*self),
        )
        .unwrap()
        .into_any()
    }
}

#[derive(Debug, Clone, Copy)]
#[pyclass(extends=VehicleEvent, subclass)]
pub struct VehicleHealthChangeEvent {
    pub vehicle_id: i32,
    pub old_health: f32,
    pub new_health: f32,
    pub current_health: f32,
}
#[pymethods]
impl VehicleHealthChangeEvent {
    #[getter]
    fn vehicle(&self) -> VehiclePy {
        let pool = ENTITY_POOL.lock().expect("Failed to lock entity pool");
        *pool.get_vehicle(self.vehicle_id).unwrap()
    }

    #[getter]
    fn get_old_health(&self) -> f32 {
        self.old_health
    }

    #[getter]
    fn get_new_health(&self) -> f32 {
        self.new_health
    }

    #[getter]
    fn get_current_health(&self) -> f32 {
        self.current_health
    }

    #[setter]
    fn set_current_health(&mut self, health: f32) {
        self.current_health = health;
        {
            let mut pool = ENTITY_POOL.lock().expect("Failed to lock entity pool");
            let vehicle = pool.get_mut_vehicle(self.vehicle_id).unwrap();
            vehicle.set_var_last_health(self.current_health);
        };
    }

    fn __repr__(&self) -> String {
        format!(
            "VehicleHealthChangeEvent(vehicle={}, old_health={}, new_health={})",
            self.vehicle(),
            self.get_old_health(),
            self.get_new_health()
        )
    }
}
impl From<(i32, f32, f32)> for VehicleHealthChangeEvent {
    fn from(value: (i32, f32, f32)) -> Self {
        let mut this = Self {
            vehicle_id: value.0,
            old_health: value.1,
            new_health: value.2,
            current_health: 0.0,
        };
        this.set_current_health(value.2);
        this
    }
}
impl VehicleHealthChangeEvent {
    pub fn new(vehicle: VehiclePy, old_health: f32, new_health: f32) -> Self {
        let mut this = Self {
            vehicle_id: vehicle.get_id(),
            old_health,
            new_health,
            current_health: 0.0,
        };
        this.set_current_health(new_health);
        this
    }
}
impl PyEvent for VehicleHealthChangeEvent {
    fn event_name(&self) -> String {
        "VehicleHealthChangeEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(VehicleEvent::new()).add_subclass(*self),
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

    // Vehicle Extra
    m.add_class::<VehicleMoveEvent>()?;
    m.add_class::<VehicleHealthChangeEvent>()?;
    Ok(())
}
