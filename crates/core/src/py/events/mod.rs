use std::collections::HashMap;

use pyo3::{
    Bound, Py, PyAny, PyResult, Python, pyclass, pymethods,
    types::{PyModule, PyModuleMethods},
};

use crate::py::fix_module_name;

pub mod abc;
pub mod player;
pub mod server;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum VcmpEventType {
    ServerInitialise,
    ServerShutdown,
    ServerFrame,
    ServerPerformanceReport,

    IncomingConnection,
    ClientScriptData,
    PlayerConnect,
    PlayerDisconnect,
    PlayerRequestClass,
    PlayerSpawn,
    PlayerRequestSpawn,
    PlayerDeath,
    PlayerUpdate,
    PlayerRequestEnterVehicle,
    PlayerEnterVehicle,
    PlayerExitVehicle,
    PlayerNameChange,
    PlayerStateChange,
    PlayerActionChange,
    PlayerOnFireChange,
    PlayerCrouchChange,
    PlayerGameKeysChange,
    PlayerBeginTyping,
    PlayerEndTyping,
    PlayerAwayChange,
    PlayerMessage,
    PlayerCommand,
    PlayerPrivateMessage,
    PlayerKeyBindDown,
    PlayerKeyBindUp,
    PlayerSpectate,
    PlayerCrashReport,
    PlayerModuleList,
}

#[derive(Debug, Clone)]
pub enum VcmpEvent {
    ServerInitialise(server::ServerInitialiseEvent),
    ServerShutdown(server::ServerShutdownEvent),
    ServerFrame(server::ServerFrameEvent),
    ServerPerformanceReport(server::ServerPerformanceReportEvent),

    IncomingConnection(player::IncomingConnectionEvent),
    ClientScriptData(player::ClientScriptDataEvent),
    PlayerConnect(player::PlayerConnectEvent),
    PlayerDisconnect(player::PlayerDisconnectEvent),
    PlayerRequestClass(player::PlayerRequestClassEvent),
    PlayerSpawn(player::PlayerSpawnEvent),
    PlayerRequestSpawn(player::PlayerRequestSpawnEvent),
    PlayerDeath(player::PlayerDeathEvent),
    PlayerUpdate(player::PlayerUpdateEvent),
    PlayerRequestEnterVehicle(player::PlayerRequestEnterVehicleEvent),
    PlayerEnterVehicle(player::PlayerEnterVehicleEvent),
    PlayerExitVehicle(player::PlayerExitVehicleEvent),
    PlayerNameChange(player::PlayerNameChangeEvent),
    PlayerStateChange(player::PlayerStateChangeEvent),
    PlayerActionChange(player::PlayerActionChangeEvent),
    PlayerOnFireChange(player::PlayerOnFireChangeEvent),
    PlayerCrouchChange(player::PlayerCrouchChangeEvent),
    PlayerGameKeysChange(player::PlayerGameKeysChangeEvent),
    PlayerBeginTyping(player::PlayerBeginTypingEvent),
    PlayerEndTyping(player::PlayerEndTypingEvent),
    PlayerAwayChange(player::PlayerAwayChangeEvent),
    PlayerMessage(player::PlayerMessageEvent),
    PlayerCommand(player::PlayerCommandEvent),
    PlayerPrivateMessage(player::PlayerPrivateMessageEvent),
    PlayerKeyBindDown(player::PlayerKeyBindDownEvent),
    PlayerKeyBindUp(player::PlayerKeyBindUpEvent),
    PlayerSpectate(player::PlayerSpectateEvent),
    PlayerCrashReport(player::PlayerCrashReportEvent),
    PlayerModuleList(player::PlayerModuleListEvent),
}

