use std::ffi::c_char;

use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{Bound, PyResult, Python, pyclass, pymethods};
use vcmp_bindings::events::player;

use crate::functions::player::PlayerPy;
use crate::functions::vehicle::VehiclePy;
use crate::pool::ENTITY_POOL;
use crate::py::events::{BaseEvent, PyBaseEvent};
// rewrite this to use pyclass

#[pyclass(extends=BaseEvent, subclass)]
#[derive(Debug, Clone, Default)]
#[pyo3(name = "PlayerEvent")]
pub struct PlayerEvent;

#[pymethods]
impl PlayerEvent {
    #[new]
    pub fn new(name: &str) -> (Self, BaseEvent) {
        (Self, BaseEvent::new(name))
    }
}

impl PyBaseEvent for PlayerEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(py, PlayerEvent::new("PlayerEvent"))?;
        Ok(value.into())
    }
}

#[pyclass(extends=PlayerEvent, unsendable)]
#[derive(Debug, Clone)]
#[pyo3(name = "IncomingConnectionEvent")]
pub struct IncomingConnectionEvent {
    inner: player::IncomingConnectionEvent,
}

impl From<(*mut c_char, usize, *const c_char, *const c_char)> for IncomingConnectionEvent {
    fn from(value: (*mut c_char, usize, *const c_char, *const c_char)) -> Self {
        Self {
            inner: player::IncomingConnectionEvent::from(value),
        }
    }
}

impl IncomingConnectionEvent {
    pub fn new(inner: player::IncomingConnectionEvent) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("IncomingConnectionEvent"))
            .add_subclass(Self { inner })
    }
}

#[pymethods]
impl IncomingConnectionEvent {
    #[new]
    pub fn py_new(player_name: String, password: String, ip: String) -> PyClassInitializer<Self> {
        // (*mut c_char, usize, *const c_char, *const c_char)
        Self::new(player::IncomingConnectionEvent::from((
            player_name.as_str().as_ptr() as *mut c_char,
            player_name.len(),
            password.as_str().as_ptr() as *const c_char,
            ip.as_str().as_ptr() as *const c_char,
        )))
    }
}

impl PyBaseEvent for IncomingConnectionEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(py, IncomingConnectionEvent::new(self.inner.clone()))?;
        Ok(value.into())
    }
}

#[pyclass(extends=PlayerEvent, unsendable)]
#[derive(Debug, Clone)]
#[pyo3(name = "PlayerConnectEvent")]
pub struct PlayerConnectEvent {
    player_id: i32,
}

#[pymethods]
impl PlayerConnectEvent {
    #[new]
    pub fn new(player_id: i32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerConnectEvent"))
            .add_subclass(Self { player_id })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }
}

impl From<player::PlayerConnectEvent> for PlayerConnectEvent {
    fn from(value: player::PlayerConnectEvent) -> Self {
        Self {
            player_id: value.player_id,
        }
    }
}

impl PyBaseEvent for PlayerConnectEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(py, PlayerConnectEvent::new(self.player_id))?;
        Ok(value.into())
    }
}

#[pyclass(extends=PlayerEvent, unsendable)]
#[derive(Debug, Clone)]
#[pyo3(name = "PlayerDisconnectEvent")]
pub struct PlayerDisconnectEvent {
    player_id: i32,
    reason: i32,
}

#[pymethods]
impl PlayerDisconnectEvent {
    #[new]
    pub fn new(player_id: i32, reason: i32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerDisconnectEvent"))
            .add_subclass(Self { player_id, reason })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }
}

impl From<player::PlayerDisconnectEvent> for PlayerDisconnectEvent {
    fn from(value: player::PlayerDisconnectEvent) -> Self {
        Self {
            player_id: value.player_id,
            reason: value.reason,
        }
    }
}

impl PyBaseEvent for PlayerDisconnectEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(py, PlayerDisconnectEvent::new(self.player_id, self.reason))?;
        Ok(value.into())
    }
}

#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone)]
#[pyo3(name = "ClientScriptDataEvent")]
pub struct ClientScriptDataEvent {
    player_id: i32,
    data: Vec<u8>,
}

#[pymethods]
impl ClientScriptDataEvent {
    #[new]
    pub fn new(player_id: i32, data: Vec<u8>) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("ClientScriptDataEvent"))
            .add_subclass(Self { player_id, data })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }
}

