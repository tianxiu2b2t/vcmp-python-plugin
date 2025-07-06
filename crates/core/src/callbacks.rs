use std::os::raw::c_char;

use crate::py::events::{player::*, server::*};
use crate::py::types::VectorPy;
use vcmp_bindings::{events::player, options::VcmpEntityPool, raw::PluginCallbacks};

use crate::logger;
use crate::{cfg::CONFIG, pool::ENTITY_POOL, py::load_script_as_module};

use crate::py::callbacks::CALLBACK;

#[unsafe(no_mangle)]
pub extern "C" fn on_server_init() -> u8 {
    logger::event!(logger::Level::TRACE, "[Rust] Server init callback");

    if !CONFIG.get().unwrap().preloader {
        load_script_as_module();
    }

    let _ = CALLBACK.call_func(ServerInitialiseEvent, None);

    1
}

#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn on_server_frame(elapsed_time: f32) {
    // println!("[Rust] Server frame callback time: {}", elapsed_time);
    let _ = CALLBACK.call_func(ServerFrameEvent::from(elapsed_time), None);
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
    let _ = CALLBACK.call_func(
        ServerPerformanceReportEvent::from((entry_count, descriptions, times)),
        None,
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn on_server_shutdown() {
    let _ = CALLBACK.call_func(ServerShutdownEvent, None);
}

/// # Safety
///
/// ffi!
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_entity_pool_change(c_entity_type: i32, entity_id: i32, is_deleted: u8) {
    let entity_type = VcmpEntityPool::from(c_entity_type);
    let deleted = is_deleted != 0;
    logger::event!(logger::Level::TRACE, "Entity pool change");
    logger::event!(logger::Level::TRACE, "entity type: {entity_type:?}");
    logger::event!(logger::Level::TRACE, "entity id: {entity_id}");
    logger::event!(logger::Level::TRACE, "deleted: {deleted}");

    let mut pool = ENTITY_POOL.lock().expect("pool is poisoned");

    if deleted {
        pool.remove(entity_type, entity_id);
    } else {
        pool.insert(entity_type, entity_id);
    }
}

#[unsafe(no_mangle)]
/// # Safety
pub unsafe extern "C" fn on_incoming_connection(
    player_name: *mut ::std::os::raw::c_char,
    name_buffer_size: usize,
    user_password: *const ::std::os::raw::c_char,
    ip_address: *const ::std::os::raw::c_char,
) -> u8 {
    CALLBACK.call_func(
        IncomingConnectionEvent::from((player_name, name_buffer_size, user_password, ip_address)),
        None,
    ) as u8
}

#[unsafe(no_mangle)]
/// # Safety
pub unsafe extern "C" fn on_player_connect(player_id: i32) {
    // first call on_entity_pool_change, maybe vcmp not call on_entity_pool_change?
    unsafe {
        on_entity_pool_change(VcmpEntityPool::Player as i32, player_id, 0);
    }

    let _ = CALLBACK.call_func(
        PlayerConnectEvent::from(player::PlayerConnectEvent::from(player_id)),
        None,
    );
}

#[unsafe(no_mangle)]
/// # Safety
pub unsafe extern "C" fn on_player_disconnect(player_id: i32, reason: i32) {
    let _ = CALLBACK.call_func(
        PlayerDisconnectEvent::from(player::PlayerDisconnectEvent::from((player_id, reason))),
        None,
    );

    unsafe {
        on_entity_pool_change(VcmpEntityPool::Player as i32, player_id, 1);
    }
}

#[unsafe(no_mangle)]
/// # Safety
///
/// This function handles raw pointers from FFI. The caller must ensure:
/// - `_data` points to a valid buffer of at least `_size` bytes
/// - The buffer is not modified during the execution of this function
pub unsafe extern "C" fn on_client_script_data(client_id: i32, data: *const u8, size: usize) {
    let _ = CALLBACK.call_func(
        ClientScriptDataEvent::from(player::ClientScriptDataEvent::from((client_id, data, size))),
        None,
    );
}

#[unsafe(no_mangle)]
/// # Safety
pub unsafe extern "C" fn on_player_request_class(player_id: i32, class_id: i32) -> u8 {
    CALLBACK.call_func(
        PlayerRequestClassEvent::from(player::PlayerRequestClassEvent::from((player_id, class_id))),
        None,
    ) as u8
}

#[unsafe(no_mangle)]
/// # Safety
pub unsafe extern "C" fn on_player_request_spawn(player_id: i32) -> u8 {
    CALLBACK.call_func(
        PlayerRequestSpawnEvent::from(player::PlayerRequestSpawnEvent::from(player_id)),
        None,
    ) as u8
}

#[unsafe(no_mangle)]
/// # Safety
pub unsafe extern "C" fn on_player_enter_vehicle(player_id: i32, vehicle_id: i32, seat_id: i32) {
    let _ = CALLBACK.call_func(
        PlayerEnterVehicleEvent::from(player::PlayerEnterVehicleEvent::from((
            player_id, vehicle_id, seat_id,
        ))),
        None,
    );
}

#[unsafe(no_mangle)]
/// # Safety
pub unsafe extern "C" fn on_player_exit_vehicle(player_id: i32, vehicle_id: i32) {
    let _ = CALLBACK.call_func(
        PlayerExitVehicleEvent::from(player::PlayerExitVehicleEvent::from((
            player_id, vehicle_id,
        ))),
        None,
    );
}

#[unsafe(no_mangle)]
/// # Safety
pub unsafe extern "C" fn on_player_request_enter_vehicle(
    player_id: i32,
    vehicle_id: i32,
    seat_id: i32,
) -> u8 {
    CALLBACK.call_func(
        PlayerRequestEnterVehicleEvent::from(player::PlayerRequestEnterVehicleEvent::from((
            player_id, vehicle_id, seat_id,
        ))),
        None,
    ) as u8
}

#[unsafe(no_mangle)]
/// # Safety
pub unsafe extern "C" fn on_player_spawn(player_id: i32) {
    let _ = CALLBACK.call_func(
        PlayerSpawnEvent::from(player::PlayerSpawnEvent::from(player_id)),
        None,
    );
}

#[unsafe(no_mangle)]
/// # Safety
pub unsafe extern "C" fn on_player_death(
    player_id: i32,
    killer_id: i32,
    reason: i32,
    body_part: i32,
) {
    let _ = CALLBACK.call_func(
        PlayerDeathEvent::from(player::PlayerDeathEvent::from((
            player_id, killer_id, reason, body_part,
        ))),
        None,
    );
}

#[unsafe(no_mangle)]
/// # Safety
pub unsafe extern "C" fn on_player_name_change(
    player_id: i32,
    old_name: *const ::std::os::raw::c_char,
    new_name: *const ::std::os::raw::c_char,
) {
    let _ = CALLBACK.call_func(
        PlayerNameChangeEvent::from(player::PlayerNameChangeEvent::from((
            player_id, old_name, new_name,
        ))),
        None,
    );
}

#[unsafe(no_mangle)]
/// # Safety
pub unsafe extern "C" fn on_player_state_change(player_id: i32, old_state: i32, new_state: i32) {
    let _ = CALLBACK.call_func(
        PlayerStateChangeEvent::from(player::PlayerStateChangeEvent::from((
            player_id, old_state, new_state,
        ))),
        None,
    );
}

#[unsafe(no_mangle)]
/// # Safety
pub unsafe extern "C" fn on_player_action_change(player_id: i32, old_action: i32, new_action: i32) {
    let _ = CALLBACK.call_func(
        PlayerActionChangeEvent::from(player::PlayerActionChangeEvent::from((
            player_id, old_action, new_action,
        ))),
        None,
    );
}

#[unsafe(no_mangle)]
/// # Safety
/// FFI callback for player on fire state change
pub unsafe extern "C" fn on_player_on_fire_change(player_id: i32, is_on_fire: u8) {
    let _ = CALLBACK.call_func(
        PlayerOnFireChangeEvent::from(player::PlayerOnFireChangeEvent::from((
            player_id, is_on_fire,
        ))),
        None,
    );
}

#[unsafe(no_mangle)]
/// # Safety
/// FFI callback for player crouch state change
pub unsafe extern "C" fn on_player_crouch_change(player_id: i32, is_crouching: u8) {
    let _ = CALLBACK.call_func(
        PlayerCrouchChangeEvent::from(player::PlayerCrouchChangeEvent::from((
            player_id,
            is_crouching,
        ))),
        None,
    );
}

#[unsafe(no_mangle)]
/// # Safety
/// FFI callback for player game keys change
pub unsafe extern "C" fn on_player_game_keys_change(player_id: i32, old_keys: u32, new_keys: u32) {
    let _ = CALLBACK.call_func(
        PlayerGameKeysChangeEvent::from(player::PlayerGameKeysChangeEvent::from((
            player_id, old_keys, new_keys,
        ))),
        None,
    );
}

#[unsafe(no_mangle)]
/// # Safety
/// FFI callback for player begin typing
pub unsafe extern "C" fn on_player_begin_typing(player_id: i32) {
    let _ = CALLBACK.call_func(PlayerBeginTypingEvent::from(player_id), None);
}

#[unsafe(no_mangle)]
/// # Safety
/// FFI callback for player end typing
pub unsafe extern "C" fn on_player_end_typing(player_id: i32) {
    let _ = CALLBACK.call_func(PlayerEndTypingEvent::from(player_id), None);
}

#[unsafe(no_mangle)]
/// # Safety
/// FFI callback for player away state change
pub unsafe extern "C" fn on_player_away_change(player_id: i32, is_away: u8) {
    let _ = CALLBACK.call_func(
        PlayerAwayChangeEvent::from(player::PlayerAwayChangeEvent::from((player_id, is_away))),
        None,
    );
}

#[unsafe(no_mangle)]
/// # Safety
/// FFI callback for player message
pub unsafe extern "C" fn on_player_message(player_id: i32, message: *const c_char) -> u8 {
    CALLBACK.call_func(
        PlayerMessageEvent::from(player::PlayerMessageEvent::from((player_id, message))),
        None,
    ) as u8
}

#[unsafe(no_mangle)]
/// # Safety
/// FFI callback for player command
pub unsafe extern "C" fn on_player_command(player_id: i32, command: *const c_char) -> u8 {
    CALLBACK.call_func(
        PlayerCommandEvent::from(player::PlayerCommandEvent::from((player_id, command))),
        None,
    ) as u8
}

#[unsafe(no_mangle)]
/// # Safety
/// FFI callback for player private message
pub unsafe extern "C" fn on_player_private_message(
    player_id: i32,
    target_id: i32,
    message: *const c_char,
) -> u8 {
    CALLBACK.call_func(
        PlayerPrivateMessageEvent::from(player::PlayerPrivateMessageEvent::from((
            player_id, target_id, message,
        ))),
        None,
    ) as u8
}

#[unsafe(no_mangle)]
/// # Safety
/// FFI callback for player key bind down
pub unsafe extern "C" fn on_player_key_bind_down(player_id: i32, bind_id: i32) {
    let _ = CALLBACK.call_func(
        PlayerKeyBindDownEvent::from(player::PlayerKeyBindDownEvent::from((player_id, bind_id))),
        None,
    );
}

#[unsafe(no_mangle)]
/// # Safety
/// FFI callback for player key bind up
pub unsafe extern "C" fn on_player_key_bind_up(player_id: i32, bind_id: i32) {
    let _ = CALLBACK.call_func(
        PlayerKeyBindUpEvent::from(player::PlayerKeyBindUpEvent::from((player_id, bind_id))),
        None,
    );
}

#[unsafe(no_mangle)]
/// # Safety
/// FFI callback for player spectate
pub unsafe extern "C" fn on_player_spectate(player_id: i32, target_id: i32) {
    let _ = CALLBACK.call_func(
        PlayerSpectateEvent::from(player::PlayerSpectateEvent::from((player_id, target_id))),
        None,
    );
}

#[unsafe(no_mangle)]
/// # Safety
/// FFI callback for player crash report
pub unsafe extern "C" fn on_player_crash_report(player_id: i32, report: *const c_char) {
    let _ = CALLBACK.call_func(
        PlayerCrashReportEvent::from(player::PlayerCrashReportEvent::from((player_id, report))),
        None,
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_update(player_id: i32, _state: i32) {
    let mut player = *ENTITY_POOL.lock().unwrap().get_player(player_id).unwrap();
    // first health
    {
        let current_health = player.get_health();
        let last_health = player.last_health;
        if current_health != last_health {
            let event = PlayerHealthEvent::from((player_id, last_health, current_health));
            let health_res = CALLBACK.call_func(event, None);
            if !health_res {
                player.set_health(event.get_health());
            } else {
                player.last_health = current_health;
            }
        }
    }
    // then armour
    {
        let current_armour = player.get_armour();
        let last_armour = player.last_armour;
        if current_armour != last_armour {
            let event = PlayerArmourEvent::from((player_id, last_armour, current_armour));
            let armour_res = CALLBACK.call_func(event, None);
            if !armour_res {
                player.set_armour(event.get_armour());
            } else {
                player.last_armour = current_armour;
            }
        }
    }
    // then weapon
    {
        let current_weapon = player.get_weapon();
        let last_weapon = player.last_weapon;
        if current_weapon != last_weapon {
            let event = PlayerWeaponEvent::from((player_id, last_weapon, current_weapon));
            let weapon_res = CALLBACK.call_func(event, None);
            if !weapon_res {
                player.give_weapon(event.get_weapon(), 0);
            } else {
                player.last_weapon = current_weapon;
            }
        }
    }
    // then ammo, not block it
    {
        let current_ammo = player.get_weapon_ammo();
        let last_ammo = player.last_ammo;
        if current_ammo != last_ammo {
            let _ = CALLBACK.call_func(
                PlayerAmmoEvent::from((player_id, last_ammo, current_ammo)),
                None,
            );
            player.last_ammo = current_ammo;
        }
    }
    // then move
    {
        let current_pos = player.get_position().into();
        let last_pos = player.last_position;
        if current_pos != last_pos {
            let event = PlayerMoveEvent::from((
                player_id,
                VectorPy::from(last_pos),
                VectorPy::from(current_pos),
            ));
            let move_res = CALLBACK.call_func(event, None);
            if !move_res {
                player.set_position(event.get_position());
            } else {
                player.last_position = current_pos;
            }
        }
    }
}

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
    callbacks.OnPlayerUpdate = Some(on_player_update);

    callbacks.OnEntityPoolChange = Some(on_entity_pool_change);
}
