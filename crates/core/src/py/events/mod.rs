// https://bitbucket.org/stormeus/0.4-squirrel/src/master/CallbackHandler.cpp

use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{Bound, PyResult, Python};
use vcmp_bindings::events::{
    EntityPoolChangeEvent, EntityStreamingChangeEvent, PluginCommandEvent, VcmpEvent,
    checkpoint as vcmp_bindings_checkpoint, object as vcmp_bindings_object,
    pickup as vcmp_bindings_pickup, player as vcmp_bindings_player, server as vcmp_bindings_server,
    vehicle as vcmp_bindings_vehicle,
};

use crate::py::fix_module_name;

use super::types::VcmpEntityPoolPy;

pub mod player;
pub mod server;

#[pyclass(name = "VcmpEvent")]
#[derive(Clone, Debug)]
pub struct PyVcmpEvent {
    pub event_enum: VcmpEvent,
}

impl PyVcmpEvent {
    pub fn new(event: VcmpEvent) -> Self {
        Self { event_enum: event }
    }
}

#[pymethods]
impl PyVcmpEvent {
    #[staticmethod]
    pub fn plugin_command(identifer: u32, message: String) -> Self {
        Self::new(VcmpEvent::PluginCommand(PluginCommandEvent::new(
            identifer, message,
        )))
    }

    #[staticmethod]
    pub fn entity_streaming(
        player_id: i32,
        entity_id: i32,
        entity_type: VcmpEntityPoolPy,
        deleted: bool,
    ) -> Self {
        Self::new(VcmpEvent::EntityStreaming(EntityStreamingChangeEvent::new(
            player_id,
            entity_id,
            entity_type.into(),
            deleted,
        )))
    }

    #[staticmethod]
    pub fn entity_pool(entity_type: VcmpEntityPoolPy, entity_id: i32, deleted: bool) -> Self {
        Self::new(VcmpEvent::EntityPool(EntityPoolChangeEvent::new(
            entity_type.into(),
            entity_id,
            deleted,
        )))
    }

    // checkpoint 相关事件
    #[staticmethod]
    pub fn checkpoint_entered(checkpoint_id: i32, player_id: i32) -> Self {
        Self::new(VcmpEvent::CheckpointEntered(
            vcmp_bindings_checkpoint::CheckpointEnteredEvent {
                checkpoint_id,
                player_id,
            },
        ))
    }

    #[staticmethod]
    pub fn checkpoint_exited(checkpoint_id: i32, player_id: i32) -> Self {
        Self::new(VcmpEvent::CheckpointExited(
            vcmp_bindings_checkpoint::CheckpointExitedEvent {
                checkpoint_id,
                player_id,
            },
        ))
    }

    // object 相关事件
    #[staticmethod]
    pub fn object_shot(object_id: i32, player_id: i32, weapon_id: i32) -> Self {
        Self::new(VcmpEvent::ObjectShot(
            vcmp_bindings_object::ObjectShotEvent {
                object_id,
                player_id,
                weapon_id,
            },
        ))
    }

    #[staticmethod]
    pub fn object_touched(object_id: i32, player_id: i32) -> Self {
        Self::new(VcmpEvent::ObjectTouched(
            vcmp_bindings_object::ObjectTouchedEvent {
                object_id,
                player_id,
            },
        ))
    }

    // pickup 相关事件
    #[staticmethod]
    pub fn pickup_picked(pickup_id: i32, player_id: i32) -> Self {
        Self::new(VcmpEvent::PickupPicked(
            vcmp_bindings_pickup::PickupPickedEvent {
                pickup_id,
                player_id,
            },
        ))
    }

    #[staticmethod]
    pub fn pickup_pick_attempt(pickup_id: i32, player_id: i32, is_allowed: bool) -> Self {
        Self::new(VcmpEvent::PickupPickAttempt(
            vcmp_bindings_pickup::PickupPickAttemptEvent {
                pickup_id,
                player_id,
                is_allowed,
            },
        ))
    }

