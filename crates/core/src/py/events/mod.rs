use std::collections::HashMap;

use pyo3::{
    Bound, Py, PyAny, PyResult, Python, pyclass, pymethods,
    types::{PyModule, PyModuleMethods, PyTraceback},
};

use crate::{
    functions::{
        checkpoint::CheckPointPy, object::ObjectPy, pickup::PickupPy, player::PlayerPy,
        vehicle::VehiclePy,
    },
    py::{fix_module_name, types::VectorPy},
};

pub mod abc;
pub mod checkpoint;
pub mod custom;
pub mod object;
pub mod pickup;
pub mod player;
pub mod server;
pub mod vehicle;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum VcmpEventType {
    // Server
    ServerInitialise,
    ServerShutdown,
    ServerFrame,
    ServerPerformanceReport,

    // Server Extra
    ServerReloaded,

    // Player
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

    // Player Extra
    PlayerHealthChange,
    PlayerArmourChange,
    PlayerWeaponChange,
    PlayerAmmoChange,
    PlayerMove,

    // Pickup
    PickupPickAttempt,
    PickupPicked,
    PickupRespawn,

    // Checkpoint
    CheckpointEntered,
    CheckpointExited,

    // Object
    ObjectShot,
    ObjectTouched,

    // Vehicle
    VehicleExplode,
    VehicleRespawn,
    VehicleUpdate,

    // Vehicle Extra
    VehicleMove,
    VehicleHealthChange,

    // Custom
    Custom,
    Traceback,
}

#[derive(Debug, Clone)]
pub enum VcmpEvent {
    // Server
    ServerInitialise(server::ServerInitialiseEvent),
    ServerShutdown(server::ServerShutdownEvent),
    ServerFrame(server::ServerFrameEvent),
    ServerPerformanceReport(server::ServerPerformanceReportEvent),

    // Server Extra
    ServerReloaded(server::ServerReloadedEvent),

    // Player
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

    // Player Extra
    PlayerHealthChange(player::PlayerHealthChangeEvent),
    PlayerArmourChange(player::PlayerArmourChangeEvent),
    PlayerWeaponChange(player::PlayerWeaponChangeEvent),
    PlayerAmmoChange(player::PlayerAmmoChangeEvent),
    PlayerMove(player::PlayerMoveEvent),

    // pickup
    PickupPickAttempt(pickup::PickupPickAttemptEvent),
    PickupPicked(pickup::PickupPickedEvent),
    PickupRespawn(pickup::PickupRespawnEvent),

    // checkpoint
    CheckpointEntered(checkpoint::CheckpointEnteredEvent),
    CheckpointExited(checkpoint::CheckpointExitedEvent),

    // object
    ObjectShot(object::ObjectShotEvent),
    ObjectTouched(object::ObjectTouchedEvent),

    // vehicle
    VehicleExplode(vehicle::VehicleExplodeEvent),
    VehicleRespawn(vehicle::VehicleRespawnEvent),
    VehicleUpdate(vehicle::VehicleUpdateEvent),

    // Vehicle Extra
    VehicleMove(vehicle::VehicleMoveEvent),
    VehicleHealthChange(vehicle::VehicleHealthChangeEvent),

    // Custom
    Custom(custom::CustomEvent),
    Traceback(custom::PyTracebackEvent),
}

