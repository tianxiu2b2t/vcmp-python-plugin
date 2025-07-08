use crate::{PlayerId, options::VcmpEntityPool};
use std::ffi::c_char;

pub mod checkpoint;
pub mod object;
pub mod pickup;
pub mod player;
pub mod server;
pub mod vehicle;

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