impl From<player::ClientScriptDataEvent> for ClientScriptDataEvent {
    fn from(value: player::ClientScriptDataEvent) -> Self {
        Self {
            player_id: value.player_id,
            data: value.data,
        }
    }
}

impl PyBaseEvent for ClientScriptDataEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(
            py,
            ClientScriptDataEvent::new(self.player_id, self.data.clone()),
        )?;
        Ok(value.into())
    }
}

#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone)]
#[pyo3(name = "PlayerRequestClassEvent")]
pub struct PlayerRequestClassEvent {
    player_id: i32,
    class_id: i32,
}

#[pymethods]
impl PlayerRequestClassEvent {
    #[new]
    pub fn new(player_id: i32, class_id: i32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerRequestClassEvent")).add_subclass(Self {
            player_id,
            class_id,
        })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }
}

impl From<player::PlayerRequestClassEvent> for PlayerRequestClassEvent {
    fn from(value: player::PlayerRequestClassEvent) -> Self {
        Self {
            player_id: value.player_id,
            class_id: value.class_id,
        }
    }
}

impl PyBaseEvent for PlayerRequestClassEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(
            py,
            PlayerRequestClassEvent::new(self.player_id, self.class_id),
        )?;
        Ok(value.into())
    }
}

#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone)]
#[pyo3(name = "PlayerRequestSpawnEvent")]
pub struct PlayerRequestSpawnEvent {
    player_id: i32,
}

#[pymethods]
impl PlayerRequestSpawnEvent {
    #[new]
    pub fn new(player_id: i32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerRequestSpawnEvent"))
            .add_subclass(Self { player_id })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }
}

impl From<player::PlayerRequestSpawnEvent> for PlayerRequestSpawnEvent {
    fn from(value: player::PlayerRequestSpawnEvent) -> Self {
        Self {
            player_id: value.player_id,
        }
    }
}

impl PyBaseEvent for PlayerRequestSpawnEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(py, PlayerRequestSpawnEvent::new(self.player_id))?;
        Ok(value.into())
    }
}

#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone)]
#[pyo3(name = "PlayerSpawnEvent")]
pub struct PlayerSpawnEvent {
    player_id: i32,
}

#[pymethods]
impl PlayerSpawnEvent {
    #[new]
    pub fn new(player_id: i32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerSpawnEvent"))
            .add_subclass(Self { player_id })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }
}

impl From<player::PlayerSpawnEvent> for PlayerSpawnEvent {
    fn from(value: player::PlayerSpawnEvent) -> Self {
        Self {
            player_id: value.player_id,
        }
    }
}

impl PyBaseEvent for PlayerSpawnEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(py, PlayerSpawnEvent::new(self.player_id))?;
        Ok(value.into())
    }
}

#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone)]
#[pyo3(name = "PlayerDeathEvent")]
pub struct PlayerDeathEvent {
    player_id: i32,
    killer_id: i32,
    reason: i32,
    body_part: i32,
}

#[pymethods]
impl PlayerDeathEvent {
    #[new]
    pub fn new(
        player_id: i32,
        killer_id: i32,
        reason: i32,
        body_part: i32,
    ) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerRequestSpawnEvent")).add_subclass(Self {
            player_id,
            killer_id,
            reason,
            body_part,
        })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }

    #[getter]
    pub fn get_killer(&self) -> Option<PlayerPy> {
        let pool = ENTITY_POOL.lock().expect("Failed to lock entity pool");
        pool.get_player(self.killer_id).copied()
    }

    #[getter]
    pub fn get_reason(&self) -> i32 {
        self.reason
    }

    #[getter]
    pub fn get_body_part(&self) -> i32 {
        self.body_part
    }
}

impl From<player::PlayerDeathEvent> for PlayerDeathEvent {
    fn from(value: player::PlayerDeathEvent) -> Self {
        Self {
            player_id: value.player_id,
            killer_id: value.killer_id,
            reason: value.reason,
            body_part: value.body,
        }
    }
}

impl PyBaseEvent for PlayerDeathEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(
            py,
            PlayerDeathEvent::new(self.player_id, self.killer_id, self.reason, self.body_part),
        )?;
        Ok(value.into())
    }
}

