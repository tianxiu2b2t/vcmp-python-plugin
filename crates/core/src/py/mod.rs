use std::ffi::CString;
use std::path::Path;

use pyo3::{Bound, Py, PyResult, Python, pymodule};

use pyo3::types::{PyModule, PyModuleMethods};

use crate::{cfg::CONFIG, func::util::module_define as util_define, py::util::create_submodule};

pub mod util;

#[cfg(target_os = "linux")]
fn get_wchar_t(content: &str) -> Vec<i32> {
    content
        .as_bytes()
        .iter()
        .map(|i| *i as i32)
        .collect::<Vec<i32>>()
}
#[cfg(target_os = "windows")]
fn get_wchar_t(content: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(content)
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<u16>>()
}

#[pymodule]
#[pyo3(name = "vcmp")]
fn register_module(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_submodule(&create_submodule(py, "util", util_define))?;

    Ok(())
}

/// 在 python 初始化之前注册所有需要的类
///
/// WARNING: 这个函数需要在 Python 初始化之前调用，否则会导致报错
///
/// The code by shenjack
pub fn init_py_module() {
    unsafe {
        // 单纯没用 macro 而已
        pyo3::ffi::PyImport_AppendInittab(
            register_module::__PYO3_NAME.as_ptr(),
            Some(register_module::__pyo3_init),
        );
    }
}

pub fn init_py() {
    init_py_module();
    let virtual_env = CONFIG.get().unwrap().virtual_env.as_str();

    let mut config;
    unsafe {
        config = std::mem::zeroed::<pyo3::ffi::PyConfig>();
        let config_ptr = &mut config as *mut pyo3::ffi::PyConfig; // let config_ptr = &mut config as *mut pyo3::ffi::PyConfig;
        pyo3::ffi::PyConfig_InitPythonConfig(config_ptr);

        if !virtual_env.is_empty() {
            pyo3::ffi::PyConfig_SetString(
                config_ptr,
                &mut config.prefix as *mut _,
                get_wchar_t(virtual_env).as_ptr(),
            );

            pyo3::ffi::PyConfig_SetString(
                config_ptr,
                &mut config.exec_prefix as *mut _,
                get_wchar_t(virtual_env).as_ptr(),
            );
        }

        config.install_signal_handlers = 0; // 必须设置为 false，不然会导致 Server 捕捉不到信号，从而导致进程无法正常退出

        pyo3::ffi::Py_InitializeFromConfig(&config as *const _);

        pyo3::ffi::PyEval_SaveThread();

        pyo3::ffi::PyConfig_Clear(config_ptr);

        println!("Status: {}", pyo3::ffi::Py_IsInitialized());
    };

    println!("Python init");

    let script_path = CONFIG.get().unwrap().script_path.as_str();

    load_script_as_module(Path::new(script_path)).unwrap();
}

pub fn load_script_as_module(script: &Path) -> PyResult<Py<PyModule>> {
    // check exists
    if !script.exists() {
        return Err(pyo3::exceptions::PyRuntimeError::new_err(
            "Script not found",
        ));
    }
    let code = CString::new(std::fs::read_to_string(script).unwrap_or_default())
        .expect("faild to create c string for code");
    let c_path = CString::new(script.to_str().unwrap_or_default())
        .expect("faild to create c string for path");
    Python::with_gil(|py| {
        let module = PyModule::from_code(
            py,
            &code,
            &c_path,
            &CString::new("_runner_vcmp").expect("faild to create c string for module name"),
        )?;
        Ok(module.unbind())
    })
}
