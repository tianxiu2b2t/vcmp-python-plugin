use std::{
    ffi::{c_char, CString}, str::FromStr
};

pub struct IncomingConnectionEvent {
    player_name: String,
    pub passowrd: String,
    pub ip: String,
    name_ptr: *mut c_char,
    name_buffer_len: usize,
}

impl IncomingConnectionEvent {
    pub fn player_name(&self) -> &str {
        &self.player_name
    }

    pub fn set_player_name(&mut self, name: String) -> bool {
        let c_name = CString::from_str(&name).expect("invalid name");
        let c_name_len = c_name.as_bytes().len();
        if c_name_len > self.name_buffer_len {
            return false;
        }
        // set 0 to name_ptr
        unsafe {
            std::ptr::write_bytes(self.name_ptr, 0, self.name_buffer_len);
        }
        // write name to name_ptr
        unsafe {
            std::ptr::copy_nonoverlapping(c_name.as_ptr(), self.name_ptr, c_name_len);
        }
        self.player_name = name;
        true
    }
}

impl From<(*mut c_char, usize, *const c_char, *const c_char)> for IncomingConnectionEvent {
    fn from(value: (*mut c_char, usize, *const c_char, *const c_char)) -> Self {
        unsafe {
            Self {
                player_name: std::ffi::CStr::from_ptr(value.0)
                    .to_string_lossy()
                    .to_string(),
                passowrd: std::ffi::CStr::from_ptr(value.2)
                    .to_string_lossy()
                    .to_string(),
                ip: std::ffi::CStr::from_ptr(value.3)
                    .to_string_lossy()
                    .to_string(),
                name_ptr: value.0,
                name_buffer_len: value.1,
            }
        }
    }
}

pub struct ClientScriptDataEvent {
    pub player_id: i32,
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

pub struct PlayerConnectEvent {
    pub player_id: i32,
}

impl From<i32> for PlayerConnectEvent {
    fn from(value: i32) -> Self {
        Self { player_id: value }
    }
}

pub struct PlayerDisconnectEvent {
    pub player_id: i32,
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

pub struct PlayerRequestClassEvent {
    pub player_id: i32,
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

pub struct PlayerSpawnEvent {
    pub player_id: i32,
}

impl From<i32> for PlayerSpawnEvent {
    fn from(value: i32) -> Self {
        Self { player_id: value }
    }
}

pub struct PlayerRequestSpawn {
    pub player_id: i32,
}

impl From<i32> for PlayerRequestSpawn {
    fn from(value: i32) -> Self {
        Self { player_id: value }
    }
}

pub struct PlayerDeathEvent {
    pub player_id: i32,
    pub killer_id: i32,
    pub reason: i32,
    pub body: i32
} 

impl From<(i32, i32, i32, i32)> for PlayerDeathEvent {
    fn from(value: (i32, i32, i32, i32)) -> Self {
        Self {
            player_id: value.0,
            killer_id: value.1,
            reason: value.2,
            body: value.3
        }
    }
}

pub struct PlayerUpdateEvent {
    pub player_id: i32,
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

pub struct PlayerRequestEnterVehicleEvent {
    pub player_id: i32,
    pub vehicle_id: i32,
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

pub struct PlayerEnterVehicleEvent {
    pub player_id: i32,
    pub vehicle_id: i32,
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

pub struct PlayerExitVehicleEvent {
    pub player_id: i32,
    pub vehicle_id: i32,
}

impl From<(i32, i32)> for PlayerExitVehicleEvent {
    fn from(value: (i32, i32)) -> Self {
        Self {
            player_id: value.0,
            vehicle_id: value.1,
        }
    }
}

pub struct PlayerNameChangeEvent {
    pub player_id: i32,
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

pub struct PlayerStateChangeEvent {
    pub player_id: i32,
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

pub struct PlayerActionChangeEvent {
    pub player_id: i32,
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

pub struct PlayerOnFireChangeEvent {
    pub player_id: i32,
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

pub struct PlayerCrouchChangeEvent {
    pub player_id: i32,
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

pub struct PlayerGameKeysChangeEvent {
    pub player_id: i32,
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

pub struct PlayerBeginTypingEvent {
    pub player_id: i32,
}

impl From<i32> for PlayerBeginTypingEvent {
    fn from(value: i32) -> Self {
        Self { player_id: value }
    }
}

pub struct PlayerEndTypingEvent {
    pub player_id: i32,
}

impl From<i32> for PlayerEndTypingEvent {
    fn from(value: i32) -> Self {
        Self { player_id: value }
    }
}

pub struct PlayerAwayChangeEvent {
    pub player_id: i32,
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

pub struct PlayerMessageEvent {
    pub player_id: i32,
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

pub struct PlayerCommandEvent {
    pub player_id: i32,
    pub command: String,
}

impl From<(i32, *const c_char)> for PlayerCommandEvent {
    fn from(value: (i32, *const c_char)) -> Self {
        unsafe {
            Self {
                player_id: value.0,
                command: std::ffi::CStr::from_ptr(value.1)
                    .to_string_lossy()
                    .to_string(),
            }
        }
    }
}

pub struct PlayerPrivateMessageEvent {
    pub player_id: i32,
    pub target_player_id: i32,
    pub message: String,
}

impl From<(i32, i32, *const c_char)> for PlayerPrivateMessageEvent {
    fn from(value: (i32, i32, *const c_char)) -> Self {
        unsafe {
            Self {
                player_id: value.0,
                target_player_id: value.1,
                message: std::ffi::CStr::from_ptr(value.2)
                    .to_string_lossy()
                    .to_string(),
            }
        }
    }
}

pub struct PlayerKeyBindDownEvent {
    pub player_id: i32,
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

pub struct PlayerKeyBindUpEvent {
    pub player_id: i32,
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

pub struct PlayerSpectateEvent {
    pub player_id: i32,
    pub target_player_id: i32,
}

impl From<(i32, i32)> for PlayerSpectateEvent {
    fn from(value: (i32, i32)) -> Self {
        Self {
            player_id: value.0,
            target_player_id: value.1,
        }
    }
}

pub struct PlayerCrashReportEvent {
    pub player_id: i32,
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

pub struct PlayerModuleListEvent {
    pub player_id: i32,
    pub modules: String
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