/*
    Extra Player Update Event,
    Health, Armour, Ammo, Weapon, etc.
*/
// Player Health
#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "PlayerHealthEvent")]
pub struct PlayerHealthEvent {
    player_id: i32,
    old_health: i32,
    new_health: i32,
}

#[pymethods]
impl PlayerHealthEvent {
    #[new]
    pub fn new(player_id: i32, old_health: i32, new_health: i32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerHealthEvent")).add_subclass(Self {
            player_id,
            old_health,
            new_health,
        })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }

    #[getter]
    pub fn get_old_health(&self) -> i32 {
        self.old_health
    }

    #[getter]
    pub fn get_new_health(&self) -> i32 {
        self.new_health
    }

    #[setter]
    pub fn set_new_health(&mut self, new_health: i32) {
        self.new_health = new_health;
    }
}

impl From<(i32, i32, i32)> for PlayerHealthEvent {
    fn from(value: (i32, i32, i32)) -> Self {
        Self {
            player_id: value.0,
            old_health: value.1,
            new_health: value.2,
        }
    }
}

impl PyBaseEvent for PlayerHealthEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(
            py,
            PlayerHealthEvent::new(self.player_id, self.old_health, self.new_health),
        )?;
        Ok(value.into())
    }
}
// Player Armour
#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "PlayerArmourEvent")]
pub struct PlayerArmourEvent {
    player_id: i32,
    old_armour: i32,
    new_armour: i32,
}

#[pymethods]
impl PlayerArmourEvent {
    #[new]
    pub fn new(player_id: i32, old_armour: i32, new_armour: i32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerArmourEvent")).add_subclass(Self {
            player_id,
            old_armour,
            new_armour,
        })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }

    #[getter]
    pub fn get_old_armour(&self) -> i32 {
        self.old_armour
    }

    #[getter]
    pub fn get_new_armour(&self) -> i32 {
        self.new_armour
    }

    #[setter]
    pub fn set_new_armour(&mut self, new_armour: i32) {
        self.new_armour = new_armour;
    }
}

impl From<(i32, i32, i32)> for PlayerArmourEvent {
    fn from(value: (i32, i32, i32)) -> Self {
        Self {
            player_id: value.0,
            old_armour: value.1,
            new_armour: value.2,
        }
    }
}

impl PyBaseEvent for PlayerArmourEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(
            py,
            PlayerArmourEvent::new(self.player_id, self.old_armour, self.new_armour),
        )?;
        Ok(value.into())
    }
}

// Ammo
#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "PlayerAmmoEvent")]
pub struct PlayerAmmoEvent {
    player_id: i32,
    old_ammo: i32,
    new_ammo: i32,
}

#[pymethods]
impl PlayerAmmoEvent {
    #[new]
    pub fn new(player_id: i32, old_ammo: i32, new_ammo: i32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerAmmoEvent")).add_subclass(Self {
            player_id,
            old_ammo,
            new_ammo,
        })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }

    #[getter]
    pub fn get_old_ammo(&self) -> i32 {
        self.old_ammo
    }

    #[getter]
    pub fn get_new_ammo(&self) -> i32 {
        self.new_ammo
    }

    #[setter]
    pub fn set_new_ammo(&mut self, new_ammo: i32) {
        self.new_ammo = new_ammo;
    }
}

impl From<(i32, i32, i32)> for PlayerAmmoEvent {
    fn from(value: (i32, i32, i32)) -> Self {
        Self {
            player_id: value.0,
            old_ammo: value.1,
            new_ammo: value.2,
        }
    }
}

impl PyBaseEvent for PlayerAmmoEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(
            py,
            PlayerAmmoEvent::new(self.player_id, self.old_ammo, self.new_ammo),
        )?;
        Ok(value.into())
    }
}

// Weapon
#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "PlayerWeaponEvent")]
pub struct PlayerWeaponEvent {
    player_id: i32,
    old_weapon: i32,
    new_weapon: i32,
}