    #[staticmethod]
    pub fn pickup_respawn(pickup_id: i32) -> Self {
        Self::new(VcmpEvent::PickupRespawn(
            vcmp_bindings_pickup::PickupRespawnEvent { pickup_id },
        ))
    }

    // player 相关事件
    #[staticmethod]
    pub fn incoming_connection(player_name: String, password: String, ip: String) -> Self {
        Self::new(VcmpEvent::IncomingConnection(
            vcmp_bindings_player::IncomingConnectionEvent::new(player_name, password, ip),
        ))
    }

    #[staticmethod]
    pub fn client_script_data(player_id: i32, data: Vec<u8>) -> Self {
        Self::new(VcmpEvent::ClientScriptData(
            vcmp_bindings_player::ClientScriptDataEvent { player_id, data },
        ))
    }

    #[staticmethod]
    pub fn player_connect(player_id: i32) -> Self {
        Self::new(VcmpEvent::PlayerConnect(
            vcmp_bindings_player::PlayerConnectEvent { player_id },
        ))
    }

    #[staticmethod]
    pub fn player_disconnect(player_id: i32, reason: i32) -> Self {
        Self::new(VcmpEvent::PlayerDisconnect(
            vcmp_bindings_player::PlayerDisconnectEvent { player_id, reason },
        ))
    }

    #[staticmethod]
    pub fn player_request_class(player_id: i32, class_id: i32) -> Self {
        Self::new(VcmpEvent::PlayerRequestClass(
            vcmp_bindings_player::PlayerRequestClassEvent {
                player_id,
                class_id,
            },
        ))
    }

    #[staticmethod]
    pub fn player_spawn(player_id: i32) -> Self {
        Self::new(VcmpEvent::PlayerSpawn(
            vcmp_bindings_player::PlayerSpawnEvent { player_id },
        ))
    }

    #[staticmethod]
    pub fn player_request_spawn(player_id: i32) -> Self {
        Self::new(VcmpEvent::PlayerRequestSpawn(
            vcmp_bindings_player::PlayerRequestSpawnEvent { player_id },
        ))
    }

    #[staticmethod]
    pub fn player_death(player_id: i32, killer_id: i32, reason: i32, body: i32) -> Self {
        Self::new(VcmpEvent::PlayerDeath(
            vcmp_bindings_player::PlayerDeathEvent {
                player_id,
                killer_id,
                reason,
                body,
            },
        ))
    }

    #[staticmethod]
    pub fn player_update(player_id: i32, update: i32) -> Self {
        Self::new(VcmpEvent::PlayerUpdate(
            vcmp_bindings_player::PlayerUpdateEvent { player_id, update },
        ))
    }

    #[staticmethod]
    pub fn player_request_enter_vehicle(player_id: i32, vehicle_id: i32, slot_index: i32) -> Self {
        Self::new(VcmpEvent::PlayerRequestEnterVehicle(
            vcmp_bindings_player::PlayerRequestEnterVehicleEvent {
                player_id,
                vehicle_id,
                slot_index,
            },
        ))
    }

    #[staticmethod]
    pub fn player_enter_vehicle(player_id: i32, vehicle_id: i32, slot_index: i32) -> Self {
        Self::new(VcmpEvent::PlayerEnterVehicle(
            vcmp_bindings_player::PlayerEnterVehicleEvent {
                player_id,
                vehicle_id,
                slot_index,
            },
        ))
    }

    #[staticmethod]
    pub fn player_exit_vehicle(player_id: i32, vehicle_id: i32) -> Self {
        Self::new(VcmpEvent::PlayerExitVehicle(
            vcmp_bindings_player::PlayerExitVehicleEvent {
                player_id,
                vehicle_id,
            },
        ))
    }

    #[staticmethod]
    pub fn player_name_change(player_id: i32, old_name: String, new_name: String) -> Self {
        Self::new(VcmpEvent::PlayerNameChange(
            vcmp_bindings_player::PlayerNameChangeEvent {
                player_id,
                old_name,
                new_name,
            },
        ))
    }

