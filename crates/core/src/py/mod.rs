use crate::cfg::CONFIG;

pub mod util;

#[cfg(target_os = "linux")]
fn get_wchar_t(content: &str) -> Vec<i32> {
    content.as_bytes().iter().map(|i| *i as i32).collect::<Vec<i32>>()
}
#[cfg(target_os = "windows")]
fn get_wchar_t(content: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(content).encode_wide().chain(Some(0)).collect::<Vec<u16>>()
}

pub fn init_py() {
    let virtual_env = CONFIG.get().unwrap().virtual_env.as_str();

    let mut config;
    unsafe {
        config = std::mem::zeroed::<pyo3::ffi::PyConfig>();
        let config_ptr = &mut config as *mut pyo3::ffi::PyConfig; // let config_ptr = &mut config as *mut pyo3::ffi::PyConfig;
        pyo3::ffi::PyConfig_InitPythonConfig(config_ptr);

        if !virtual_env.is_empty() {
            pyo3::ffi::PyConfig_SetString(config_ptr, &mut config.prefix as *mut _, get_wchar_t(virtual_env).as_ptr()); 
            
            pyo3::ffi::PyConfig_SetString(config_ptr, &mut config.exec_prefix as *mut _, get_wchar_t(virtual_env).as_ptr());
            
        }

        config.install_signal_handlers = 0; // 必须设置为 false，不然会导致 Server 捕捉不到信号，从而导致进程无法正常退出

        pyo3::ffi::Py_InitializeFromConfig(&config as *const _);

        pyo3::ffi::PyEval_SaveThread();

        pyo3::ffi::PyConfig_Clear(config_ptr);

        println!("Status: {}", pyo3::ffi::Py_IsInitialized());
    };

    println!("Python init");

}