#[pymethods]
impl PlayerWeaponEvent {
    #[new]
    pub fn new(player_id: i32, old_weapon: i32, new_weapon: i32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerWeaponEvent")).add_subclass(Self {
            player_id,
            old_weapon,
            new_weapon,
        })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }

    #[getter]
    pub fn get_old_weapon(&self) -> i32 {
        self.old_weapon
    }

    #[getter]
    pub fn get_new_weapon(&self) -> i32 {
        self.new_weapon
    }

    #[setter]
    pub fn set_new_weapon(&mut self, new_weapon: i32) {
        self.new_weapon = new_weapon;
    }
}

impl From<(i32, i32, i32)> for PlayerWeaponEvent {
    fn from(value: (i32, i32, i32)) -> Self {
        Self {
            player_id: value.0,
            old_weapon: value.1,
            new_weapon: value.2,
        }
    }
}

impl PyBaseEvent for PlayerWeaponEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(
            py,
            PlayerWeaponEvent::new(self.player_id, self.old_weapon, self.new_weapon),
        )?;
        Ok(value.into())
    }
}

#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "PlayerRequestEnterVehicleEvent")]
pub struct PlayerRequestEnterVehicleEvent {
    player_id: i32,
    vehicle_id: i32,
}

#[pymethods]
impl PlayerRequestEnterVehicleEvent {
    #[new]
    pub fn new(player_id: i32, vehicle_id: i32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerRequestEnterVehicleEvent")).add_subclass(
            Self {
                player_id,
                vehicle_id,
            },
        )
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }

    #[getter]
    pub fn get_vehicle(&self) -> VehiclePy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_vehicle(self.vehicle_id)
            .unwrap()
    }
}

impl From<player::PlayerRequestEnterVehicleEvent> for PlayerRequestEnterVehicleEvent {
    fn from(value: player::PlayerRequestEnterVehicleEvent) -> Self {
        Self {
            player_id: value.player_id,
            vehicle_id: value.vehicle_id,
        }
    }
}

impl PyBaseEvent for PlayerRequestEnterVehicleEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(
            py,
            PlayerRequestEnterVehicleEvent::new(self.player_id, self.vehicle_id),
        )?;
        Ok(value.into())
    }
}

#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "PlayerEnterVehicleEvent")]
pub struct PlayerEnterVehicleEvent {
    player_id: i32,
    vehicle_id: i32,
}

#[pymethods]
impl PlayerEnterVehicleEvent {
    #[new]
    pub fn new(player_id: i32, vehicle_id: i32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerEnterVehicleEvent")).add_subclass(Self {
            player_id,
            vehicle_id,
        })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }

    #[getter]
    pub fn get_vehicle(&self) -> VehiclePy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_vehicle(self.vehicle_id)
            .unwrap()
    }
}

impl From<player::PlayerEnterVehicleEvent> for PlayerEnterVehicleEvent {
    fn from(value: player::PlayerEnterVehicleEvent) -> Self {
        Self {
            player_id: value.player_id,
            vehicle_id: value.vehicle_id,
        }
    }
}

impl PyBaseEvent for PlayerEnterVehicleEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(
            py,
            PlayerEnterVehicleEvent::new(self.player_id, self.vehicle_id),
        )?;
        Ok(value.into())
    }
}

#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "PlayerExitVehicleEvent")]
pub struct PlayerExitVehicleEvent {
    player_id: i32,
    vehicle_id: i32,
}

#[pymethods]
impl PlayerExitVehicleEvent {
    #[new]
    pub fn new(player_id: i32, vehicle_id: i32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerExitVehicleEvent")).add_subclass(Self {
            player_id,
            vehicle_id,
        })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }

    #[getter]
    pub fn get_vehicle(&self) -> VehiclePy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_vehicle(self.vehicle_id)
            .unwrap()
    }
}

impl From<player::PlayerExitVehicleEvent> for PlayerExitVehicleEvent {
    fn from(value: player::PlayerExitVehicleEvent) -> Self {
        Self {
            player_id: value.player_id,
            vehicle_id: value.vehicle_id,
        }
    }
}

impl PyBaseEvent for PlayerExitVehicleEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(
            py,
            PlayerExitVehicleEvent::new(self.player_id, self.vehicle_id),
        )?;
        Ok(value.into())
    }
}

// Player Name Change Event
#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone)]
#[pyo3(name = "PlayerNameChangeEvent")]
pub struct PlayerNameChangeEvent {
    player_id: i32,
    old_name: String,
    new_name: String,
}

