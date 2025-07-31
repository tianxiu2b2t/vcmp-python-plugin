use std::ffi::c_char;

use crate::{PlayerId, VehicleId};

#[derive(Debug, Clone)]
pub struct IncomingConnectionEvent {
    pub player_name: String,
    pub password: String,
    pub ip: String,
}

impl IncomingConnectionEvent {
    pub fn new(player_name: String, password: String, ip: String) -> Self {
        Self {
            player_name,
            password,
            ip,
        }
    }
    pub fn player_name(&self) -> &str {
        &self.player_name
    }
}

impl From<(*mut c_char, usize, *const c_char, *const c_char)> for IncomingConnectionEvent {
    fn from(value: (*mut c_char, usize, *const c_char, *const c_char)) -> Self {
        unsafe {
            Self {
                player_name: std::ffi::CStr::from_ptr(value.0)
                    .to_string_lossy()
                    .to_string().trim_end_matches("\0").to_string(),
                password: std::ffi::CStr::from_ptr(value.2)
                    .to_string_lossy()
                    .to_string().trim_end_matches("\0").to_string(),
                ip: std::ffi::CStr::from_ptr(value.3)
                    .to_string_lossy()
                    .to_string().trim_end_matches("\0").to_string(),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClientScriptDataEvent {
    pub player_id: PlayerId,
    pub data: Vec<u8>,
}

impl From<(i32, *const u8, usize)> for ClientScriptDataEvent {
    fn from(value: (i32, *const u8, usize)) -> Self {
        unsafe {
            Self {
                player_id: value.0,
                data: std::slice::from_raw_parts(value.1, value.2).to_vec(),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerConnectEvent {
    pub player_id: PlayerId,
}

impl From<i32> for PlayerConnectEvent {
    fn from(value: i32) -> Self {
        Self { player_id: value }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerDisconnectEvent {
    pub player_id: PlayerId,
    pub reason: i32,
}

impl From<(i32, i32)> for PlayerDisconnectEvent {
    fn from(value: (i32, i32)) -> Self {
        Self {
            player_id: value.0,
            reason: value.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerRequestClassEvent {
    pub player_id: PlayerId,
    pub class_id: i32,
}

impl From<(i32, i32)> for PlayerRequestClassEvent {
    fn from(value: (i32, i32)) -> Self {
        Self {
            player_id: value.0,
            class_id: value.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerSpawnEvent {
    pub player_id: PlayerId,
}

impl From<i32> for PlayerSpawnEvent {
    fn from(value: i32) -> Self {
        Self { player_id: value }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerRequestSpawnEvent {
    pub player_id: PlayerId,
}

impl From<i32> for PlayerRequestSpawnEvent {
    fn from(value: i32) -> Self {
        Self { player_id: value }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerDeathEvent {
    pub player_id: PlayerId,
    pub killer_id: i32,
    pub reason: i32,
    pub body: i32,
}

impl From<(i32, i32, i32, i32)> for PlayerDeathEvent {
    fn from(value: (i32, i32, i32, i32)) -> Self {
        Self {
            player_id: value.0,
            killer_id: value.1,
            reason: value.2,
            body: value.3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerUpdateEvent {
    pub player_id: PlayerId,
    pub update: i32,
}

impl From<(i32, i32)> for PlayerUpdateEvent {
    fn from(value: (i32, i32)) -> Self {
        Self {
            player_id: value.0,
            update: value.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerRequestEnterVehicleEvent {
    pub player_id: PlayerId,
    pub vehicle_id: VehicleId,
    pub slot_index: i32,
}

impl From<(i32, i32, i32)> for PlayerRequestEnterVehicleEvent {
    fn from(value: (i32, i32, i32)) -> Self {
        Self {
            player_id: value.0,
            vehicle_id: value.1,
            slot_index: value.2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerEnterVehicleEvent {
    pub player_id: PlayerId,
    pub vehicle_id: VehicleId,
    pub slot_index: i32,
}

impl From<(i32, i32, i32)> for PlayerEnterVehicleEvent {
    fn from(value: (i32, i32, i32)) -> Self {
        Self {
            player_id: value.0,
            vehicle_id: value.1,
            slot_index: value.2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerExitVehicleEvent {
    pub player_id: PlayerId,
    pub vehicle_id: VehicleId,
}

impl From<(i32, i32)> for PlayerExitVehicleEvent {
    fn from(value: (i32, i32)) -> Self {
        Self {
            player_id: value.0,
            vehicle_id: value.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerNameChangeEvent {
    pub player_id: PlayerId,
    pub old_name: String,
    pub new_name: String,
}

impl From<(i32, *const c_char, *const c_char)> for PlayerNameChangeEvent {
    fn from(value: (i32, *const c_char, *const c_char)) -> Self {
        unsafe {
            Self {
                player_id: value.0,
                old_name: std::ffi::CStr::from_ptr(value.1)
                    .to_string_lossy()
                    .to_string(),
                new_name: std::ffi::CStr::from_ptr(value.2)
                    .to_string_lossy()
                    .to_string(),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerStateChangeEvent {
    pub player_id: PlayerId,
    pub old_state: i32,
    pub new_state: i32,
}

impl From<(i32, i32, i32)> for PlayerStateChangeEvent {
    fn from(value: (i32, i32, i32)) -> Self {
        Self {
            player_id: value.0,
            old_state: value.1,
            new_state: value.2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerActionChangeEvent {
    pub player_id: PlayerId,
    pub old_action: i32,
    pub new_action: i32,
}

impl From<(i32, i32, i32)> for PlayerActionChangeEvent {
    fn from(value: (i32, i32, i32)) -> Self {
        Self {
            player_id: value.0,
            old_action: value.1,
            new_action: value.2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerOnFireChangeEvent {
    pub player_id: PlayerId,
    pub is_on_fire: bool,
}

impl From<(i32, u8)> for PlayerOnFireChangeEvent {
    fn from(value: (i32, u8)) -> Self {
        Self {
            player_id: value.0,
            is_on_fire: value.1 != 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerCrouchChangeEvent {
    pub player_id: PlayerId,
    pub is_crouching: bool,
}

impl From<(i32, u8)> for PlayerCrouchChangeEvent {
    fn from(value: (i32, u8)) -> Self {
        Self {
            player_id: value.0,
            is_crouching: value.1 != 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerGameKeysChangeEvent {
    pub player_id: PlayerId,
    pub old_keys: u32,
    pub new_keys: u32,
}

impl From<(i32, u32, u32)> for PlayerGameKeysChangeEvent {
    fn from(value: (i32, u32, u32)) -> Self {
        Self {
            player_id: value.0,
            old_keys: value.1,
            new_keys: value.2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerBeginTypingEvent {
    pub player_id: PlayerId,
}

impl From<i32> for PlayerBeginTypingEvent {
    fn from(value: i32) -> Self {
        Self { player_id: value }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerEndTypingEvent {
    pub player_id: PlayerId,
}

impl From<i32> for PlayerEndTypingEvent {
    fn from(value: i32) -> Self {
        Self { player_id: value }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerAwayChangeEvent {
    pub player_id: PlayerId,
    pub is_away: bool,
}

impl From<(i32, u8)> for PlayerAwayChangeEvent {
    fn from(value: (i32, u8)) -> Self {
        Self {
            player_id: value.0,
            is_away: value.1 != 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerMessageEvent {
    pub player_id: PlayerId,
    pub message: String,
}

impl From<(i32, *const c_char)> for PlayerMessageEvent {
    fn from(value: (i32, *const c_char)) -> Self {
        unsafe {
            Self {
                player_id: value.0,
                message: std::ffi::CStr::from_ptr(value.1)
                    .to_string_lossy()
                    .to_string(),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerCommandEvent {
    pub player_id: PlayerId,
    pub command: String,
    pub text: String,
}

impl From<(i32, *const c_char)> for PlayerCommandEvent {
    fn from(value: (i32, *const c_char)) -> Self {
        let command = unsafe {
            std::ffi::CStr::from_ptr(value.1)
                .to_string_lossy()
                .to_string()
        };
        // command is in the format "command text" or "command"
        let mut split = command.splitn(2, ' ');
        let command = split.next().unwrap();
        let text = split.next().unwrap_or("");
        Self {
            player_id: value.0,
            command: command.to_string(),
            text: text.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerPrivateMessageEvent {
    pub player_id: PlayerId,
    pub target_id: PlayerId,
    pub message: String,
}

impl From<(i32, i32, *const c_char)> for PlayerPrivateMessageEvent {
    fn from(value: (i32, i32, *const c_char)) -> Self {
        unsafe {
            Self {
                player_id: value.0,
                target_id: value.1,
                message: std::ffi::CStr::from_ptr(value.2)
                    .to_string_lossy()
                    .to_string(),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerKeyBindDownEvent {
    pub player_id: PlayerId,
    pub bind_id: i32,
}

impl From<(i32, i32)> for PlayerKeyBindDownEvent {
    fn from(value: (i32, i32)) -> Self {
        Self {
            player_id: value.0,
            bind_id: value.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerKeyBindUpEvent {
    pub player_id: PlayerId,
    pub bind_id: i32,
}

impl From<(i32, i32)> for PlayerKeyBindUpEvent {
    fn from(value: (i32, i32)) -> Self {
        Self {
            player_id: value.0,
            bind_id: value.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerSpectateEvent {
    pub player_id: PlayerId,
    pub target_id: PlayerId,
}

impl From<(i32, i32)> for PlayerSpectateEvent {
    fn from(value: (i32, i32)) -> Self {
        Self {
            player_id: value.0,
            target_id: value.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerCrashReportEvent {
    pub player_id: PlayerId,
    pub report: String,
}

impl From<(i32, *const c_char)> for PlayerCrashReportEvent {
    fn from(value: (i32, *const c_char)) -> Self {
        unsafe {
            Self {
                player_id: value.0,
                report: std::ffi::CStr::from_ptr(value.1)
                    .to_string_lossy()
                    .to_string(),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerModuleListEvent {
    pub player_id: PlayerId,
    pub modules: String,
}

impl From<(i32, *const c_char)> for PlayerModuleListEvent {
    fn from(value: (i32, *const c_char)) -> Self {
        unsafe {
            Self {
                player_id: value.0,
                modules: std::ffi::CStr::from_ptr(value.1)
                    .to_string_lossy()
                    .to_string(),
            }
        }
    }
}
