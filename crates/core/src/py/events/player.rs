use pyo3::{
    Bound, Py, PyAny, PyClassInitializer, PyResult, Python, pyclass, pymethods,
    types::{PyModule, PyModuleMethods},
};
use vcmp_bindings::events::player;

use crate::{
    functions::{
        keybind::{KeyBindPy, get_bindkey},
        player::PlayerPy,
        vehicle::VehiclePy,
    },
    pool::ENTITY_POOL,
    py::{
        events::abc::{BaseEvent, PyEvent},
        streams::ReadStream,
        types::VectorPy,
    },
};

#[derive(Debug, Clone)]
#[pyclass(extends=BaseEvent, subclass)]
pub struct PlayerEvent {}
impl PlayerEvent {
    pub fn new() -> (Self, BaseEvent) {
        (Self {}, BaseEvent::default())
    }
}
impl PyEvent for PlayerEvent {
    fn event_name(&self) -> String {
        "PlayerEvent".to_string()
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
#[pyclass(extends=PlayerEvent, subclass)]
pub struct IncomingConnectionEvent {
    pub inner: player::IncomingConnectionEvent,
}
#[pymethods]
impl IncomingConnectionEvent {
    #[getter]
    fn ip(&self) -> String {
        self.inner.ip.clone()
    }
    #[getter]
    fn player_name(&self) -> String {
        self.inner.player_name.clone()
    }
    #[getter]
    fn password(&self) -> String {
        self.inner.password.clone()
    }
    fn __repr__(&self) -> String {
        format!(
            "IncomingConnectionEvent(player_name={}, ip={}, password={})",
            self.player_name(),
            self.ip(),
            self.password()
        )
    }
}
impl From<player::IncomingConnectionEvent> for IncomingConnectionEvent {
    fn from(event: player::IncomingConnectionEvent) -> Self {
        Self { inner: event }
    }
}
impl IncomingConnectionEvent {
    pub fn new(ip: String, player_name: String, password: String) -> Self {
        Self {
            inner: player::IncomingConnectionEvent {
                ip,
                player_name,
                password,
            },
        }
    }
}
impl PyEvent for IncomingConnectionEvent {
    fn event_name(&self) -> String {
        "IncomingConnectionEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct ClientScriptDataEvent {
    pub inner: player::ClientScriptDataEvent,
}
#[pymethods]
impl ClientScriptDataEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn stream(&self) -> ReadStream {
        ReadStream::from(self.inner.data.clone())
    }
    fn __repr__(&self) -> String {
        format!(
            "ClientScriptDataEvent(player={:?}, stream={})",
            self.player(),
            self.stream()
        )
    }
}
impl From<player::ClientScriptDataEvent> for ClientScriptDataEvent {
    fn from(event: player::ClientScriptDataEvent) -> Self {
        Self { inner: event }
    }
}
impl ClientScriptDataEvent {
    pub fn new(player: PlayerPy, data: Vec<u8>) -> Self {
        Self {
            inner: player::ClientScriptDataEvent {
                player_id: player.get_id(),
                data,
            },
        }
    }
}
impl PyEvent for ClientScriptDataEvent {
    fn event_name(&self) -> String {
        "ClientScriptDataEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerConnectEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerConnectEvent {
    pub inner: player::PlayerConnectEvent,
}
#[pymethods]
impl PlayerConnectEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    fn __repr__(&self) -> String {
        format!("PlayerConnectEvent(player={:?})", self.player())
    }
}
impl From<player::PlayerConnectEvent> for PlayerConnectEvent {
    fn from(event: player::PlayerConnectEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerConnectEvent {
    pub fn new(player: PlayerPy) -> Self {
        Self {
            inner: player::PlayerConnectEvent {
                player_id: player.get_id(),
            },
        }
    }
}
impl PyEvent for PlayerConnectEvent {
    fn event_name(&self) -> String {
        "PlayerConnectEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerDisconnectEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerDisconnectEvent {
    pub inner: player::PlayerDisconnectEvent,
}
#[pymethods]
impl PlayerDisconnectEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn reason(&self) -> i32 {
        self.inner.reason
    }
    fn __repr__(&self) -> String {
        format!(
            "PlayerDisconnectEvent(player={:?}, reason={})",
            self.player(),
            self.reason()
        )
    }
}
impl From<player::PlayerDisconnectEvent> for PlayerDisconnectEvent {
    fn from(event: player::PlayerDisconnectEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerDisconnectEvent {
    pub fn new(player: PlayerPy, reason: i32) -> Self {
        Self {
            inner: player::PlayerDisconnectEvent {
                player_id: player.get_id(),
                reason,
            },
        }
    }
}
impl PyEvent for PlayerDisconnectEvent {
    fn event_name(&self) -> String {
        "PlayerDisconnectEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerRequestClassEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerRequestClassEvent {
    pub inner: player::PlayerRequestClassEvent,
}
#[pymethods]
impl PlayerRequestClassEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn class_id(&self) -> i32 {
        self.inner.class_id
    }
    fn __repr__(&self) -> String {
        format!(
            "PlayerRequestClassEvent(player={:?}, class_id={})",
            self.player(),
            self.class_id()
        )
    }
}
impl From<player::PlayerRequestClassEvent> for PlayerRequestClassEvent {
    fn from(event: player::PlayerRequestClassEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerRequestClassEvent {
    pub fn new(player: PlayerPy, class_id: i32) -> Self {
        Self {
            inner: player::PlayerRequestClassEvent {
                player_id: player.get_id(),
                class_id,
            },
        }
    }
}
impl PyEvent for PlayerRequestClassEvent {
    fn event_name(&self) -> String {
        "PlayerRequestClassEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerSpawnEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerSpawnEvent {
    pub inner: player::PlayerSpawnEvent,
}
#[pymethods]
impl PlayerSpawnEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    fn __repr__(&self) -> String {
        format!("PlayerSpawnEvent(player={:?})", self.player())
    }
}
impl From<player::PlayerSpawnEvent> for PlayerSpawnEvent {
    fn from(event: player::PlayerSpawnEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerSpawnEvent {
    pub fn new(player: PlayerPy) -> Self {
        Self {
            inner: player::PlayerSpawnEvent {
                player_id: player.get_id(),
            },
        }
    }
}
impl PyEvent for PlayerSpawnEvent {
    fn event_name(&self) -> String {
        "PlayerSpawnEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerRequestSpawnEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerRequestSpawnEvent {
    pub inner: player::PlayerRequestSpawnEvent,
}
#[pymethods]
impl PlayerRequestSpawnEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    fn __repr__(&self) -> String {
        format!("PlayerRequestSpawnEvent(player={:?})", self.player())
    }
}
impl From<player::PlayerRequestSpawnEvent> for PlayerRequestSpawnEvent {
    fn from(event: player::PlayerRequestSpawnEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerRequestSpawnEvent {
    pub fn new(player: PlayerPy) -> Self {
        Self {
            inner: player::PlayerRequestSpawnEvent {
                player_id: player.get_id(),
            },
        }
    }
}
impl PyEvent for PlayerRequestSpawnEvent {
    fn event_name(&self) -> String {
        "PlayerRequestSpawnEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerDeathEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerDeathEvent {
    pub inner: player::PlayerDeathEvent,
}
#[pymethods]
impl PlayerDeathEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn killer(&self) -> Option<PlayerPy> {
        let pool = ENTITY_POOL.lock().unwrap();
        pool.get_player(self.inner.killer_id).map(|p| *p)
    }
    #[getter]
    fn reason(&self) -> i32 {
        self.inner.reason
    }
    #[getter]
    fn body(&self) -> i32 {
        self.inner.body
    }
    fn __repr__(&self) -> String {
        format!(
            "PlayerDeathEvent(player={:?}, killer={:?}, reason={}, body={})",
            self.player(),
            self.killer(),
            self.reason(),
            self.body()
        )
    }
}
impl From<player::PlayerDeathEvent> for PlayerDeathEvent {
    fn from(event: player::PlayerDeathEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerDeathEvent {
    pub fn new(player: PlayerPy, killer: Option<PlayerPy>, reason: i32, body: i32) -> Self {
        Self {
            inner: player::PlayerDeathEvent {
                player_id: player.get_id(),
                killer_id: killer.map(|p| p.get_id()).unwrap_or(-1),
                reason,
                body,
            },
        }
    }
}
impl PyEvent for PlayerDeathEvent {
    fn event_name(&self) -> String {
        "PlayerDeathEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerUpdateEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerUpdateEvent {
    pub inner: player::PlayerUpdateEvent,
}
#[pymethods]
impl PlayerUpdateEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn update(&self) -> i32 {
        self.inner.update
    }
    fn __repr__(&self) -> String {
        format!(
            "PlayerUpdateEvent(player={:?}, update={})",
            self.player(),
            self.update()
        )
    }
}
impl From<player::PlayerUpdateEvent> for PlayerUpdateEvent {
    fn from(event: player::PlayerUpdateEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerUpdateEvent {
    pub fn new(player: PlayerPy, update: i32) -> Self {
        Self {
            inner: player::PlayerUpdateEvent {
                player_id: player.get_id(),
                update,
            },
        }
    }
}
impl PyEvent for PlayerUpdateEvent {
    fn event_name(&self) -> String {
        "PlayerUpdateEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerRequestEnterVehicleEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerRequestEnterVehicleEvent {
    pub inner: player::PlayerRequestEnterVehicleEvent,
}
#[pymethods]
impl PlayerRequestEnterVehicleEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn vehicle(&self) -> VehiclePy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_vehicle(self.inner.vehicle_id).unwrap()
    }
    #[getter]
    fn slot_index(&self) -> i32 {
        self.inner.slot_index
    }
    fn __repr__(&self) -> String {
        format!(
            "PlayerRequestEnterVehicleEvent(player={:?}, vehicle={:?}, slot_index={})",
            self.player(),
            self.vehicle(),
            self.slot_index()
        )
    }
}
impl From<player::PlayerRequestEnterVehicleEvent> for PlayerRequestEnterVehicleEvent {
    fn from(event: player::PlayerRequestEnterVehicleEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerRequestEnterVehicleEvent {
    pub fn new(player: PlayerPy, vehicle: VehiclePy, slot_index: i32) -> Self {
        Self {
            inner: player::PlayerRequestEnterVehicleEvent {
                player_id: player.get_id(),
                vehicle_id: vehicle.get_id(),
                slot_index,
            },
        }
    }
}
impl PyEvent for PlayerRequestEnterVehicleEvent {
    fn event_name(&self) -> String {
        "PlayerRequestEnterVehicleEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerEnterVehicleEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerEnterVehicleEvent {
    pub inner: player::PlayerEnterVehicleEvent,
}
#[pymethods]
impl PlayerEnterVehicleEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn vehicle(&self) -> VehiclePy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_vehicle(self.inner.vehicle_id).unwrap()
    }
    #[getter]
    fn slot_index(&self) -> i32 {
        self.inner.slot_index
    }
    fn __repr__(&self) -> String {
        format!(
            "PlayerEnterVehicleEvent(player={:?}, vehicle={:?}, slot_index={})",
            self.player(),
            self.vehicle(),
            self.slot_index()
        )
    }
}
impl From<player::PlayerEnterVehicleEvent> for PlayerEnterVehicleEvent {
    fn from(event: player::PlayerEnterVehicleEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerEnterVehicleEvent {
    pub fn new(player: PlayerPy, vehicle: VehiclePy, slot_index: i32) -> Self {
        Self {
            inner: player::PlayerEnterVehicleEvent {
                player_id: player.get_id(),
                vehicle_id: vehicle.get_id(),
                slot_index,
            },
        }
    }
}
impl PyEvent for PlayerEnterVehicleEvent {
    fn event_name(&self) -> String {
        "PlayerEnterVehicleEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerExitVehicleEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerExitVehicleEvent {
    pub inner: player::PlayerExitVehicleEvent,
}
#[pymethods]
impl PlayerExitVehicleEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn vehicle(&self) -> VehiclePy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_vehicle(self.inner.vehicle_id).unwrap()
    }
    fn __repr__(&self) -> String {
        format!(
            "PlayerExitVehicleEvent(player={:?}, vehicle={:?})",
            self.player(),
            self.vehicle()
        )
    }
}
impl From<player::PlayerExitVehicleEvent> for PlayerExitVehicleEvent {
    fn from(event: player::PlayerExitVehicleEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerExitVehicleEvent {
    pub fn new(player: PlayerPy, vehicle: VehiclePy) -> Self {
        Self {
            inner: player::PlayerExitVehicleEvent {
                player_id: player.get_id(),
                vehicle_id: vehicle.get_id(),
            },
        }
    }
}
impl PyEvent for PlayerExitVehicleEvent {
    fn event_name(&self) -> String {
        "PlayerExitVehicleEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerNameChangeEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerNameChangeEvent {
    pub inner: player::PlayerNameChangeEvent,
}
#[pymethods]
impl PlayerNameChangeEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn old_name(&self) -> String {
        self.inner.old_name.clone()
    }
    #[getter]
    fn new_name(&self) -> String {
        self.inner.new_name.clone()
    }
    fn __repr__(&self) -> String {
        format!(
            "PlayerNameChangeEvent(player={:?}, old_name={}, new_name={})",
            self.player(),
            self.old_name(),
            self.new_name()
        )
    }
}
impl From<player::PlayerNameChangeEvent> for PlayerNameChangeEvent {
    fn from(event: player::PlayerNameChangeEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerNameChangeEvent {
    pub fn new(player: PlayerPy, old_name: String, new_name: String) -> Self {
        Self {
            inner: player::PlayerNameChangeEvent {
                player_id: player.get_id(),
                old_name,
                new_name,
            },
        }
    }
}
impl PyEvent for PlayerNameChangeEvent {
    fn event_name(&self) -> String {
        "PlayerNameChangeEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerStateChangeEvent {
    pub inner: player::PlayerStateChangeEvent,
}

#[pymethods]
impl PlayerStateChangeEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn old_state(&self) -> i32 {
        self.inner.old_state
    }
    #[getter]
    fn new_state(&self) -> i32 {
        self.inner.new_state
    }
    fn __repr__(&self) -> String {
        format!(
            "PlayerStateChangeEvent(player={:?}, old_state={}, new_state={})",
            self.player(),
            self.old_state(),
            self.new_state()
        )
    }
}
impl From<player::PlayerStateChangeEvent> for PlayerStateChangeEvent {
    fn from(event: player::PlayerStateChangeEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerStateChangeEvent {
    pub fn new(player: PlayerPy, old_state: i32, new_state: i32) -> Self {
        Self {
            inner: player::PlayerStateChangeEvent {
                player_id: player.get_id(),
                old_state,
                new_state,
            },
        }
    }
}
impl PyEvent for PlayerStateChangeEvent {
    fn event_name(&self) -> String {
        "PlayerStateChangeEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerActionChangeEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerActionChangeEvent {
    pub inner: player::PlayerActionChangeEvent,
}
#[pymethods]
impl PlayerActionChangeEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn old_action(&self) -> i32 {
        self.inner.old_action
    }
    #[getter]
    fn new_action(&self) -> i32 {
        self.inner.new_action
    }
    fn __repr__(&self) -> String {
        format!(
            "PlayerActionChangeEvent(player={:?}, old_action={}, new_action={})",
            self.player(),
            self.old_action(),
            self.new_action()
        )
    }
}
impl From<player::PlayerActionChangeEvent> for PlayerActionChangeEvent {
    fn from(event: player::PlayerActionChangeEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerActionChangeEvent {
    pub fn new(player: PlayerPy, old_action: i32, new_action: i32) -> Self {
        Self {
            inner: player::PlayerActionChangeEvent {
                player_id: player.get_id(),
                old_action,
                new_action,
            },
        }
    }
}
impl PyEvent for PlayerActionChangeEvent {
    fn event_name(&self) -> String {
        "PlayerActionChangeEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerOnFireChangeEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerOnFireChangeEvent {
    pub inner: player::PlayerOnFireChangeEvent,
}
#[pymethods]
impl PlayerOnFireChangeEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn is_on_fire(&self) -> bool {
        self.inner.is_on_fire
    }
    fn __repr__(&self) -> String {
        format!(
            "PlayerOnFireChangeEvent(player={:?}, is_on_fire={})",
            self.player(),
            self.is_on_fire()
        )
    }
}
impl From<player::PlayerOnFireChangeEvent> for PlayerOnFireChangeEvent {
    fn from(event: player::PlayerOnFireChangeEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerOnFireChangeEvent {
    pub fn new(player: PlayerPy, is_on_fire: bool) -> Self {
        Self {
            inner: player::PlayerOnFireChangeEvent {
                player_id: player.get_id(),
                is_on_fire,
            },
        }
    }
}
impl PyEvent for PlayerOnFireChangeEvent {
    fn event_name(&self) -> String {
        "PlayerOnFireChangeEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerCrouchChangeEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerCrouchChangeEvent {
    pub inner: player::PlayerCrouchChangeEvent,
}
#[pymethods]
impl PlayerCrouchChangeEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn is_crouching(&self) -> bool {
        self.inner.is_crouching
    }
    fn __repr__(&self) -> String {
        format!(
            "PlayerCrouchChangeEvent(player={:?}, is_crouching={})",
            self.player(),
            self.is_crouching()
        )
    }
}
impl From<player::PlayerCrouchChangeEvent> for PlayerCrouchChangeEvent {
    fn from(event: player::PlayerCrouchChangeEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerCrouchChangeEvent {
    pub fn new(player: PlayerPy, is_crouching: bool) -> Self {
        Self {
            inner: player::PlayerCrouchChangeEvent {
                player_id: player.get_id(),
                is_crouching,
            },
        }
    }
}
impl PyEvent for PlayerCrouchChangeEvent {
    fn event_name(&self) -> String {
        "PlayerCrouchChangeEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerGameKeysChangeEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerGameKeysChangeEvent {
    pub inner: player::PlayerGameKeysChangeEvent,
}
#[pymethods]
impl PlayerGameKeysChangeEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn old_keys(&self) -> u32 {
        self.inner.old_keys
    }
    #[getter]
    fn new_keys(&self) -> u32 {
        self.inner.new_keys
    }
    fn __repr__(&self) -> String {
        format!(
            "PlayerGameKeysChangeEvent(player={:?}, old_keys={}, new_keys={})",
            self.player(),
            self.old_keys(),
            self.new_keys()
        )
    }
}
impl From<player::PlayerGameKeysChangeEvent> for PlayerGameKeysChangeEvent {
    fn from(event: player::PlayerGameKeysChangeEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerGameKeysChangeEvent {
    pub fn new(player: PlayerPy, old_keys: u32, new_keys: u32) -> Self {
        Self {
            inner: player::PlayerGameKeysChangeEvent {
                player_id: player.get_id(),
                old_keys,
                new_keys,
            },
        }
    }
}
impl PyEvent for PlayerGameKeysChangeEvent {
    fn event_name(&self) -> String {
        "PlayerGameKeysChangeEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerBeginTypingEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerBeginTypingEvent {
    pub inner: player::PlayerBeginTypingEvent,
}
#[pymethods]
impl PlayerBeginTypingEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    fn __repr__(&self) -> String {
        format!("PlayerBeginTypingEvent(player={:?})", self.player())
    }
}
impl From<player::PlayerBeginTypingEvent> for PlayerBeginTypingEvent {
    fn from(event: player::PlayerBeginTypingEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerBeginTypingEvent {
    pub fn new(player: PlayerPy) -> Self {
        Self {
            inner: player::PlayerBeginTypingEvent {
                player_id: player.get_id(),
            },
        }
    }
}
impl PyEvent for PlayerBeginTypingEvent {
    fn event_name(&self) -> String {
        "PlayerBeginTypingEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerEndTypingEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerEndTypingEvent {
    pub inner: player::PlayerEndTypingEvent,
}
#[pymethods]
impl PlayerEndTypingEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    fn __repr__(&self) -> String {
        format!("PlayerEndTypingEvent(player={:?})", self.player())
    }
}
impl From<player::PlayerEndTypingEvent> for PlayerEndTypingEvent {
    fn from(event: player::PlayerEndTypingEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerEndTypingEvent {
    pub fn new(player: PlayerPy) -> Self {
        Self {
            inner: player::PlayerEndTypingEvent {
                player_id: player.get_id(),
            },
        }
    }
}
impl PyEvent for PlayerEndTypingEvent {
    fn event_name(&self) -> String {
        "PlayerEndTypingEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerAwayChangeEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerAwayChangeEvent {
    pub inner: player::PlayerAwayChangeEvent,
}
#[pymethods]
impl PlayerAwayChangeEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn is_away(&self) -> bool {
        self.inner.is_away
    }
    fn __repr__(&self) -> String {
        format!(
            "PlayerAwayChangeEvent(player={:?}, is_away={})",
            self.player(),
            self.is_away()
        )
    }
}
impl From<player::PlayerAwayChangeEvent> for PlayerAwayChangeEvent {
    fn from(event: player::PlayerAwayChangeEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerAwayChangeEvent {
    pub fn new(player: PlayerPy, is_away: bool) -> Self {
        Self {
            inner: player::PlayerAwayChangeEvent {
                player_id: player.get_id(),
                is_away,
            },
        }
    }
}
impl PyEvent for PlayerAwayChangeEvent {
    fn event_name(&self) -> String {
        "PlayerAwayChangeEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerMessageEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerMessageEvent {
    pub inner: player::PlayerMessageEvent,
}
#[pymethods]
impl PlayerMessageEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn message(&self) -> String {
        self.inner.message.clone()
    }
    fn __repr__(&self) -> String {
        format!(
            "PlayerMessageEvent(player={:?}, message='{}')",
            self.player(),
            self.message()
        )
    }
}
impl From<player::PlayerMessageEvent> for PlayerMessageEvent {
    fn from(event: player::PlayerMessageEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerMessageEvent {
    pub fn new(player: PlayerPy, message: String) -> Self {
        Self {
            inner: player::PlayerMessageEvent {
                player_id: player.get_id(),
                message,
            },
        }
    }
}
impl PyEvent for PlayerMessageEvent {
    fn event_name(&self) -> String {
        "PlayerMessageEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerCommandEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerCommandEvent {
    pub inner: player::PlayerCommandEvent,
}
#[pymethods]
impl PlayerCommandEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn command(&self) -> String {
        self.inner.command.clone()
    }
    #[getter]
    fn text(&self) -> String {
        self.inner.text.clone()
    }
    #[getter]
    fn args(&self) -> Vec<String> {
        let mut res = vec![];
        // text split ' ' and strip() '  '
        for arg in self.text().split(' ').filter(|s| !s.is_empty()) {
            res.push(arg.to_string())
        }
        res
    }

    fn __repr__(&self) -> String {
        format!(
            "PlayerCommandEvent(player={:?}, command='{}', text='{}')",
            self.player(),
            self.command(),
            self.text()
        )
    }
}
impl From<player::PlayerCommandEvent> for PlayerCommandEvent {
    fn from(event: player::PlayerCommandEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerCommandEvent {
    pub fn new(player: PlayerPy, command: String, text: String) -> Self {
        Self {
            inner: player::PlayerCommandEvent {
                player_id: player.get_id(),
                command,
                text,
            },
        }
    }
}
impl PyEvent for PlayerCommandEvent {
    fn event_name(&self) -> String {
        "PlayerCommandEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerPrivateMessageEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerPrivateMessageEvent {
    pub inner: player::PlayerPrivateMessageEvent,
}
#[pymethods]
impl PlayerPrivateMessageEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn target(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.target_id).unwrap()
    }
    #[getter]
    fn message(&self) -> String {
        self.inner.message.clone()
    }
    fn __repr__(&self) -> String {
        format!(
            "PlayerPrivateMessageEvent(player={:?}, target={:?}, message='{}')",
            self.player(),
            self.target(),
            self.message()
        )
    }
}
impl From<player::PlayerPrivateMessageEvent> for PlayerPrivateMessageEvent {
    fn from(event: player::PlayerPrivateMessageEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerPrivateMessageEvent {
    pub fn new(player: PlayerPy, target: PlayerPy, message: String) -> Self {
        Self {
            inner: player::PlayerPrivateMessageEvent {
                player_id: player.get_id(),
                target_id: target.get_id(),
                message,
            },
        }
    }
}
impl PyEvent for PlayerPrivateMessageEvent {
    fn event_name(&self) -> String {
        "PlayerPrivateMessageEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerKeyBindDownEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerKeyBindDownEvent {
    pub inner: player::PlayerKeyBindDownEvent,
}
#[pymethods]
impl PlayerKeyBindDownEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn key(&self) -> KeyBindPy {
        get_bindkey(self.inner.bind_id).unwrap()
    }
    fn __repr__(&self) -> String {
        format!(
            "PlayerKeyBindDownEvent(player={:?}, key={:?})",
            self.player(),
            self.key()
        )
    }
}
impl From<player::PlayerKeyBindDownEvent> for PlayerKeyBindDownEvent {
    fn from(event: player::PlayerKeyBindDownEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerKeyBindDownEvent {
    pub fn new(player: PlayerPy, bind_id: i32) -> Self {
        Self {
            inner: player::PlayerKeyBindDownEvent {
                player_id: player.get_id(),
                bind_id,
            },
        }
    }
}
impl PyEvent for PlayerKeyBindDownEvent {
    fn event_name(&self) -> String {
        "PlayerKeyBindDownEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerKeyBindUpEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerKeyBindUpEvent {
    pub inner: player::PlayerKeyBindUpEvent,
}
#[pymethods]
impl PlayerKeyBindUpEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn key(&self) -> KeyBindPy {
        get_bindkey(self.inner.bind_id).unwrap()
    }
    fn __repr__(&self) -> String {
        format!(
            "PlayerKeyBindUpEvent(player={:?}, key={:?})",
            self.player(),
            self.key()
        )
    }
}
impl From<player::PlayerKeyBindUpEvent> for PlayerKeyBindUpEvent {
    fn from(event: player::PlayerKeyBindUpEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerKeyBindUpEvent {
    pub fn new(player: PlayerPy, bind_id: i32) -> Self {
        Self {
            inner: player::PlayerKeyBindUpEvent {
                player_id: player.get_id(),
                bind_id,
            },
        }
    }
}
impl PyEvent for PlayerKeyBindUpEvent {
    fn event_name(&self) -> String {
        "PlayerKeyBindUpEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerSpectateEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerSpectateEvent {
    pub inner: player::PlayerSpectateEvent,
}
#[pymethods]
impl PlayerSpectateEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn target(&self) -> Option<PlayerPy> {
        let pool = ENTITY_POOL.lock().unwrap();
        pool.get_player(self.inner.target_id).map(|p| *p)
    }
    fn __repr__(&self) -> String {
        format!(
            "PlayerSpectateEvent(player={:?}, target={:?})",
            self.player(),
            self.target()
        )
    }
}
impl From<player::PlayerSpectateEvent> for PlayerSpectateEvent {
    fn from(event: player::PlayerSpectateEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerSpectateEvent {
    pub fn new(player: PlayerPy, target: Option<PlayerPy>) -> Self {
        Self {
            inner: player::PlayerSpectateEvent {
                player_id: player.get_id(),
                target_id: target.map(|p| p.get_id()).unwrap_or(-1),
            },
        }
    }
}
impl PyEvent for PlayerSpectateEvent {
    fn event_name(&self) -> String {
        "PlayerSpectateEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerCrashReportEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerCrashReportEvent {
    pub inner: player::PlayerCrashReportEvent,
}
#[pymethods]
impl PlayerCrashReportEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn report(&self) -> String {
        self.inner.report.clone()
    }
    fn __repr__(&self) -> String {
        format!(
            "PlayerCrashReportEvent(player={:?}, report='{}')",
            self.player(),
            self.report()
        )
    }
}
impl From<player::PlayerCrashReportEvent> for PlayerCrashReportEvent {
    fn from(event: player::PlayerCrashReportEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerCrashReportEvent {
    pub fn new(player: PlayerPy, report: String) -> Self {
        Self {
            inner: player::PlayerCrashReportEvent {
                player_id: player.get_id(),
                report,
            },
        }
    }
}
impl PyEvent for PlayerCrashReportEvent {
    fn event_name(&self) -> String {
        "PlayerCrashReportEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// PlayerModuleListEvent
#[derive(Debug, Clone)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerModuleListEvent {
    pub inner: player::PlayerModuleListEvent,
}
#[pymethods]
impl PlayerModuleListEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.inner.player_id).unwrap()
    }
    #[getter]
    fn modules(&self) -> String {
        self.inner.modules.clone()
    }
    fn __repr__(&self) -> String {
        format!(
            "PlayerModuleListEvent(player={:?}, modules='{}')",
            self.player(),
            self.modules()
        )
    }
}
impl From<player::PlayerModuleListEvent> for PlayerModuleListEvent {
    fn from(event: player::PlayerModuleListEvent) -> Self {
        Self { inner: event }
    }
}
impl PlayerModuleListEvent {
    pub fn new(player: PlayerPy, modules: String) -> Self {
        Self {
            inner: player::PlayerModuleListEvent {
                player_id: player.get_id(),
                modules,
            },
        }
    }
}
impl PyEvent for PlayerModuleListEvent {
    fn event_name(&self) -> String {
        "PlayerModuleListEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

// Extra

#[derive(Debug, Clone, Copy)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerHealthChangeEvent {
    pub player_id: i32,
    pub old_health: f32,
    pub new_health: f32,
    pub current_health: f32,
}
#[pymethods]
impl PlayerHealthChangeEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.player_id).unwrap()
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
            let mut pool = ENTITY_POOL.lock().unwrap();
            let player = pool.get_mut_player(self.player_id).unwrap();
            player.set_var_last_health(self.current_health);
        };
    }

    fn __repr__(&self) -> String {
        format!(
            "PlayerHealthChangeEvent(player={:?}, old_health={}, new_health={})",
            self.player(),
            self.get_old_health(),
            self.get_new_health()
        )
    }
}
impl From<(i32, f32, f32)> for PlayerHealthChangeEvent {
    fn from(value: (i32, f32, f32)) -> Self {
        let mut this = Self {
            player_id: value.0,
            old_health: value.1,
            new_health: value.2,
            current_health: 0.0,
        };
        this.set_current_health(value.2);
        this
    }
}
impl PlayerHealthChangeEvent {
    pub fn new(player: PlayerPy, old_health: f32, new_health: f32) -> Self {
        let mut this = Self {
            player_id: player.get_id(),
            old_health,
            new_health,
            current_health: 0.0,
        };
        this.set_current_health(new_health);
        this
    }
}
impl PyEvent for PlayerHealthChangeEvent {
    fn event_name(&self) -> String {
        "PlayerHealthChangeEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

#[derive(Debug, Clone, Copy)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerArmourChangeEvent {
    pub player_id: i32,
    pub old_armour: f32,
    pub new_armour: f32,
    pub current_armour: f32,
}
#[pymethods]
impl PlayerArmourChangeEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.player_id).unwrap()
    }

    #[getter]
    fn get_old_armour(&self) -> f32 {
        self.old_armour
    }

    #[getter]
    fn get_new_armour(&self) -> f32 {
        self.new_armour
    }

    #[getter]
    fn get_current_armour(&self) -> f32 {
        self.current_armour
    }

    #[setter]
    fn set_current_armour(&mut self, armour: f32) {
        self.current_armour = armour;
        {
            let mut pool = ENTITY_POOL.lock().unwrap();
            let player = pool.get_mut_player(self.player_id).unwrap();
            player.set_var_last_armour(self.current_armour);
        };
    }

    fn __repr__(&self) -> String {
        format!(
            "PlayerArmourChangeEvent(player={:?}, old_armour={}, new_armour={})",
            self.player(),
            self.get_old_armour(),
            self.get_new_armour()
        )
    }
}
impl From<(i32, f32, f32)> for PlayerArmourChangeEvent {
    fn from(value: (i32, f32, f32)) -> Self {
        let mut this = Self {
            player_id: value.0,
            old_armour: value.1,
            new_armour: value.2,
            current_armour: 0.0,
        };
        this.set_current_armour(value.2);
        this
    }
}
impl PlayerArmourChangeEvent {
    pub fn new(player: PlayerPy, old_armour: f32, new_armour: f32) -> Self {
        let mut this = Self {
            player_id: player.get_id(),
            old_armour,
            new_armour,
            current_armour: 0.0,
        };
        this.set_current_armour(new_armour);
        this
    }
}
impl PyEvent for PlayerArmourChangeEvent {
    fn event_name(&self) -> String {
        "PlayerArmourChangeEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

#[derive(Debug, Clone, Copy)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerWeaponChangeEvent {
    pub player_id: i32,
    pub old_weapon: i32,
    pub new_weapon: i32,
    pub current_weapon: i32,
}
#[pymethods]
impl PlayerWeaponChangeEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.player_id).unwrap()
    }

    #[getter]
    fn get_old_weapon(&self) -> i32 {
        self.old_weapon
    }

    #[getter]
    fn get_new_weapon(&self) -> i32 {
        self.new_weapon
    }

    #[getter]
    fn get_current_weapon(&self) -> i32 {
        self.current_weapon
    }

    #[setter]
    fn set_current_weapon(&mut self, weapon: i32) {
        self.current_weapon = weapon;
        {
            let mut pool = ENTITY_POOL.lock().unwrap();
            let player = pool.get_mut_player(self.player_id).unwrap();
            player.set_var_last_weapon(self.current_weapon);
        };
    }

    fn __repr__(&self) -> String {
        format!(
            "PlayerWeaponChangeEvent(player={:?}, old_weapon={}, new_weapon={})",
            self.player(),
            self.get_old_weapon(),
            self.get_new_weapon()
        )
    }
}
impl From<(i32, i32, i32)> for PlayerWeaponChangeEvent {
    fn from(value: (i32, i32, i32)) -> Self {
        let mut this = Self {
            player_id: value.0,
            old_weapon: value.1,
            new_weapon: value.2,
            current_weapon: 0,
        };
        this.set_current_weapon(value.2);
        this
    }
}
impl PlayerWeaponChangeEvent {
    pub fn new(player: PlayerPy, old_weapon: i32, new_weapon: i32) -> Self {
        let mut this = Self {
            player_id: player.get_id(),
            old_weapon,
            new_weapon,
            current_weapon: 0,
        };
        this.set_current_weapon(new_weapon);
        this
    }
}
impl PyEvent for PlayerWeaponChangeEvent {
    fn event_name(&self) -> String {
        "PlayerWeaponChangeEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

#[derive(Debug, Clone, Copy)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerAmmoChangeEvent {
    pub player_id: i32,
    pub old_ammo: i32,
    pub new_ammo: i32,
    pub current_ammo: i32,
}
#[pymethods]
impl PlayerAmmoChangeEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.player_id).unwrap()
    }

    #[getter]
    fn get_old_ammo(&self) -> i32 {
        self.old_ammo
    }

    #[getter]
    fn get_new_ammo(&self) -> i32 {
        self.new_ammo
    }

    #[getter]
    fn get_current_ammo(&self) -> i32 {
        self.current_ammo
    }

    #[setter]
    fn set_current_ammo(&mut self, ammo: i32) {
        self.current_ammo = ammo;
        {
            let mut pool = ENTITY_POOL.lock().unwrap();
            let player = pool.get_mut_player(self.player_id).unwrap();
            player.set_var_last_ammo(self.current_ammo);
        };
    }

    fn __repr__(&self) -> String {
        format!(
            "PlayerWeaponChangeEvent(player={:?}, old_ammo={}, new_ammo={})",
            self.player(),
            self.get_old_ammo(),
            self.get_new_ammo()
        )
    }
}
impl From<(i32, i32, i32)> for PlayerAmmoChangeEvent {
    fn from(value: (i32, i32, i32)) -> Self {
        let mut this = Self {
            player_id: value.0,
            old_ammo: value.1,
            new_ammo: value.2,
            current_ammo: 0,
        };
        this.set_current_ammo(value.2);
        this
    }
}
impl PlayerAmmoChangeEvent {
    pub fn new(player: PlayerPy, old_ammo: i32, new_ammo: i32) -> Self {
        let mut this = Self {
            player_id: player.get_id(),
            old_ammo,
            new_ammo,
            current_ammo: 0,
        };
        this.set_current_ammo(new_ammo);
        this
    }
}
impl PyEvent for PlayerAmmoChangeEvent {
    fn event_name(&self) -> String {
        "PlayerAmmoChangeEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

#[derive(Debug, Clone, Copy)]
#[pyclass(extends=PlayerEvent, subclass)]
pub struct PlayerMoveEvent {
    pub player_id: i32,
    pub old_position: VectorPy,
    pub new_position: VectorPy,
    pub current_position: VectorPy,
}
#[pymethods]
impl PlayerMoveEvent {
    #[getter]
    fn player(&self) -> PlayerPy {
        let pool = ENTITY_POOL.lock().unwrap();
        *pool.get_player(self.player_id).unwrap()
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
            let mut pool = ENTITY_POOL.lock().unwrap();
            let player = pool.get_mut_player(self.player_id).unwrap();
            player.set_var_last_position(self.current_position.get_entity_pos());
        };
    }

    fn __repr__(&self) -> String {
        format!(
            "PlayerMoveEvent(player={:?}, old_position={:?}, new_position={:?})",
            self.player(),
            self.get_old_position(),
            self.get_new_position()
        )
    }
}
impl From<(i32, VectorPy, VectorPy)> for PlayerMoveEvent {
    fn from(value: (i32, VectorPy, VectorPy)) -> Self {
        let mut this = Self {
            player_id: value.0,
            old_position: value.1,
            new_position: value.2,
            current_position: VectorPy::default(),
        };
        this.set_current_position(value.2);
        this
    }
}
impl PlayerMoveEvent {
    pub fn new(player: PlayerPy, old_position: VectorPy, new_position: VectorPy) -> Self {
        let mut this = Self {
            player_id: player.get_id(),
            old_position,
            new_position,
            current_position: VectorPy::default(),
        };
        this.set_current_position(new_position);
        this
    }
}
impl PyEvent for PlayerMoveEvent {
    fn event_name(&self) -> String {
        "PlayerMoveEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(PlayerEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PlayerEvent>()?;
    m.add_class::<IncomingConnectionEvent>()?;
    m.add_class::<ClientScriptDataEvent>()?;
    m.add_class::<PlayerConnectEvent>()?;
    m.add_class::<PlayerDisconnectEvent>()?;
    m.add_class::<PlayerRequestClassEvent>()?;
    m.add_class::<PlayerSpawnEvent>()?;
    m.add_class::<PlayerRequestSpawnEvent>()?;
    m.add_class::<PlayerDeathEvent>()?;
    m.add_class::<PlayerUpdateEvent>()?;
    m.add_class::<PlayerRequestEnterVehicleEvent>()?;
    m.add_class::<PlayerEnterVehicleEvent>()?;
    m.add_class::<PlayerExitVehicleEvent>()?;
    m.add_class::<PlayerNameChangeEvent>()?;
    m.add_class::<PlayerStateChangeEvent>()?;
    m.add_class::<PlayerActionChangeEvent>()?;
    m.add_class::<PlayerOnFireChangeEvent>()?;
    m.add_class::<PlayerCrouchChangeEvent>()?;
    m.add_class::<PlayerGameKeysChangeEvent>()?;
    m.add_class::<PlayerBeginTypingEvent>()?;
    m.add_class::<PlayerEndTypingEvent>()?;
    m.add_class::<PlayerAwayChangeEvent>()?;
    m.add_class::<PlayerMessageEvent>()?;
    m.add_class::<PlayerCommandEvent>()?;
    m.add_class::<PlayerPrivateMessageEvent>()?;
    m.add_class::<PlayerKeyBindDownEvent>()?;
    m.add_class::<PlayerKeyBindUpEvent>()?;
    m.add_class::<PlayerSpectateEvent>()?;
    m.add_class::<PlayerCrashReportEvent>()?;
    m.add_class::<PlayerModuleListEvent>()?;

    // Extra
    m.add_class::<PlayerAmmoChangeEvent>()?;
    m.add_class::<PlayerMoveEvent>()?;
    m.add_class::<PlayerHealthChangeEvent>()?;
    m.add_class::<PlayerArmourChangeEvent>()?;
    m.add_class::<PlayerWeaponChangeEvent>()?;

    Ok(())
}