#[pymethods]
impl PlayerNameChangeEvent {
    #[new]
    pub fn new(player_id: i32, old_name: String, new_name: String) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerNameChangeEvent")).add_subclass(Self {
            player_id,
            old_name,
            new_name,
        })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }

    #[getter]
    pub fn get_old_name(&self) -> String {
        self.old_name.clone()
    }

    #[getter]
    pub fn get_new_name(&self) -> String {
        self.new_name.clone()
    }
}

impl From<player::PlayerNameChangeEvent> for PlayerNameChangeEvent {
    fn from(value: player::PlayerNameChangeEvent) -> Self {
        Self {
            player_id: value.player_id,
            old_name: value.old_name,
            new_name: value.new_name,
        }
    }
}

impl PyBaseEvent for PlayerNameChangeEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(
            py,
            PlayerNameChangeEvent::new(
                self.player_id,
                self.old_name.clone(),
                self.new_name.clone(),
            ),
        )?;
        Ok(value.into())
    }
}

// Player State Change Event
#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "PlayerStateChangeEvent")]
pub struct PlayerStateChangeEvent {
    player_id: i32,
    old_state: i32,
    new_state: i32,
}

#[pymethods]
impl PlayerStateChangeEvent {
    #[new]
    pub fn new(player_id: i32, old_state: i32, new_state: i32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerStateChangeEvent")).add_subclass(Self {
            player_id,
            old_state,
            new_state,
        })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }

    #[getter]
    pub fn get_old_state(&self) -> i32 {
        self.old_state
    }

    #[getter]
    pub fn get_new_state(&self) -> i32 {
        self.new_state
    }
}

impl From<player::PlayerStateChangeEvent> for PlayerStateChangeEvent {
    fn from(value: player::PlayerStateChangeEvent) -> Self {
        Self {
            player_id: value.player_id,
            old_state: value.old_state,
            new_state: value.new_state,
        }
    }
}

impl PyBaseEvent for PlayerStateChangeEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(
            py,
            PlayerStateChangeEvent::new(self.player_id, self.old_state, self.new_state),
        )?;
        Ok(value.into())
    }
}

// Player Action Change Event
#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "PlayerActionChangeEvent")]
pub struct PlayerActionChangeEvent {
    player_id: i32,
    old_action: i32,
    new_action: i32,
}

#[pymethods]
impl PlayerActionChangeEvent {
    #[new]
    pub fn new(player_id: i32, old_action: i32, new_action: i32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerActionChangeEvent")).add_subclass(Self {
            player_id,
            old_action,
            new_action,
        })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }

    #[getter]
    pub fn get_old_action(&self) -> i32 {
        self.old_action
    }

    #[getter]
    pub fn get_new_action(&self) -> i32 {
        self.new_action
    }
}

impl From<player::PlayerActionChangeEvent> for PlayerActionChangeEvent {
    fn from(value: player::PlayerActionChangeEvent) -> Self {
        Self {
            player_id: value.player_id,
            old_action: value.old_action,
            new_action: value.new_action,
        }
    }
}

impl PyBaseEvent for PlayerActionChangeEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(
            py,
            PlayerActionChangeEvent::new(self.player_id, self.old_action, self.new_action),
        )?;
        Ok(value.into())
    }
}

// Player On Fire Change Event
#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "PlayerOnFireChangeEvent")]
pub struct PlayerOnFireChangeEvent {
    player_id: i32,
    is_on_fire: bool,
}

#[pymethods]
impl PlayerOnFireChangeEvent {
    #[new]
    pub fn new(player_id: i32, is_on_fire: bool) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerOnFireChangeEvent")).add_subclass(Self {
            player_id,
            is_on_fire,
        })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }

    #[getter]
    pub fn is_on_fire(&self) -> bool {
        self.is_on_fire
    }
}

impl From<player::PlayerOnFireChangeEvent> for PlayerOnFireChangeEvent {
    fn from(value: player::PlayerOnFireChangeEvent) -> Self {
        Self {
            player_id: value.player_id,
            is_on_fire: value.is_on_fire,
        }
    }
}

impl PyBaseEvent for PlayerOnFireChangeEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(
            py,
            PlayerOnFireChangeEvent::new(self.player_id, self.is_on_fire),
        )?;
        Ok(value.into())
    }
}

