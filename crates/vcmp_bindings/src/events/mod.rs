use std::ffi::c_char;

pub struct PluginCommandEvent {
    pub identifer: u32,
    pub message: String,
}

pub mod checkpoint;
pub mod object;
pub mod pickup;
pub mod player;
pub mod server;
pub mod vehicle;

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
    pub player_id: i32,
    pub entity_id: i32,
    pub entity_type: i32,
    pub deleted: bool,
}

impl From<(i32, i32, i32, u8)> for EntityStreamingChangeEvent {
    fn from(value: (i32, i32, i32, u8)) -> Self {
        Self {
            player_id: value.0,
            entity_id: value.1,
            entity_type: value.2,
            deleted: value.3 != 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntityPoolChangeEvent {
    pub entity_type: i32,
    pub entity_id: i32,
    pub deleted: bool,
}

impl From<(i32, i32, u8)> for EntityPoolChangeEvent {
    fn from(value: (i32, i32, u8)) -> Self {
        Self {
            entity_type: value.0,
            entity_id: value.1,
            deleted: value.2 != 0,
        }
    }
}
