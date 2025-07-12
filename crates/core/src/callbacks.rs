use std::os::raw::c_char;

use crate::py::GLOBAL_VAR;
use crate::py::callbacks::PY_CALLBACK_MANAGER;
use crate::py::events::VcmpEvent;
use crate::py::events::player::*;
use crate::py::events::server::{
    ServerFrameEvent, ServerInitialiseEvent, ServerPerformanceReportEvent, ServerShutdownEvent,
};
use vcmp_bindings::events::{player, server};
use vcmp_bindings::{options::VcmpEntityPool, raw::PluginCallbacks};

use crate::{cfg::CONFIG, pool::ENTITY_POOL, py::load_script_as_module};
use tracing::{Level, event};

// use crate::py::callbacks::CALLBACK;

#[unsafe(no_mangle)]
pub extern "C" fn on_server_init() -> u8 {
    if !CONFIG.get().unwrap().preloader {
        load_script_as_module();
    }

    let _ =
        PY_CALLBACK_MANAGER.handle(VcmpEvent::ServerInitialise(ServerInitialiseEvent {}), false);

    1
}

#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn on_server_frame(elapsed_time: f32) {
    // first check need reload

    let mut var = GLOBAL_VAR.lock().unwrap();
    if var.need_reload {
        var.need_reload = false;
        load_script_as_module();
    }

    // println!("[Rust] Server frame callback time: {}", elapsed_time);
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::ServerFrame(ServerFrameEvent::from(server::ServerFrameEvent::from(
            elapsed_time,
        ))),
        false,
    );
}

