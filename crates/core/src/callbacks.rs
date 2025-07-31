use std::os::raw::c_char;

use crate::py::callbacks::PY_CALLBACK_MANAGER;
use crate::py::events::{
    VcmpEvent, checkpoint::*, object::*, pickup::*, player::*, server::*, vehicle::*,
};
use crate::py::types::VectorPy;
use squirrel_ffi::init_squirrel;
use vcmp_bindings::events::{checkpoint, object, pickup, player, server, vehicle};
use vcmp_bindings::func::{PlayerMethods, QueryVehicle, SetVehicle};
use vcmp_bindings::vcmp_func;
use vcmp_bindings::{options::VcmpEntityPool, raw::PluginCallbacks};

use crate::{cfg::CONFIG, pool::ENTITY_POOL, py::load_script, py::reload};
use tracing::{Level, event};

// FFI Squirrel
use squirrel_ffi::sq::SQUIRREL_LOAD_IDENTIFIER;

// use crate::py::callbacks::CALLBACK;

#[unsafe(no_mangle)]
pub extern "C" fn on_server_init() -> u8 {
    if !CONFIG.get().unwrap().preloader {
        load_script();
    }

    let _ =
        PY_CALLBACK_MANAGER.handle(VcmpEvent::ServerInitialise(ServerInitialiseEvent {}), false);

    1
}

#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn on_server_frame(elapsed_time: f32) {
    // first check need reload

    reload();

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
        on_entity_pool_change(VcmpEntityPool::Player as i32, player_id, 0);
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
        on_entity_pool_change(VcmpEntityPool::Player as i32, player_id, 1);
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
    {
        // set loaded
        let mut pool = ENTITY_POOL.lock().unwrap();
        let player = pool.get_mut_player(player_id).unwrap();
        player.set_var_loaded(true);
    }

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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_player_update(player_id: i32, state: i32) {
    {
        // first health
        {
            // use raw vcmp_bindings
            let current_health = vcmp_func().get_player_health(player_id);
            let last_health = {
                let mut pool = ENTITY_POOL.lock().unwrap();
                let player = pool.get_mut_player(player_id).unwrap();
                player.get_var_last_health()
            };
            if current_health != last_health {
                let event = PlayerHealthChangeEvent::from((player_id, last_health, current_health));
                let health_res =
                    PY_CALLBACK_MANAGER.handle(VcmpEvent::PlayerHealthChange(event), true);
                if !health_res {
                    let _ = vcmp_func().set_player_health(player_id, {
                        let mut pool = ENTITY_POOL.lock().unwrap();
                        let player = pool.get_mut_player(player_id).unwrap();
                        player.get_var_last_health()
                    });
                }
            }
        }
        // then armour
        {
            let current_armour = vcmp_func().get_player_armour(player_id);
            let last_armour = {
                let mut pool = ENTITY_POOL.lock().unwrap();
                let player = pool.get_mut_player(player_id).unwrap();
                player.get_var_last_armour()
            };
            if current_armour != last_armour {
                let event = PlayerArmourChangeEvent::from((player_id, last_armour, current_armour));
                let armour_res =
                    PY_CALLBACK_MANAGER.handle(VcmpEvent::PlayerArmourChange(event), true);
                if !armour_res {
                    let _ = vcmp_func().set_player_armour(player_id, {
                        let mut pool = ENTITY_POOL.lock().unwrap();
                        let player = pool.get_mut_player(player_id).unwrap();
                        player.get_var_last_armour()
                    });
                }
            }
        }
        // then weapon
        {
            let current_weapon = vcmp_func().get_player_weapon(player_id);
            let last_weapon = {
                let mut pool = ENTITY_POOL.lock().unwrap();
                let player = pool.get_mut_player(player_id).unwrap();
                player.get_var_last_weapon()
            };
            if current_weapon != last_weapon {
                let event = PlayerWeaponChangeEvent::from((player_id, last_weapon, current_weapon));
                let weapon_res =
                    PY_CALLBACK_MANAGER.handle(VcmpEvent::PlayerWeaponChange(event), true);
                if !weapon_res {
                    let _ = vcmp_func().give_player_weapon(
                        player_id,
                        {
                            let mut pool = ENTITY_POOL.lock().unwrap();
                            let player = pool.get_mut_player(player_id).unwrap();
                            player.get_var_last_weapon()
                        },
                        0,
                    );
                }
            }
        }
        // then ammo
        {
            let current_ammo = vcmp_func().get_player_weapon_ammo(player_id);
            let current_wep = vcmp_func().get_player_weapon(player_id);
            let last_ammo = {
                let mut pool = ENTITY_POOL.lock().unwrap();
                let player = pool.get_mut_player(player_id).unwrap();
                player.get_var_last_ammo()
            };
            if current_ammo != last_ammo {
                let event = PlayerAmmoChangeEvent::from((player_id, last_ammo, current_ammo));
                let res = PY_CALLBACK_MANAGER.handle(VcmpEvent::PlayerAmmoChange(event), true);
                if !res {
                    let real_ammo = vcmp_func().get_player_weapon_ammo(player_id);
                    let restore_ammo = {
                        let mut pool = ENTITY_POOL.lock().unwrap();
                        let player = pool.get_mut_player(player_id).unwrap();
                        player.get_var_last_ammo()
                    } - real_ammo;
                    let _ = vcmp_func().give_player_weapon(player_id, current_wep, restore_ammo);
                }
            }
        }
        // then move
        {
            let current_pos = vcmp_func()
                .get_player_position(player_id)
                .unwrap_or_default();
            let last_pos = {
                let mut pool = ENTITY_POOL.lock().unwrap();
                let player = pool.get_mut_player(player_id).unwrap();
                player.get_var_last_position()
            };
            if current_pos != last_pos {
                let event = PlayerMoveEvent::from((
                    player_id,
                    VectorPy::from(last_pos),
                    VectorPy::from(current_pos),
                ));
                let move_res = PY_CALLBACK_MANAGER.handle(VcmpEvent::PlayerMove(event), true);
                if !move_res {
                    let _ = vcmp_func().set_player_position(player_id, {
                        let mut pool = ENTITY_POOL.lock().unwrap();
                        let player = pool.get_mut_player(player_id).unwrap();
                        player.get_var_last_position()
                    });
                }
            }
        }
    }
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PlayerUpdate(PlayerUpdateEvent::from(player::PlayerUpdateEvent::from((
            player_id, state,
        )))),
        true,
    );
}

