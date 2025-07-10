use crate::{PlayerId, options::VcmpEntityPool};
use std::ffi::c_char;

pub mod checkpoint;
pub mod object;
pub mod pickup;
pub mod player;
pub mod server;
pub mod vehicle;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Copy)]
pub enum VcmpEventType {
    PluginCommand,
    EntityStreaming,
    EntityPool,

    // checkpoint
    CheckpointEntered,
    CheckpointExited,

    // object
    ObjectShot,
    ObjectTouched,

    // pickup
    PickupPicked,
    PickupPickAttempt,
    PickupRespawn,

    // player
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

    // server
    ServerInitialise,
    ServerShutdown,
    ServerFrame,
    ServerPerformanceReport,

    // vehicle
    VehicleUpdate,
    VehicleExplode,
    VehicleRespawn,
}

#[derive(Clone, Debug)]
pub enum VcmpEvent {
    PluginCommand(PluginCommandEvent),
    EntityStreaming(EntityStreamingChangeEvent),
    EntityPool(EntityPoolChangeEvent),

    // checkpoint
    CheckpointEntered(checkpoint::CheckpointEnteredEvent),
    CheckpointExited(checkpoint::CheckpointExitedEvent),

    // object
    ObjectShot(object::ObjectShotEvent),
    ObjectTouched(object::ObjectTouchedEvent),

    // pickup
    PickupPicked(pickup::PickupPickedEvent),
    PickupPickAttempt(pickup::PickupPickAttemptEvent),
    PickupRespawn(pickup::PickupRespawnEvent),

    // player
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

    // server
    ServerInitialise(server::ServerInitialiseEvent),
    ServerShutdown(server::ServerShutdownEvent),
    ServerFrame(server::ServerFrameEvent),
    ServerPerformanceReport(server::ServerPerformanceReportEvent),

    // vehicle
    VehicleUpdate(vehicle::VehicleUpdateEvent),
    VehicleExplode(vehicle::VehicleExplodeEvent),
    VehicleRespawn(vehicle::VehicleRespawnEvent),
}