    #[staticmethod]
    pub fn player_state_change(player_id: i32, old_state: i32, new_state: i32) -> Self {
        Self::new(VcmpEvent::PlayerStateChange(
            vcmp_bindings_player::PlayerStateChangeEvent {
                player_id,
                old_state,
                new_state,
            },
        ))
    }

    #[staticmethod]
    pub fn player_action_change(player_id: i32, old_action: i32, new_action: i32) -> Self {
        Self::new(VcmpEvent::PlayerActionChange(
            vcmp_bindings_player::PlayerActionChangeEvent {
                player_id,
                old_action,
                new_action,
            },
        ))
    }

    #[staticmethod]
    pub fn player_on_fire_change(player_id: i32, is_on_fire: bool) -> Self {
        Self::new(VcmpEvent::PlayerOnFireChange(
            vcmp_bindings_player::PlayerOnFireChangeEvent {
                player_id,
                is_on_fire,
            },
        ))
    }

    #[staticmethod]
    pub fn player_crouch_change(player_id: i32, is_crouching: bool) -> Self {
        Self::new(VcmpEvent::PlayerCrouchChange(
            vcmp_bindings_player::PlayerCrouchChangeEvent {
                player_id,
                is_crouching,
            },
        ))
    }

    #[staticmethod]
    pub fn player_game_keys_change(player_id: i32, old_keys: u32, new_keys: u32) -> Self {
        Self::new(VcmpEvent::PlayerGameKeysChange(
            vcmp_bindings_player::PlayerGameKeysChangeEvent {
                player_id,
                old_keys,
                new_keys,
            },
        ))
    }

    #[staticmethod]
    pub fn player_begin_typing(player_id: i32) -> Self {
        Self::new(VcmpEvent::PlayerBeginTyping(
            vcmp_bindings_player::PlayerBeginTypingEvent { player_id },
        ))
    }

    #[staticmethod]
    pub fn player_end_typing(player_id: i32) -> Self {
        Self::new(VcmpEvent::PlayerEndTyping(
            vcmp_bindings_player::PlayerEndTypingEvent { player_id },
        ))
    }

    #[staticmethod]
    pub fn player_away_change(player_id: i32, is_away: bool) -> Self {
        Self::new(VcmpEvent::PlayerAwayChange(
            vcmp_bindings_player::PlayerAwayChangeEvent { player_id, is_away },
        ))
    }

    #[staticmethod]
    pub fn player_message(player_id: i32, message: String) -> Self {
        Self::new(VcmpEvent::PlayerMessage(
            vcmp_bindings_player::PlayerMessageEvent { player_id, message },
        ))
    }

    #[staticmethod]
    pub fn player_command(player_id: i32, command: String) -> Self {
        // 解析command为command和text部分
        let mut parts = command.splitn(2, ' ');
        let cmd = parts.next().unwrap_or("").to_string();
        let text = parts.next().unwrap_or("").to_string();

        Self::new(VcmpEvent::PlayerCommand(
            vcmp_bindings_player::PlayerCommandEvent {
                player_id,
                command: cmd,
                text,
            },
        ))
    }

    #[staticmethod]
    pub fn player_private_message(player_id: i32, target_id: i32, message: String) -> Self {
        Self::new(VcmpEvent::PlayerPrivateMessage(
            vcmp_bindings_player::PlayerPrivateMessageEvent {
                player_id,
                target_id,
                message,
            },
        ))
    }

    #[staticmethod]
    pub fn player_key_bind_down(player_id: i32, bind_id: i32) -> Self {
        Self::new(VcmpEvent::PlayerKeyBindDown(
            vcmp_bindings_player::PlayerKeyBindDownEvent { player_id, bind_id },
        ))
    }