// Player Crouch Change Event
#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "PlayerCrouchChangeEvent")]
pub struct PlayerCrouchChangeEvent {
    player_id: i32,
    is_crouching: bool,
}

#[pymethods]
impl PlayerCrouchChangeEvent {
    #[new]
    pub fn new(player_id: i32, is_crouching: bool) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerCrouchChangeEvent")).add_subclass(Self {
            player_id,
            is_crouching,
        })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }

    #[getter]
    pub fn is_crouching(&self) -> bool {
        self.is_crouching
    }
}

impl From<player::PlayerCrouchChangeEvent> for PlayerCrouchChangeEvent {
    fn from(value: player::PlayerCrouchChangeEvent) -> Self {
        Self {
            player_id: value.player_id,
            is_crouching: value.is_crouching,
        }
    }
}

impl PyBaseEvent for PlayerCrouchChangeEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(
            py,
            PlayerCrouchChangeEvent::new(self.player_id, self.is_crouching),
        )?;
        Ok(value.into())
    }
}

// Player Game Keys Change Event
#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "PlayerGameKeysChangeEvent")]
pub struct PlayerGameKeysChangeEvent {
    player_id: i32,
    old_keys: u32,
    new_keys: u32,
}

#[pymethods]
impl PlayerGameKeysChangeEvent {
    #[new]
    pub fn new(player_id: i32, old_keys: u32, new_keys: u32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerGameKeysChangeEvent")).add_subclass(Self {
            player_id,
            old_keys,
            new_keys,
        })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }

    #[getter]
    pub fn get_old_keys(&self) -> u32 {
        self.old_keys
    }

    #[getter]
    pub fn get_new_keys(&self) -> u32 {
        self.new_keys
    }
}

impl From<player::PlayerGameKeysChangeEvent> for PlayerGameKeysChangeEvent {
    fn from(value: player::PlayerGameKeysChangeEvent) -> Self {
        Self {
            player_id: value.player_id,
            old_keys: value.old_keys,
            new_keys: value.new_keys,
        }
    }
}

impl PyBaseEvent for PlayerGameKeysChangeEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(
            py,
            PlayerGameKeysChangeEvent::new(self.player_id, self.old_keys, self.new_keys),
        )?;
        Ok(value.into())
    }
}

// Player Begin Typing Event
#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "PlayerBeginTypingEvent")]
pub struct PlayerBeginTypingEvent {
    player_id: i32,
}

#[pymethods]
impl PlayerBeginTypingEvent {
    #[new]
    pub fn new(player_id: i32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerBeginTypingEvent"))
            .add_subclass(Self { player_id })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }
}

impl From<i32> for PlayerBeginTypingEvent {
    fn from(value: i32) -> Self {
        Self { player_id: value }
    }
}

impl PyBaseEvent for PlayerBeginTypingEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(py, PlayerBeginTypingEvent::new(self.player_id))?;
        Ok(value.into())
    }
}

// Player End Typing Event
#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "PlayerEndTypingEvent")]
pub struct PlayerEndTypingEvent {
    player_id: i32,
}

#[pymethods]
impl PlayerEndTypingEvent {
    #[new]
    pub fn new(player_id: i32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerEndTypingEvent"))
            .add_subclass(Self { player_id })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }
}

impl From<i32> for PlayerEndTypingEvent {
    fn from(value: i32) -> Self {
        Self { player_id: value }
    }
}

impl PyBaseEvent for PlayerEndTypingEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(py, PlayerEndTypingEvent::new(self.player_id))?;
        Ok(value.into())
    }
}

// Player Away Change Event
#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "PlayerAwayChangeEvent")]
pub struct PlayerAwayChangeEvent {
    player_id: i32,
    is_away: bool,
}

#[pymethods]
impl PlayerAwayChangeEvent {
    #[new]
    pub fn new(player_id: i32, is_away: bool) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerAwayChangeEvent"))
            .add_subclass(Self { player_id, is_away })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }

    #[getter]
    pub fn is_away(&self) -> bool {
        self.is_away
    }
}

impl From<player::PlayerAwayChangeEvent> for PlayerAwayChangeEvent {
    fn from(value: player::PlayerAwayChangeEvent) -> Self {
        Self {
            player_id: value.player_id,
            is_away: value.is_away,
        }
    }
}

