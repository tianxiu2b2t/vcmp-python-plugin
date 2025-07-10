use std::ffi::CString;
use std::path::Path;

use pyo3::types::{PyModule, PyModuleMethods, PyTracebackMethods};
use pyo3::{Bound, Py, PyErr, PyResult, Python, pymodule};
use tracing::{Level, event};

use crate::cfg::CONFIG;
use crate::functions;
use crate::functions::checkpoint::CheckPointPy;
use crate::functions::marker::MarkerPy;
use crate::functions::object::ObjectPy;
use crate::functions::pickup::PickupPy;
use crate::functions::player::PlayerPy;
use crate::functions::vehicle::VehiclePy;

pub mod callbacks;
pub mod events;
pub mod exceptions;
pub mod streams;
pub mod types;
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
    let util_module = PyModule::new(py, "util")?;
    util::module_define(py, &util_module)?;
    fix_module_name(py, &util_module, "util");
    m.add_submodule(&util_module)?;

    let streams_module = PyModule::new(py, "streams")?;
    streams::module_define(py, &streams_module)?;
    fix_module_name(py, &streams_module, "streams");
    m.add_submodule(&streams_module)?;

    let types_module = PyModule::new(py, "types")?;
    types::module_define(py, &types_module)?;
    fix_module_name(py, &types_module, "types");
    m.add_submodule(&types_module)?;

    let funcs_module = PyModule::new(py, "functions")?;
    functions::module_define(py, &funcs_module)?;
    fix_module_name(py, &funcs_module, "functions");
    m.add_submodule(&funcs_module)?;

    let callbacks_module = PyModule::new(py, "callback")?;
    callbacks::module_define(py, &callbacks_module)?;
    fix_module_name(py, &callbacks_module, "callback");
    m.add_submodule(&callbacks_module)?;

    {
        let events_module = PyModule::new(py, "events")?;
        events::module_define(py, &events_module)?;
        fix_module_name(py, &events_module, "events");
        m.add_submodule(&events_module)?;
    }

    let exceptions_module = PyModule::new(py, "exceptions")?;
    exceptions::module_define(py, &exceptions_module)?;
    fix_module_name(py, &exceptions_module, "exceptions");
    m.add_submodule(&exceptions_module)?;

    {
        // import instance from player, ...
        let instance_module = PyModule::new(py, "instance")?;
        instance_module.add_class::<PlayerPy>()?;
        instance_module.add_class::<VehiclePy>()?;
        instance_module.add_class::<CheckPointPy>()?;
        instance_module.add_class::<ObjectPy>()?;
        instance_module.add_class::<PickupPy>()?;
        instance_module.add_class::<MarkerPy>()?;
        fix_module_name(py, &instance_module, "instance");
        m.add_submodule(&instance_module)?;
    }

    Ok(())
}

/// 在 python 初始化之前注册所有需要的类
///
/// WARNING: 这个函数需要在 Python 初始化之前调用，否则会导致报错
///
/// The code by shenjack
///
/// https://github.com/shenjackyuanjie/icalingua-bridge-bot/blob/d827f6c36e7787693f47f0b9167324fa24fb0538/ica-rs/src/py/class.rs#L94-L110
pub fn init_py_module() {
    unsafe {
        // 单纯没用 macro 而已
        pyo3::ffi::PyImport_AppendInittab(
            register_module::__PYO3_NAME.as_ptr(),
            Some(register_module::__pyo3_init),
        );
    }
}

pub fn fix_module_name(py: Python<'_>, module: &Bound<'_, PyModule>, name: &str) {
    pyo3::py_run!(
        py,
        module,
        format!("import sys; sys.modules['vcmp.{name}'] = module").as_str()
    );
}

pub fn init_py() {
    init_py_module();
    let virtual_env = CONFIG.get().unwrap().virtual_env.as_str();

    let mut config;
    unsafe {
        config = std::mem::zeroed::<pyo3::ffi::PyConfig>();
        let config_ptr = &mut config as *mut pyo3::ffi::PyConfig;
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

        event!(Level::INFO, "Status: {}", pyo3::ffi::Py_IsInitialized());
    }

    event!(Level::INFO, "Python init done");

    if CONFIG.get().unwrap().preloader {
        load_script_as_module();
    }
}

pub fn load_script_as_module() {
    let script_path = CONFIG.get().unwrap().script_path.as_str();
    let res = raw_load_script_as_module(Path::new(script_path));
    if let Err(e) = res {
        event!(Level::ERROR, "Error: {}", get_traceback(&e, None));
    } else {
        event!(Level::INFO, "Script loaded");
    }
}

pub fn raw_load_script_as_module(script: &Path) -> PyResult<Py<PyModule>> {
    // check exists
    if !script.exists() {
        return Err(pyo3::exceptions::PyRuntimeWarning::new_err(
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

pub fn bytes_repr(data: Vec<u8>) -> String {
    let mut result = String::from("b'");

    for &byte in data.iter() {
        match byte {
            // 常见转义字符
            b'\n' => result.push_str("\\n"),
            b'\r' => result.push_str("\\r"),
            b'\t' => result.push_str("\\t"),
            b'\\' => result.push_str("\\\\"),
            b'\'' => result.push_str("\\'"),
            b'"' => result.push_str("\\\""),
            b'\0' => result.push_str("\\x00"),
            // 可打印ASCII字符（32-126）
            32..=126 => result.push(byte as char),
            // 其他字节用十六进制表示
            _ => result.push_str(&format!("\\x{byte:02x}")),
        }
    }

    result.push('\'');
    result
}

/// 获取 python 错误信息
///
/// 可以提供一个 gil 来减少 gil 获取次数
pub fn get_traceback(py_err: &PyErr, py: Option<Python<'_>>) -> String {
    let traceback = match py {
        Some(py) => match py_err.traceback(py) {
            Some(traceback) => traceback.format().unwrap_or_else(|e| format!("{e:?}")),
            None => "none traceback".to_string(),
        },
        None => Python::with_gil(|py| match py_err.traceback(py) {
            Some(traceback) => traceback.format().unwrap_or_else(|e| format!("{e:?}")),
            None => "none traceback".to_string(),
        }),
    };

    format!("{traceback}{py_err}")
}