    #[staticmethod]
    pub fn player_key_bind_up(player_id: i32, bind_id: i32) -> Self {
        Self::new(VcmpEvent::PlayerKeyBindUp(
            vcmp_bindings_player::PlayerKeyBindUpEvent { player_id, bind_id },
        ))
    }

    #[staticmethod]
    pub fn player_spectate(player_id: i32, target_id: i32) -> Self {
        Self::new(VcmpEvent::PlayerSpectate(
            vcmp_bindings_player::PlayerSpectateEvent {
                player_id,
                target_id,
            },
        ))
    }

    #[staticmethod]
    pub fn player_crash_report(player_id: i32, report: String) -> Self {
        Self::new(VcmpEvent::PlayerCrashReport(
            vcmp_bindings_player::PlayerCrashReportEvent { player_id, report },
        ))
    }

    #[staticmethod]
    pub fn player_module_list(player_id: i32, modules: String) -> Self {
        Self::new(VcmpEvent::PlayerModuleList(
            vcmp_bindings_player::PlayerModuleListEvent { player_id, modules },
        ))
    }

    // server 相关事件
    #[staticmethod]
    pub fn server_initialise() -> Self {
        Self::new(VcmpEvent::ServerInitialise(
            vcmp_bindings_server::ServerInitialiseEvent,
        ))
    }

    #[staticmethod]
    pub fn server_shutdown() -> Self {
        Self::new(VcmpEvent::ServerShutdown(
            vcmp_bindings_server::ServerShutdownEvent,
        ))
    }

    #[staticmethod]
    pub fn server_frame(elapsed_time: f32) -> Self {
        Self::new(VcmpEvent::ServerFrame(
            vcmp_bindings_server::ServerFrameEvent { elapsed_time },
        ))
    }

    #[staticmethod]
    pub fn server_performance_report(
        entry_count: usize,
        descriptions: Vec<String>,
        times: Vec<u64>,
    ) -> Self {
        Self::new(VcmpEvent::ServerPerformanceReport(
            vcmp_bindings_server::ServerPerformanceReportEvent {
                entry_count,
                descriptions,
                times,
            },
        ))
    }

    // vehicle 相关事件
    #[staticmethod]
    pub fn vehicle_update(vehicle_id: i32, update_type: i32) -> Self {
        Self::new(VcmpEvent::VehicleUpdate(
            vcmp_bindings_vehicle::VehicleUpdateEvent {
                vehicle_id,
                update_type,
            },
        ))
    }

    #[staticmethod]
    pub fn vehicle_explode(vehicle_id: i32) -> Self {
        Self::new(VcmpEvent::VehicleExplode(
            vcmp_bindings_vehicle::VehicleExplodeEvent { vehicle_id },
        ))
    }

    #[staticmethod]
    pub fn vehicle_respawn(vehicle_id: i32) -> Self {
        Self::new(VcmpEvent::VehicleRespawn(
            vcmp_bindings_vehicle::VehicleRespawnEvent { vehicle_id },
        ))
    }
}

#[pyclass(subclass)]
#[pyo3(name = "Event")]
#[derive(Debug, Clone)]
pub struct BaseEvent {
    pub name: String,
}

pub trait PyBaseEvent: std::fmt::Debug + Clone {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>>;
}
impl PyBaseEvent for BaseEvent {
    fn init(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let _ = py;
        todo!()
    }
}

impl BaseEvent {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

pub fn module_define(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let server_module = PyModule::new(py, "server")?;
    server::module_define(py, &server_module)?;
    fix_module_name(py, &server_module, "events.server");
    m.add_submodule(&server_module)?;

    let player_module = PyModule::new(py, "player")?;
    player::module_define(py, &player_module)?;
    fix_module_name(py, &player_module, "events.player");
    m.add_submodule(&player_module)?;

    // abc
    let abc_module = PyModule::new(py, "abc")?;
    {
        let abc = &abc_module;
        abc.add_class::<BaseEvent>()?;
    }
    fix_module_name(py, &abc_module, "events.abc");
    m.add_submodule(&abc_module)?;
    Ok(())
}