impl PyBaseEvent for PlayerAwayChangeEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(py, PlayerAwayChangeEvent::new(self.player_id, self.is_away))?;
        Ok(value.into())
    }
}

// Player Message Event
#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone)]
#[pyo3(name = "PlayerMessageEvent")]
pub struct PlayerMessageEvent {
    player_id: i32,
    message: String,
}

#[pymethods]
impl PlayerMessageEvent {
    #[new]
    pub fn new(player_id: i32, message: String) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerMessageEvent"))
            .add_subclass(Self { player_id, message })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }

    #[getter]
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}

impl From<player::PlayerMessageEvent> for PlayerMessageEvent {
    fn from(value: player::PlayerMessageEvent) -> Self {
        Self {
            player_id: value.player_id,
            message: value.message,
        }
    }
}

impl PyBaseEvent for PlayerMessageEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(
            py,
            PlayerMessageEvent::new(self.player_id, self.message.clone()),
        )?;
        Ok(value.into())
    }
}

// Player Command Event
#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone)]
#[pyo3(name = "PlayerCommandEvent")]
pub struct PlayerCommandEvent {
    player_id: i32,
    command: String,
    text: String,
}

#[pymethods]
impl PlayerCommandEvent {
    #[new]
    pub fn new(player_id: i32, command: String, text: String) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerCommandEvent")).add_subclass(Self {
            player_id,
            command,
            text,
        })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }

    #[getter]
    pub fn get_command(&self) -> String {
        self.command.clone()
    }
}

impl From<player::PlayerCommandEvent> for PlayerCommandEvent {
    fn from(value: player::PlayerCommandEvent) -> Self {
        Self {
            player_id: value.player_id,
            command: value.command,
            text: value.text,
        }
    }
}

impl PyBaseEvent for PlayerCommandEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(
            py,
            PlayerCommandEvent::new(self.player_id, self.command.clone(), self.text.clone()),
        )?;
        Ok(value.into())
    }
}

// Player Private Message Event
#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone)]
#[pyo3(name = "PlayerPrivateMessageEvent")]
pub struct PlayerPrivateMessageEvent {
    player_id: i32,
    target_id: i32,
    message: String,
}

#[pymethods]
impl PlayerPrivateMessageEvent {
    #[new]
    pub fn new(player_id: i32, target_id: i32, message: String) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerPrivateMessageEvent")).add_subclass(Self {
            player_id,
            target_id,
            message,
        })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }

    #[getter]
    pub fn get_target(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.target_id)
            .unwrap()
    }

    #[getter]
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}

impl From<player::PlayerPrivateMessageEvent> for PlayerPrivateMessageEvent {
    fn from(value: player::PlayerPrivateMessageEvent) -> Self {
        Self {
            player_id: value.player_id,
            target_id: value.target_id,
            message: value.message,
        }
    }
}

impl PyBaseEvent for PlayerPrivateMessageEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(
            py,
            PlayerPrivateMessageEvent::new(self.player_id, self.target_id, self.message.clone()),
        )?;
        Ok(value.into())
    }
}

// Player Key Bind Down Event
#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "PlayerKeyBindDownEvent")]
pub struct PlayerKeyBindDownEvent {
    player_id: i32,
    bind_id: i32,
}

#[pymethods]
impl PlayerKeyBindDownEvent {
    #[new]
    pub fn new(player_id: i32, bind_id: i32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerKeyBindDownEvent"))
            .add_subclass(Self { player_id, bind_id })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }

    #[getter]
    pub fn get_bind_id(&self) -> i32 {
        self.bind_id
    }
}

impl From<player::PlayerKeyBindDownEvent> for PlayerKeyBindDownEvent {
    fn from(value: player::PlayerKeyBindDownEvent) -> Self {
        Self {
            player_id: value.player_id,
            bind_id: value.bind_id,
        }
    }
}

impl PyBaseEvent for PlayerKeyBindDownEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(
            py,
            PlayerKeyBindDownEvent::new(self.player_id, self.bind_id),
        )?;
        Ok(value.into())
    }
}