impl From<VcmpEvent> for VcmpEventType {
    fn from(event_type: VcmpEvent) -> Self {
        match event_type {
            VcmpEvent::ServerInitialise(_) => Self::ServerInitialise,
            VcmpEvent::ServerShutdown(_) => Self::ServerShutdown,
            VcmpEvent::ServerFrame(_) => Self::ServerFrame,
            VcmpEvent::ServerPerformanceReport(_) => Self::ServerPerformanceReport,

            VcmpEvent::IncomingConnection(_) => Self::IncomingConnection,
            VcmpEvent::ClientScriptData(_) => Self::ClientScriptData,
            VcmpEvent::PlayerConnect(_) => Self::PlayerConnect,
            VcmpEvent::PlayerDisconnect(_) => Self::PlayerDisconnect,
            VcmpEvent::PlayerRequestClass(_) => Self::PlayerRequestClass,
            VcmpEvent::PlayerSpawn(_) => Self::PlayerSpawn,
            VcmpEvent::PlayerRequestSpawn(_) => Self::PlayerRequestSpawn,
            VcmpEvent::PlayerDeath(_) => Self::PlayerDeath,
            VcmpEvent::PlayerUpdate(_) => Self::PlayerUpdate,
            VcmpEvent::PlayerRequestEnterVehicle(_) => Self::PlayerRequestEnterVehicle,
            VcmpEvent::PlayerEnterVehicle(_) => Self::PlayerEnterVehicle,
            VcmpEvent::PlayerExitVehicle(_) => Self::PlayerExitVehicle,
            VcmpEvent::PlayerNameChange(_) => Self::PlayerNameChange,
            VcmpEvent::PlayerStateChange(_) => Self::PlayerStateChange,
            VcmpEvent::PlayerActionChange(_) => Self::PlayerActionChange,
            VcmpEvent::PlayerOnFireChange(_) => Self::PlayerOnFireChange,
            VcmpEvent::PlayerCrouchChange(_) => Self::PlayerCrouchChange,
            VcmpEvent::PlayerGameKeysChange(_) => Self::PlayerGameKeysChange,
            VcmpEvent::PlayerBeginTyping(_) => Self::PlayerBeginTyping,
            VcmpEvent::PlayerEndTyping(_) => Self::PlayerEndTyping,
            VcmpEvent::PlayerAwayChange(_) => Self::PlayerAwayChange,
            VcmpEvent::PlayerMessage(_) => Self::PlayerMessage,
            VcmpEvent::PlayerCommand(_) => Self::PlayerCommand,
            VcmpEvent::PlayerPrivateMessage(_) => Self::PlayerPrivateMessage,
            VcmpEvent::PlayerKeyBindDown(_) => Self::PlayerKeyBindDown,
            VcmpEvent::PlayerKeyBindUp(_) => Self::PlayerKeyBindUp,
            VcmpEvent::PlayerSpectate(_) => Self::PlayerSpectate,
            VcmpEvent::PlayerCrashReport(_) => Self::PlayerCrashReport,
            VcmpEvent::PlayerModuleList(_) => Self::PlayerModuleList,
        }
    }
}

#[derive(Debug, Clone)]
#[pyclass]
#[pyo3(name = "EventBuilder")]
pub struct PyVcmpEvent {
    pub event_type: VcmpEvent,
    pub kwargs: HashMap<String, Py<PyAny>>,
}

impl PyVcmpEvent {
    pub fn new(event_type: VcmpEvent) -> Self {
        Self {
            event_type,
            kwargs: HashMap::new(),
        }
    }
}

impl From<VcmpEvent> for PyVcmpEvent {
    fn from(event_type: VcmpEvent) -> Self {
        Self::new(event_type)
    }
}

#[pymethods]
impl PyVcmpEvent {
    fn with_kwargs(&mut self, kwargs: HashMap<String, Py<PyAny>>) -> Self {
        self.kwargs = kwargs;
        self.clone()
    }

    #[staticmethod]
    fn server_initialise() -> Self {
        Self::new(VcmpEvent::ServerInitialise(
            server::ServerInitialiseEvent::default(),
        ))
    }

    #[staticmethod]
    fn server_shutdown() -> Self {
        Self::new(VcmpEvent::ServerShutdown(
            server::ServerShutdownEvent::default(),
        ))
    }
}

pub fn module_define(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let abc_module = PyModule::new(py, "abc")?;
    abc::module_define(py, &abc_module)?;
    fix_module_name(py, &abc_module, "events.abc");
    m.add_submodule(&abc_module)?;

    let server_module = PyModule::new(py, "server")?;
    server::module_define(py, &server_module)?;
    fix_module_name(py, &server_module, "events.server");
    m.add_submodule(&server_module)?;

    let player_module = PyModule::new(py, "player")?;
    player::module_define(py, &player_module)?;
    fix_module_name(py, &player_module, "events.player");
    m.add_submodule(&player_module)?;

    m.add_class::<PyVcmpEvent>()?;

    Ok(())
}