impl From<VcmpEvent> for VcmpEventType {
    fn from(event_type: VcmpEvent) -> Self {
        match event_type {
            // Server
            VcmpEvent::ServerInitialise(_) => Self::ServerInitialise,
            VcmpEvent::ServerShutdown(_) => Self::ServerShutdown,
            VcmpEvent::ServerFrame(_) => Self::ServerFrame,
            VcmpEvent::ServerPerformanceReport(_) => Self::ServerPerformanceReport,

            // Server Extra
            VcmpEvent::ServerReloaded(_) => Self::ServerReloaded,

            // Player
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

            // Player extra
            VcmpEvent::PlayerHealthChange(_) => Self::PlayerHealthChange,
            VcmpEvent::PlayerArmourChange(_) => Self::PlayerArmourChange,
            VcmpEvent::PlayerWeaponChange(_) => Self::PlayerWeaponChange,
            VcmpEvent::PlayerAmmoChange(_) => Self::PlayerAmmoChange,
            VcmpEvent::PlayerMove(_) => Self::PlayerMove,

            // Pickup
            VcmpEvent::PickupPickAttempt(_) => Self::PickupPickAttempt,
            VcmpEvent::PickupPicked(_) => Self::PickupPicked,
            VcmpEvent::PickupRespawn(_) => Self::PickupRespawn,

            // Checkpoint
            VcmpEvent::CheckpointEntered(_) => Self::CheckpointEntered,
            VcmpEvent::CheckpointExited(_) => Self::CheckpointExited,

            // Object
            VcmpEvent::ObjectShot(_) => Self::ObjectShot,
            VcmpEvent::ObjectTouched(_) => Self::ObjectTouched,

            // Vehicle
            VcmpEvent::VehicleExplode(_) => Self::VehicleExplode,
            VcmpEvent::VehicleRespawn(_) => Self::VehicleRespawn,
            VcmpEvent::VehicleUpdate(_) => Self::VehicleUpdate,

            // Vehicle Extra
            VcmpEvent::VehicleMove(_) => Self::VehicleMove,
            VcmpEvent::VehicleHealthChange(_) => Self::VehicleHealthChange,

            // Custom
            VcmpEvent::Custom(_) => Self::Custom,
            VcmpEvent::Traceback(_) => Self::Traceback,
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
    pub fn with_kwargs(&mut self, kwargs: HashMap<String, Py<PyAny>>) -> Self {
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

    #[staticmethod]
    fn server_frame(elapsed_time: f32) -> Self {
        Self::new(VcmpEvent::ServerFrame(server::ServerFrameEvent::new(
            elapsed_time,
        )))
    }

    #[staticmethod]
    #[pyo3(signature = (descriptions, times, entry_count = None))]
    fn server_performance_report(
        descriptions: Vec<String>,
        times: Vec<u64>,
        entry_count: Option<usize>,
    ) -> Self {
        let count = entry_count.unwrap_or(std::cmp::max(descriptions.len(), times.len()));
        // fill descriptions and times with empty strings and 0s respectively
        let mut descriptions = descriptions;
        let mut times = times;
        descriptions.resize(count, "".to_string());
        times.resize(count, 0);

        Self::new(VcmpEvent::ServerPerformanceReport(
            server::ServerPerformanceReportEvent::new(descriptions, times, count),
        ))
    }

    #[staticmethod]
    #[pyo3(signature = (elapsed_time))]
    fn server_reloaded(elapsed_time: f64) -> Self {
        Self::new(VcmpEvent::ServerReloaded(server::ServerReloadedEvent::new(
            elapsed_time,
        )))
    }

    #[staticmethod]
    fn checkpoint_entered(checkpoint: CheckPointPy, player: PlayerPy) -> Self {
        Self::new(VcmpEvent::CheckpointEntered(
            checkpoint::CheckpointEnteredEvent::new(checkpoint, player),
        ))
    }

    #[staticmethod]
    fn checkpoint_exited(checkpoint: CheckPointPy, player: PlayerPy) -> Self {
        Self::new(VcmpEvent::CheckpointExited(
            checkpoint::CheckpointExitedEvent::new(checkpoint, player),
        ))
    }

    #[staticmethod]
    fn object_shot(object: ObjectPy, player: PlayerPy, weapon_id: i32) -> Self {
        Self::new(VcmpEvent::ObjectShot(object::ObjectShotEvent::new(
            object, player, weapon_id,
        )))
    }

    #[staticmethod]
    fn object_touched(object: ObjectPy, player: PlayerPy) -> Self {
        Self::new(VcmpEvent::ObjectTouched(object::ObjectTouchedEvent::new(
            object, player,
        )))
    }

    #[staticmethod]
    fn pickup_pick_attempt(pickup: PickupPy, player: PlayerPy) -> Self {
        Self::new(VcmpEvent::PickupPickAttempt(
            pickup::PickupPickAttemptEvent::new(pickup, player),
        ))
    }

    #[staticmethod]
    fn pickup_picked(pickup: PickupPy, player: PlayerPy) -> Self {
        Self::new(VcmpEvent::PickupPicked(pickup::PickupPickedEvent::new(
            pickup, player,
        )))
    }

    #[staticmethod]
    fn pickup_respawn(pickup: PickupPy) -> Self {
        Self::new(VcmpEvent::PickupRespawn(pickup::PickupRespawnEvent::new(
            pickup,
        )))
    }

    #[staticmethod]
    fn incoming_connection(ip: String, player_name: String, password: String) -> Self {
        Self::new(VcmpEvent::IncomingConnection(
            player::IncomingConnectionEvent::new(ip, player_name, password),
        ))
    }

    #[staticmethod]
    fn client_script_data(player: PlayerPy, data: Vec<u8>) -> Self {
        Self::new(VcmpEvent::ClientScriptData(
            player::ClientScriptDataEvent::new(player, data),
        ))
    }

    #[staticmethod]
    fn player_connect(player: PlayerPy) -> Self {
        Self::new(VcmpEvent::PlayerConnect(player::PlayerConnectEvent::new(
            player,
        )))
    }

    #[staticmethod]
    fn player_disconnect(player: PlayerPy, reason: i32) -> Self {
        Self::new(VcmpEvent::PlayerDisconnect(
            player::PlayerDisconnectEvent::new(player, reason),
        ))
    }

    #[staticmethod]
    fn player_request_class(player: PlayerPy, class_id: i32) -> Self {
        Self::new(VcmpEvent::PlayerRequestClass(
            player::PlayerRequestClassEvent::new(player, class_id),
        ))
    }

    #[staticmethod]
    fn player_spawn(player: PlayerPy) -> Self {
        Self::new(VcmpEvent::PlayerSpawn(player::PlayerSpawnEvent::new(
            player,
        )))
    }

    #[staticmethod]
    fn player_request_spawn(player: PlayerPy) -> Self {
        Self::new(VcmpEvent::PlayerRequestSpawn(
            player::PlayerRequestSpawnEvent::new(player),
        ))
    }

    #[staticmethod]
    fn player_death(player: PlayerPy, killer: Option<PlayerPy>, reason: i32, body: i32) -> Self {
        Self::new(VcmpEvent::PlayerDeath(player::PlayerDeathEvent::new(
            player, killer, reason, body,
        )))
    }

    #[staticmethod]
    fn player_update(player: PlayerPy, update: i32) -> Self {
        Self::new(VcmpEvent::PlayerUpdate(player::PlayerUpdateEvent::new(
            player, update,
        )))
    }

    #[staticmethod]
    fn player_request_enter_vehicle(player: PlayerPy, vehicle: VehiclePy, slot_index: i32) -> Self {
        Self::new(VcmpEvent::PlayerRequestEnterVehicle(
            player::PlayerRequestEnterVehicleEvent::new(player, vehicle, slot_index),
        ))
    }

    #[staticmethod]
    fn player_enter_vehicle(player: PlayerPy, vehicle: VehiclePy, slot_index: i32) -> Self {
        Self::new(VcmpEvent::PlayerEnterVehicle(
            player::PlayerEnterVehicleEvent::new(player, vehicle, slot_index),
        ))
    }

    #[staticmethod]
    fn player_exit_vehicle(player: PlayerPy, vehicle: VehiclePy) -> Self {
        Self::new(VcmpEvent::PlayerExitVehicle(
            player::PlayerExitVehicleEvent::new(player, vehicle),
        ))
    }

    #[staticmethod]
    fn player_name_change(player: PlayerPy, old_name: String, new_name: String) -> Self {
        Self::new(VcmpEvent::PlayerNameChange(
            player::PlayerNameChangeEvent::new(player, old_name, new_name),
        ))
    }

    #[staticmethod]
    fn player_state_change(player: PlayerPy, old_state: i32, new_state: i32) -> Self {
        Self::new(VcmpEvent::PlayerStateChange(
            player::PlayerStateChangeEvent::new(player, old_state, new_state),
        ))
    }

    #[staticmethod]
    fn player_action_change(player: PlayerPy, old_action: i32, new_action: i32) -> Self {
        Self::new(VcmpEvent::PlayerActionChange(
            player::PlayerActionChangeEvent::new(player, old_action, new_action),
        ))
    }

    #[staticmethod]
    fn player_on_fire_change(player: PlayerPy, is_on_fire: bool) -> Self {
        Self::new(VcmpEvent::PlayerOnFireChange(
            player::PlayerOnFireChangeEvent::new(player, is_on_fire),
        ))
    }

    #[staticmethod]
    fn player_crouch_change(player: PlayerPy, is_crouching: bool) -> Self {
        Self::new(VcmpEvent::PlayerCrouchChange(
            player::PlayerCrouchChangeEvent::new(player, is_crouching),
        ))
    }

    #[staticmethod]
    fn player_game_keys_change(player: PlayerPy, old_keys: u32, new_keys: u32) -> Self {
        Self::new(VcmpEvent::PlayerGameKeysChange(
            player::PlayerGameKeysChangeEvent::new(player, old_keys, new_keys),
        ))
    }

    #[staticmethod]
    fn player_begin_typing(player: PlayerPy) -> Self {
        Self::new(VcmpEvent::PlayerBeginTyping(
            player::PlayerBeginTypingEvent::new(player),
        ))
    }

    #[staticmethod]
    fn player_end_typing(player: PlayerPy) -> Self {
        Self::new(VcmpEvent::PlayerEndTyping(
            player::PlayerEndTypingEvent::new(player),
        ))
    }

    #[staticmethod]
    fn player_away_change(player: PlayerPy, is_away: bool) -> Self {
        Self::new(VcmpEvent::PlayerAwayChange(
            player::PlayerAwayChangeEvent::new(player, is_away),
        ))
    }

    #[staticmethod]
    fn player_message(player: PlayerPy, message: String) -> Self {
        Self::new(VcmpEvent::PlayerMessage(player::PlayerMessageEvent::new(
            player, message,
        )))
    }

    #[staticmethod]
    fn player_command(player: PlayerPy, command: String, text: String) -> Self {
        Self::new(VcmpEvent::PlayerCommand(player::PlayerCommandEvent::new(
            player, command, text,
        )))
    }

    #[staticmethod]
    fn player_private_message(player: PlayerPy, target: PlayerPy, message: String) -> Self {
        Self::new(VcmpEvent::PlayerPrivateMessage(
            player::PlayerPrivateMessageEvent::new(player, target, message),
        ))
    }

    #[staticmethod]
    fn player_key_bind_down(player: PlayerPy, bind_id: i32) -> Self {
        Self::new(VcmpEvent::PlayerKeyBindDown(
            player::PlayerKeyBindDownEvent::new(player, bind_id),
        ))
    }

    #[staticmethod]
    fn player_key_bind_up(player: PlayerPy, bind_id: i32) -> Self {
        Self::new(VcmpEvent::PlayerKeyBindUp(
            player::PlayerKeyBindUpEvent::new(player, bind_id),
        ))
    }

    #[staticmethod]
    fn player_spectate(player: PlayerPy, target: Option<PlayerPy>) -> Self {
        Self::new(VcmpEvent::PlayerSpectate(player::PlayerSpectateEvent::new(
            player, target,
        )))
    }

    #[staticmethod]
    fn player_crash_report(player: PlayerPy, report: String) -> Self {
        Self::new(VcmpEvent::PlayerCrashReport(
            player::PlayerCrashReportEvent::new(player, report),
        ))
    }

    #[staticmethod]
    fn player_module_list(player: PlayerPy, modules: String) -> Self {
        Self::new(VcmpEvent::PlayerModuleList(
            player::PlayerModuleListEvent::new(player, modules),
        ))
    }

    // Extra events
    #[staticmethod]
    fn player_health_change(player: PlayerPy, old_health: f32, new_health: f32) -> Self {
        Self::new(VcmpEvent::PlayerHealthChange(
            player::PlayerHealthChangeEvent::new(player, old_health, new_health),
        ))
    }

    #[staticmethod]
    fn player_armour_change(player: PlayerPy, old_armour: f32, new_armour: f32) -> Self {
        Self::new(VcmpEvent::PlayerArmourChange(
            player::PlayerArmourChangeEvent::new(player, old_armour, new_armour),
        ))
    }

    #[staticmethod]
    fn player_weapon_change(player: PlayerPy, old_weapon: i32, new_weapon: i32) -> Self {
        Self::new(VcmpEvent::PlayerWeaponChange(
            player::PlayerWeaponChangeEvent::new(player, old_weapon, new_weapon),
        ))
    }

    #[staticmethod]
    fn player_ammo_change(player: PlayerPy, old_ammo: i32, new_ammo: i32) -> Self {
        Self::new(VcmpEvent::PlayerAmmoChange(
            player::PlayerAmmoChangeEvent::new(player, old_ammo, new_ammo),
        ))
    }

    #[staticmethod]
    fn player_move(player: PlayerPy, old_position: VectorPy, new_position: VectorPy) -> Self {
        Self::new(VcmpEvent::PlayerMove(player::PlayerMoveEvent::new(
            player,
            old_position,
            new_position,
        )))
    }

    #[staticmethod]
    fn vehicle_update(vehicle: VehiclePy, update_type: i32) -> Self {
        Self::new(VcmpEvent::VehicleUpdate(vehicle::VehicleUpdateEvent::new(
            vehicle,
            update_type,
        )))
    }

    #[staticmethod]
    fn vehicle_explode(vehicle: VehiclePy) -> Self {
        Self::new(VcmpEvent::VehicleExplode(
            vehicle::VehicleExplodeEvent::new(vehicle),
        ))
    }

    #[staticmethod]
    fn vehicle_respawn(vehicle: VehiclePy) -> Self {
        Self::new(VcmpEvent::VehicleRespawn(
            vehicle::VehicleRespawnEvent::new(vehicle),
        ))
    }

    // Vehicle Extra
    #[staticmethod]
    fn vehicle_move(vehicle: VehiclePy, old_position: VectorPy, new_position: VectorPy) -> Self {
        Self::new(VcmpEvent::VehicleMove(vehicle::VehicleMoveEvent::new(
            vehicle,
            old_position,
            new_position,
        )))
    }

    #[staticmethod]
    fn vehicle_health_change(vehicle: VehiclePy, old_health: f32, new_health: f32) -> Self {
        Self::new(VcmpEvent::VehicleHealthChange(
            vehicle::VehicleHealthChangeEvent::new(vehicle, old_health, new_health),
        ))
    }

    // Custom
    #[staticmethod]
    #[pyo3(signature = (**kwargs))]
    fn custom(kwargs: Option<HashMap<String, Py<PyAny>>>) -> Self {
        Self::new(VcmpEvent::Custom(custom::CustomEvent::default()))
            .with_kwargs(kwargs.unwrap_or_default())
    }

    #[staticmethod]
    #[pyo3(signature = (traceback))]
    fn traceback(traceback: Py<PyTraceback>) -> Self {
        Self::new(VcmpEvent::Traceback(custom::PyTracebackEvent::new(
            traceback,
        )))
    }
}

pub fn module_define(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let abc_module = PyModule::new(py, "abc")?;
    abc::module_define(py, &abc_module)?;
    fix_module_name(py, &abc_module, "events.abc");
    m.add_submodule(&abc_module)?;

    let checkpoint_module = PyModule::new(py, "checkpoint")?;
    checkpoint::module_define(py, &checkpoint_module)?;
    fix_module_name(py, &checkpoint_module, "events.checkpoint");
    m.add_submodule(&checkpoint_module)?;

    let object_module = PyModule::new(py, "object")?;
    object::module_define(py, &object_module)?;
    fix_module_name(py, &object_module, "events.object");
    m.add_submodule(&object_module)?;

    let pickup_module = PyModule::new(py, "pickup")?;
    pickup::module_define(py, &pickup_module)?;
    fix_module_name(py, &pickup_module, "events.pickup");
    m.add_submodule(&pickup_module)?;

    let player_module = PyModule::new(py, "player")?;
    player::module_define(py, &player_module)?;
    fix_module_name(py, &player_module, "events.player");
    m.add_submodule(&player_module)?;

    let server_module = PyModule::new(py, "server")?;
    server::module_define(py, &server_module)?;
    fix_module_name(py, &server_module, "events.server");
    m.add_submodule(&server_module)?;

    let vehicle_module = PyModule::new(py, "vehicle")?;
    vehicle::module_define(py, &vehicle_module)?;
    fix_module_name(py, &vehicle_module, "events.vehicle");
    m.add_submodule(&vehicle_module)?;

    m.add_class::<PyVcmpEvent>()?;

    // Custom
    let custom_module = PyModule::new(py, "custom")?;
    custom::module_define(py, &custom_module)?;
    fix_module_name(py, &custom_module, "events.custom");
    m.add_submodule(&custom_module)?;

    Ok(())
}