// Player Key Bind Up Event
#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "PlayerKeyBindUpEvent")]
pub struct PlayerKeyBindUpEvent {
    player_id: i32,
    bind_id: i32,
}

#[pymethods]
impl PlayerKeyBindUpEvent {
    #[new]
    pub fn new(player_id: i32, bind_id: i32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerKeyBindUpEvent"))
            .add_subclass(Self { player_id, bind_id })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }

    #[getter]
    pub fn get_bind_id(&self) -> i32 {
        self.bind_id
    }
}

impl From<player::PlayerKeyBindUpEvent> for PlayerKeyBindUpEvent {
    fn from(value: player::PlayerKeyBindUpEvent) -> Self {
        Self {
            player_id: value.player_id,
            bind_id: value.bind_id,
        }
    }
}

impl PyBaseEvent for PlayerKeyBindUpEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(py, PlayerKeyBindUpEvent::new(self.player_id, self.bind_id))?;
        Ok(value.into())
    }
}

// Player Spectate Event
#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "PlayerSpectateEvent")]
pub struct PlayerSpectateEvent {
    player_id: i32,
    target_id: i32,
}

#[pymethods]
impl PlayerSpectateEvent {
    #[new]
    pub fn new(player_id: i32, target_id: i32) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerSpectateEvent")).add_subclass(Self {
            player_id,
            target_id,
        })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }

    #[getter]
    pub fn get_target(&self) -> Option<PlayerPy> {
        let pool = ENTITY_POOL.lock().expect("Failed to lock entity pool");
        pool.get_player(self.target_id).copied()
    }
}

impl From<player::PlayerSpectateEvent> for PlayerSpectateEvent {
    fn from(value: player::PlayerSpectateEvent) -> Self {
        Self {
            player_id: value.player_id,
            target_id: value.target_id,
        }
    }
}

impl PyBaseEvent for PlayerSpectateEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(py, PlayerSpectateEvent::new(self.player_id, self.target_id))?;
        Ok(value.into())
    }
}

// Player Crash Report Event
#[pyclass(extends=PlayerEvent)]
#[derive(Debug, Clone)]
#[pyo3(name = "PlayerCrashReportEvent")]
pub struct PlayerCrashReportEvent {
    player_id: i32,
    report: String,
}

#[pymethods]
impl PlayerCrashReportEvent {
    #[new]
    pub fn new(player_id: i32, report: String) -> PyClassInitializer<Self> {
        PyClassInitializer::from(PlayerEvent::new("PlayerCrashReportEvent"))
            .add_subclass(Self { player_id, report })
    }

    #[getter]
    pub fn get_player(&self) -> PlayerPy {
        *ENTITY_POOL
            .lock()
            .expect("Failed to lock entity pool")
            .get_player(self.player_id)
            .unwrap()
    }

    #[getter]
    pub fn get_report(&self) -> String {
        self.report.clone()
    }
}

impl From<player::PlayerCrashReportEvent> for PlayerCrashReportEvent {
    fn from(value: player::PlayerCrashReportEvent) -> Self {
        Self {
            player_id: value.player_id,
            report: value.report,
        }
    }
}

impl PyBaseEvent for PlayerCrashReportEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = Py::new(
            py,
            PlayerCrashReportEvent::new(self.player_id, self.report.clone()),
        )?;
        Ok(value.into())
    }
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PlayerEvent>()?;
    m.add_class::<IncomingConnectionEvent>()?;
    m.add_class::<PlayerConnectEvent>()?;
    m.add_class::<PlayerDisconnectEvent>()?;
    m.add_class::<ClientScriptDataEvent>()?;
    m.add_class::<PlayerRequestClassEvent>()?;
    m.add_class::<PlayerRequestSpawnEvent>()?;
    m.add_class::<PlayerSpawnEvent>()?;
    m.add_class::<PlayerDeathEvent>()?;

    m.add_class::<PlayerHealthEvent>()?;
    m.add_class::<PlayerArmourEvent>()?;
    m.add_class::<PlayerAmmoEvent>()?;
    m.add_class::<PlayerWeaponEvent>()?;

    m.add_class::<PlayerEnterVehicleEvent>()?;
    m.add_class::<PlayerExitVehicleEvent>()?;
    m.add_class::<PlayerRequestEnterVehicleEvent>()?;

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

    Ok(())
}
