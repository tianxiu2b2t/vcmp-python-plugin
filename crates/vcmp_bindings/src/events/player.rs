use std::{
    ffi::{c_char, CString}, fmt::Display, str::FromStr
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
#[derive(Debug, Clone)]
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

impl Display for ClientScriptDataEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClientScriptDataEvent {{ player_id: {}, data: {:?} }}", self.player_id, self.data)
    }
}