/// # Safety
/// FFI callback for vehicle update
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_vehicle_update(vehicle_id: i32, update_type: i32) {
    {
        let mut pool = ENTITY_POOL.lock().unwrap();
        let vehicle = pool.get_mut_vehicle(vehicle_id).unwrap();
        if vehicle.get_var_updating() {
            return;
        }
        vehicle.set_var_updating(true);
    }
    {
        {
            // health change
            let current_health = vcmp_func().get_vehicle_health(vehicle_id);
            let last_health = {
                let mut pool = ENTITY_POOL.lock().unwrap();
                let vehicle = pool.get_mut_vehicle(vehicle_id).unwrap();
                vehicle.get_var_last_health()
            };
            if current_health != last_health {
                let event =
                    VehicleHealthChangeEvent::from((vehicle_id, last_health, current_health));
                let health_res =
                    PY_CALLBACK_MANAGER.handle(VcmpEvent::VehicleHealthChange(event), true);
                if !health_res {
                    let _ = vcmp_func().set_vehicle_health(vehicle_id, {
                        let mut pool = ENTITY_POOL.lock().unwrap();
                        let vehicle = pool.get_mut_vehicle(vehicle_id).unwrap();
                        vehicle.get_var_last_health()
                    });
                }
            }
        }
        {
            // move change
            let current_pos = vcmp_func().get_vehicle_position(vehicle_id);
            let last_pos = {
                let mut pool = ENTITY_POOL.lock().unwrap();
                let vehicle = pool.get_mut_vehicle(vehicle_id).unwrap();
                vehicle.get_var_last_position()
            };
            if current_pos != last_pos {
                let event = VehicleMoveEvent::from((
                    vehicle_id,
                    VectorPy::from(last_pos),
                    VectorPy::from(current_pos),
                ));
                let move_res = PY_CALLBACK_MANAGER.handle(VcmpEvent::VehicleMove(event), true);
                if !move_res {
                    let _ = vcmp_func().set_vehicle_position(
                        vehicle_id,
                        {
                            // fix object clone bug
                            let mut pool = ENTITY_POOL.lock().unwrap();
                            let vehicle = pool.get_mut_vehicle(vehicle_id).unwrap();
                            vehicle.get_var_last_position()
                        },
                        Some(false),
                    );
                }
            }
        }
    }
    let binding_event = vehicle::VehicleUpdateEvent::from((vehicle_id, update_type));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::VehicleUpdate(VehicleUpdateEvent::from(binding_event)),
        false,
    );

    {
        let mut pool = ENTITY_POOL.lock().unwrap();
        let vehicle = pool.get_mut_vehicle(vehicle_id).unwrap();
        vehicle.set_var_updating(false);
    }
}

