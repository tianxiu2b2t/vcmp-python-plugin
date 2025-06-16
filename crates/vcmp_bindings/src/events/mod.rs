use std::{
    ffi::{CString, c_char},
    str::FromStr,
};

pub struct PluginCommandEvent {
    pub identifer: u32,
    pub message: String,
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