///
/// # Safety
/// it's for ffi
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_server_performance_report(
    entry_count: usize,
    descriptions: *mut *const c_char,
    times: *mut u64,
) {
    let bindings_event =
        server::ServerPerformanceReportEvent::from((entry_count, descriptions, times));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::ServerPerformanceReport(ServerPerformanceReportEvent::from(bindings_event)),
        false,
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn on_server_shutdown() {
    let _ = PY_CALLBACK_MANAGER.handle(VcmpEvent::ServerShutdown(ServerShutdownEvent {}), false);
}

/// # Safety
///
/// ffi!
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_entity_pool_change(c_entity_type: i32, entity_id: i32, is_deleted: u8) {
    let entity_type = VcmpEntityPool::from(c_entity_type);
    let deleted = is_deleted != 0;
    event!(Level::TRACE, "Entity pool change");
    event!(Level::TRACE, "entity type: {entity_type:?}");
    event!(Level::TRACE, "entity id: {entity_id}");
    event!(Level::TRACE, "deleted: {deleted}");

    let mut pool = ENTITY_POOL.lock().expect("pool is poisoned");

    if deleted {
        pool.remove(entity_type, entity_id);
    } else {
        pool.insert(entity_type, entity_id);
    }
}

/// # Safety
/// FFI callback for incoming player connection
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_incoming_connection(
    player_name: *mut c_char,
    name_buffer_size: usize,
    user_password: *const c_char,
    ip_address: *const c_char,
) -> u8 {
    let binding_event = player::IncomingConnectionEvent::from((
        player_name,
        name_buffer_size,
        user_password,
        ip_address,
    ));

    PY_CALLBACK_MANAGER.handle(
        VcmpEvent::IncomingConnection(IncomingConnectionEvent::from(binding_event)),
        true,
    ) as u8
}

/// # Safety
/// FFI callback for player connection
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_connect(player_id: i32) {
    // Update entity pool
    unsafe {
        let _ = on_entity_pool_change(VcmpEntityPool::Player as i32, player_id, 0);
    }

    let binding_event = player::PlayerConnectEvent::from(player_id);
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerConnect(PlayerConnectEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for player disconnection
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_disconnect(player_id: i32, reason: i32) {
    let binding_event = player::PlayerDisconnectEvent::from((player_id, reason));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerDisconnect(PlayerDisconnectEvent::from(binding_event)),
        false,
    );

    // Update entity pool
    unsafe {
        let _ = on_entity_pool_change(VcmpEntityPool::Player as i32, player_id, 1);
    }
}

/// # Safety
/// FFI callback for client script data
/// - `data` must point to a valid buffer of at least `size` bytes
/// - Buffer must remain valid during function execution
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_client_script_data(client_id: i32, data: *const u8, size: usize) {
    let binding_event = player::ClientScriptDataEvent::from((client_id, data, size));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::ClientScriptData(ClientScriptDataEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for player class request
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_request_class(player_id: i32, class_id: i32) -> u8 {
    let binding_event = player::PlayerRequestClassEvent::from((player_id, class_id));
    PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerRequestClass(PlayerRequestClassEvent::from(binding_event)),
        true,
    ) as u8
}

/// # Safety
/// FFI callback for player spawn request
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_request_spawn(player_id: i32) -> u8 {
    let binding_event = player::PlayerRequestSpawnEvent::from(player_id);
    PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerRequestSpawn(PlayerRequestSpawnEvent::from(binding_event)),
        true,
    ) as u8
}

/// # Safety
/// FFI callback for player entering vehicle
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_enter_vehicle(player_id: i32, vehicle_id: i32, seat_id: i32) {
    let binding_event = player::PlayerEnterVehicleEvent::from((player_id, vehicle_id, seat_id));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerEnterVehicle(PlayerEnterVehicleEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for player exiting vehicle
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_exit_vehicle(player_id: i32, vehicle_id: i32) {
    let binding_event = player::PlayerExitVehicleEvent::from((player_id, vehicle_id));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerExitVehicle(PlayerExitVehicleEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for player vehicle entry request
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_request_enter_vehicle(
    player_id: i32,
    vehicle_id: i32,
    seat_id: i32,
) -> u8 {
    let binding_event =
        player::PlayerRequestEnterVehicleEvent::from((player_id, vehicle_id, seat_id));
    PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerRequestEnterVehicle(PlayerRequestEnterVehicleEvent::from(binding_event)),
        true,
    ) as u8
}

/// # Safety
/// FFI callback for player spawn
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_spawn(player_id: i32) {
    let binding_event = player::PlayerSpawnEvent::from(player_id);
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerSpawn(PlayerSpawnEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for player death
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_death(
    player_id: i32,
    killer_id: i32,
    reason: i32,
    body_part: i32,
) {
    let binding_event = player::PlayerDeathEvent::from((player_id, killer_id, reason, body_part));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerDeath(PlayerDeathEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for player name change
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_name_change(
    player_id: i32,
    old_name: *const c_char,
    new_name: *const c_char,
) {

    let binding_event = player::PlayerNameChangeEvent::from((player_id, old_name, new_name));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerNameChange(PlayerNameChangeEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for player state change
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_state_change(player_id: i32, old_state: i32, new_state: i32) {
    let binding_event = player::PlayerStateChangeEvent::from((player_id, old_state, new_state));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerStateChange(PlayerStateChangeEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for player action change
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_action_change(player_id: i32, old_action: i32, new_action: i32) {
    let binding_event = player::PlayerActionChangeEvent::from((player_id, old_action, new_action));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerActionChange(PlayerActionChangeEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for player on fire state change
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_on_fire_change(player_id: i32, is_on_fire: u8) {
    let binding_event = player::PlayerOnFireChangeEvent::from((player_id, is_on_fire));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerOnFireChange(PlayerOnFireChangeEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for player crouch state change
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_crouch_change(player_id: i32, is_crouching: u8) {
    let binding_event = player::PlayerCrouchChangeEvent::from((player_id, is_crouching));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerCrouchChange(PlayerCrouchChangeEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for player game keys change
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_game_keys_change(player_id: i32, old_keys: u32, new_keys: u32) {
    let binding_event = player::PlayerGameKeysChangeEvent::from((player_id, old_keys, new_keys));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerGameKeysChange(PlayerGameKeysChangeEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for player beginning to type
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_begin_typing(player_id: i32) {
    let binding_event = player::PlayerBeginTypingEvent::from(player_id);
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerBeginTyping(PlayerBeginTypingEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for player ending typing
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_end_typing(player_id: i32) {
    let binding_event = player::PlayerEndTypingEvent::from(player_id);
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerEndTyping(PlayerEndTypingEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for player away state change
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_away_change(player_id: i32, is_away: u8) {
    let binding_event = player::PlayerAwayChangeEvent::from((player_id, is_away));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerAwayChange(PlayerAwayChangeEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for player message
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_message(player_id: i32, message: *const c_char) -> u8 {
    let binding_event = player::PlayerMessageEvent::from((player_id, message));
    PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerMessage(PlayerMessageEvent::from(binding_event)),
        true,
    ) as u8
}

/// # Safety
/// FFI callback for player command
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_command(player_id: i32, command: *const c_char) -> u8 {
    let binding_event = player::PlayerCommandEvent::from((player_id, command));
    PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerCommand(PlayerCommandEvent::from(binding_event)),
        true,
    ) as u8
}

/// # Safety
/// FFI callback for player private message
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_private_message(
    player_id: i32,
    target_id: i32,
    message: *const c_char,
) -> u8 {
    let binding_event = player::PlayerPrivateMessageEvent::from((player_id, target_id, message));
    PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerPrivateMessage(PlayerPrivateMessageEvent::from(binding_event)),
        true,
    ) as u8
}

/// # Safety
/// FFI callback for player key bind down
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_key_bind_down(player_id: i32, bind_id: i32) {
    let binding_event = player::PlayerKeyBindDownEvent::from((player_id, bind_id));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerKeyBindDown(PlayerKeyBindDownEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for player key bind up
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_key_bind_up(player_id: i32, bind_id: i32) {
    let binding_event = player::PlayerKeyBindUpEvent::from((player_id, bind_id));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerKeyBindUp(PlayerKeyBindUpEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for player spectate action
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_spectate(player_id: i32, target_id: i32) {
    let binding_event = player::PlayerSpectateEvent::from((player_id, target_id));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerSpectate(PlayerSpectateEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for player crash report
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_crash_report(player_id: i32, report: *const c_char) {
    let binding_event = player::PlayerCrashReportEvent::from((player_id, report));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerCrashReport(PlayerCrashReportEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for player module list (补充实现)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_module_list(player_id: i32, modules: *const c_char) {
    let binding_event = player::PlayerModuleListEvent::from((player_id, modules));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerModuleList(PlayerModuleListEvent::from(binding_event)),
        false,
    );
}

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn on_player_update(player_id: i32, _state: i32) {
//     let mut player = *ENTITY_POOL.lock().unwrap().get_player(player_id).unwrap();
//     // first health
//     {
//         let current_health = player.get_health();
//         let last_health = player.last_health;
//         if current_health != last_health {
//             let event = PlayerHealthEvent::from((player_id, last_health, current_health));
//             let health_res = CALLBACK.call_func(event, None, true);
//             if !health_res {
//                 player.set_health(event.get_health());
//             } else {
//                 player.last_health = current_health;
//             }
//         }
//     }
//     // then armour
//     {
//         let current_armour = player.get_armour();
//         let last_armour = player.last_armour;
//         if current_armour != last_armour {
//             let event = PlayerArmourEvent::from((player_id, last_armour, current_armour));
//             let armour_res = CALLBACK.call_func(event, None, true);
//             if !armour_res {
//                 player.set_armour(event.get_armour());
//             } else {
//                 player.last_armour = current_armour;
//             }
//         }
//     }
//     // then weapon
//     {
//         let current_weapon = player.get_weapon();
//         let last_weapon = player.last_weapon;
//         if current_weapon != last_weapon {
//             let event = PlayerWeaponEvent::from((player_id, last_weapon, current_weapon));
//             let weapon_res = CALLBACK.call_func(event, None, true);
//             if !weapon_res {
//                 player.give_weapon(event.get_weapon(), 0);
//             } else {
//                 player.last_weapon = current_weapon;
//             }
//         }
//     }
//     // then ammo, not block it
//     {
//         let current_ammo = player.get_weapon_ammo();
//         let last_ammo = player.last_ammo;
//         if current_ammo != last_ammo {
//             let _ = CALLBACK.call_func(
//                 PlayerAmmoEvent::from((player_id, last_ammo, current_ammo)),
//                 None,
//                 true,
//             );
//             player.last_ammo = current_ammo;
//         }
//     }
//     // then move
//     {
//         let current_pos = player.get_position().into();
//         let last_pos = player.last_position;
//         if current_pos != last_pos {
//             let event = PlayerMoveEvent::from((
//                 player_id,
//                 VectorPy::from(last_pos),
//                 VectorPy::from(current_pos),
//             ));
//             let move_res = CALLBACK.call_func(event, None, true);
//             if !move_res {
//                 player.set_position(event.position.get_entity_pos());
//             } else {
//                 player.last_position = current_pos;
//             }
//         }
//     }
// }

pub fn init_callbacks(callbacks: &mut PluginCallbacks) {
    callbacks.OnServerInitialise = Some(on_server_init);
    callbacks.OnServerFrame = Some(on_server_frame);
    callbacks.OnServerPerformanceReport = Some(on_server_performance_report);
    callbacks.OnServerShutdown = Some(on_server_shutdown);

    callbacks.OnIncomingConnection = Some(on_incoming_connection);
    callbacks.OnPlayerConnect = Some(on_player_connect);
    callbacks.OnPlayerDisconnect = Some(on_player_disconnect);
    callbacks.OnClientScriptData = Some(on_client_script_data);

    callbacks.OnPlayerRequestClass = Some(on_player_request_class);
    callbacks.OnPlayerRequestSpawn = Some(on_player_request_spawn);
    callbacks.OnPlayerEnterVehicle = Some(on_player_enter_vehicle);
    callbacks.OnPlayerExitVehicle = Some(on_player_exit_vehicle);
    callbacks.OnPlayerRequestEnterVehicle = Some(on_player_request_enter_vehicle);
    callbacks.OnPlayerSpawn = Some(on_player_spawn);
    callbacks.OnPlayerDeath = Some(on_player_death);

    callbacks.OnPlayerNameChange = Some(on_player_name_change);
    callbacks.OnPlayerStateChange = Some(on_player_state_change);
    callbacks.OnPlayerActionChange = Some(on_player_action_change);
    callbacks.OnPlayerOnFireChange = Some(on_player_on_fire_change);
    callbacks.OnPlayerCrouchChange = Some(on_player_crouch_change);
    callbacks.OnPlayerGameKeysChange = Some(on_player_game_keys_change);
    callbacks.OnPlayerBeginTyping = Some(on_player_begin_typing);
    callbacks.OnPlayerEndTyping = Some(on_player_end_typing);
    callbacks.OnPlayerAwayChange = Some(on_player_away_change);
    callbacks.OnPlayerMessage = Some(on_player_message);
    callbacks.OnPlayerCommand = Some(on_player_command);
    callbacks.OnPlayerPrivateMessage = Some(on_player_private_message);
    callbacks.OnPlayerKeyBindDown = Some(on_player_key_bind_down);
    callbacks.OnPlayerKeyBindUp = Some(on_player_key_bind_up);
    callbacks.OnPlayerSpectate = Some(on_player_spectate);
    callbacks.OnPlayerCrashReport = Some(on_player_crash_report);
    //callbacks.OnPlayerUpdate = Some(on_player_update);

    callbacks.OnEntityPoolChange = Some(on_entity_pool_change);
}