/// # Safety
/// FFI callback for vehicle explode
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_vehicle_explode(vehicle_id: i32) {
    let binding_event = vehicle::VehicleExplodeEvent::from(vehicle_id);
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::VehicleExplode(VehicleExplodeEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for vehicle respawn
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_vehicle_respawn(vehicle_id: i32) {
    let binding_event = vehicle::VehicleRespawnEvent::from(vehicle_id);
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::VehicleRespawn(VehicleRespawnEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for object shot
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_object_shot(object_id: i32, player_id: i32, weapon_id: i32) {
    let binding_event = object::ObjectShotEvent::from((object_id, player_id, weapon_id));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::ObjectShot(ObjectShotEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for object touched
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_object_touched(object_id: i32, player_id: i32) {
    let binding_event = object::ObjectTouchedEvent::from((object_id, player_id));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::ObjectTouched(ObjectTouchedEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for pickup pick attempt
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_pickup_pick_attempt(pickup_id: i32, player_id: i32) -> u8 {
    let binding_event = pickup::PickupPickAttemptEvent::from((pickup_id, player_id));
    PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PickupPickAttempt(PickupPickAttemptEvent::from(binding_event)),
        true,
    ) as u8
}

/// # Safety
/// FFI callback for pickup picked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_pickup_picked(pickup_id: i32, player_id: i32) {
    let binding_event = pickup::PickupPickedEvent::from((pickup_id, player_id));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PickupPicked(PickupPickedEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for pickup respawn
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_pickup_respawn(pickup_id: i32) {
    let binding_event = pickup::PickupRespawnEvent::from(pickup_id);
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::PickupRespawn(PickupRespawnEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for checkpoint entered
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_checkpoint_entered(checkpoint_id: i32, player_id: i32) {
    let binding_event = checkpoint::CheckpointEnteredEvent::from((checkpoint_id, player_id));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::CheckpointEntered(CheckpointEnteredEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for checkpoint exited
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_checkpoint_exited(checkpoint_id: i32, player_id: i32) {
    let binding_event = checkpoint::CheckpointExitedEvent::from((checkpoint_id, player_id));
    let _ = PY_CALLBACK_MANAGER.handle(
        VcmpEvent::CheckpointExited(CheckpointExitedEvent::from(binding_event)),
        false,
    );
}

/// # Safety
/// FFI callback for plugin command
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_plugin_command(identifier: u32, command: *const c_char) -> u8 {
    println!("on_plugin_command: {} {:?}", identifier, command);
    // FFI Squirrel
    match identifier {
        SQUIRREL_LOAD_IDENTIFIER => {
            init_squirrel();
        },
        _ => {}
    } 
    1
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

    // 车辆相关回调设置
    callbacks.OnVehicleUpdate = Some(on_vehicle_update);
    callbacks.OnVehicleExplode = Some(on_vehicle_explode);
    callbacks.OnVehicleRespawn = Some(on_vehicle_respawn);

    // 对象相关回调设置
    callbacks.OnObjectShot = Some(on_object_shot);
    callbacks.OnObjectTouched = Some(on_object_touched);

    // 拾取物相关回调设置
    callbacks.OnPickupPickAttempt = Some(on_pickup_pick_attempt);
    callbacks.OnPickupPicked = Some(on_pickup_picked);
    callbacks.OnPickupRespawn = Some(on_pickup_respawn);

    // 检查点相关回调设置
    callbacks.OnCheckpointEntered = Some(on_checkpoint_entered);
    callbacks.OnCheckpointExited = Some(on_checkpoint_exited);

    callbacks.OnEntityPoolChange = Some(on_entity_pool_change);

    callbacks.OnPluginCommand = Some(on_plugin_command);
}