impl From<VcmpEvent> for VcmpEventType {
    fn from(val: VcmpEvent) -> Self {
        match val {
            VcmpEvent::PluginCommand(_) => VcmpEventType::PluginCommand,
            VcmpEvent::EntityStreaming(_) => VcmpEventType::EntityStreaming,
            VcmpEvent::EntityPool(_) => VcmpEventType::EntityPool,
            VcmpEvent::CheckpointEntered(_) => VcmpEventType::CheckpointEntered,
            VcmpEvent::CheckpointExited(_) => VcmpEventType::CheckpointExited,
            VcmpEvent::ObjectShot(_) => VcmpEventType::ObjectShot,
            VcmpEvent::ObjectTouched(_) => VcmpEventType::ObjectTouched,
            VcmpEvent::PickupPicked(_) => VcmpEventType::PickupPicked,
            VcmpEvent::PickupPickAttempt(_) => VcmpEventType::PickupPickAttempt,
            VcmpEvent::PickupRespawn(_) => VcmpEventType::PickupRespawn,
            VcmpEvent::IncomingConnection(_) => VcmpEventType::IncomingConnection,
            VcmpEvent::ClientScriptData(_) => VcmpEventType::ClientScriptData,
            VcmpEvent::PlayerConnect(_) => VcmpEventType::PlayerConnect,
            VcmpEvent::PlayerDisconnect(_) => VcmpEventType::PlayerDisconnect,
            VcmpEvent::PlayerRequestClass(_) => VcmpEventType::PlayerRequestClass,
            VcmpEvent::PlayerSpawn(_) => VcmpEventType::PlayerSpawn,
            VcmpEvent::PlayerRequestSpawn(_) => VcmpEventType::PlayerRequestSpawn,
            VcmpEvent::PlayerDeath(_) => VcmpEventType::PlayerDeath,
            VcmpEvent::PlayerUpdate(_) => VcmpEventType::PlayerUpdate,
            VcmpEvent::PlayerRequestEnterVehicle(_) => VcmpEventType::PlayerRequestEnterVehicle,
            VcmpEvent::PlayerEnterVehicle(_) => VcmpEventType::PlayerEnterVehicle,
            VcmpEvent::PlayerExitVehicle(_) => VcmpEventType::PlayerExitVehicle,
            VcmpEvent::PlayerNameChange(_) => VcmpEventType::PlayerNameChange,
            VcmpEvent::PlayerStateChange(_) => VcmpEventType::PlayerStateChange,
            VcmpEvent::PlayerActionChange(_) => VcmpEventType::PlayerActionChange,
            VcmpEvent::PlayerOnFireChange(_) => VcmpEventType::PlayerOnFireChange,
            VcmpEvent::PlayerCrouchChange(_) => VcmpEventType::PlayerCrouchChange,
            VcmpEvent::PlayerGameKeysChange(_) => VcmpEventType::PlayerGameKeysChange,
            VcmpEvent::PlayerBeginTyping(_) => VcmpEventType::PlayerBeginTyping,
            VcmpEvent::PlayerEndTyping(_) => VcmpEventType::PlayerEndTyping,
            VcmpEvent::PlayerAwayChange(_) => VcmpEventType::PlayerAwayChange,
            VcmpEvent::PlayerMessage(_) => VcmpEventType::PlayerMessage,
            VcmpEvent::PlayerCommand(_) => VcmpEventType::PlayerCommand,
            VcmpEvent::PlayerPrivateMessage(_) => VcmpEventType::PlayerPrivateMessage,
            VcmpEvent::PlayerKeyBindDown(_) => VcmpEventType::PlayerKeyBindDown,
            VcmpEvent::PlayerKeyBindUp(_) => VcmpEventType::PlayerKeyBindUp,
            VcmpEvent::PlayerSpectate(_) => VcmpEventType::PlayerSpectate,
            VcmpEvent::PlayerCrashReport(_) => VcmpEventType::PlayerCrashReport,
            VcmpEvent::PlayerModuleList(_) => VcmpEventType::PlayerModuleList,
            VcmpEvent::ServerInitialise(_) => VcmpEventType::ServerInitialise,
            VcmpEvent::ServerShutdown(_) => VcmpEventType::ServerShutdown,
            VcmpEvent::ServerFrame(_) => VcmpEventType::ServerFrame,
            VcmpEvent::ServerPerformanceReport(_) => VcmpEventType::ServerPerformanceReport,
            VcmpEvent::VehicleUpdate(_) => VcmpEventType::VehicleUpdate,
            VcmpEvent::VehicleExplode(_) => VcmpEventType::VehicleExplode,
            VcmpEvent::VehicleRespawn(_) => VcmpEventType::VehicleRespawn,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PluginCommandEvent {
    pub identifer: u32,
    pub message: String,
}

impl PluginCommandEvent {
    pub fn new(identifer: u32, message: String) -> Self {
        Self { identifer, message }
    }
}

impl From<(u32, *const c_char)> for PluginCommandEvent {
    fn from(value: (u32, *const c_char)) -> Self {
        Self {
            identifer: value.0,
            message: unsafe {
                std::ffi::CStr::from_ptr(value.1)
                    .to_string_lossy()
                    .to_string()
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntityStreamingChangeEvent {
    pub player_id: PlayerId,
    pub entity_id: i32,
    pub entity_type: VcmpEntityPool,
    pub deleted: bool,
}

impl EntityStreamingChangeEvent {
    pub fn new(
        player_id: PlayerId,
        entity_id: i32,
        entity_type: VcmpEntityPool,
        deleted: bool,
    ) -> Self {
        Self {
            player_id,
            entity_id,
            entity_type,
            deleted,
        }
    }
}

impl From<(i32, i32, i32, u8)> for EntityStreamingChangeEvent {
    fn from(value: (i32, i32, i32, u8)) -> Self {
        Self {
            player_id: value.0,
            entity_id: value.1,
            entity_type: VcmpEntityPool::from(value.2),
            deleted: value.3 != 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntityPoolChangeEvent {
    pub entity_type: VcmpEntityPool,
    pub entity_id: i32,
    pub deleted: bool,
}

impl EntityPoolChangeEvent {
    pub fn new(entity_type: VcmpEntityPool, entity_id: i32, deleted: bool) -> Self {
        Self {
            entity_type,
            entity_id,
            deleted,
        }
    }
}

impl From<(i32, i32, u8)> for EntityPoolChangeEvent {
    fn from(value: (i32, i32, u8)) -> Self {
        Self {
            entity_type: VcmpEntityPool::from(value.0),
            entity_id: value.1,
            deleted: value.2 != 0,
        }
    }
}
