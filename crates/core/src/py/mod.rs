use std::collections::HashMap;
use std::ffi::CString;
use std::path::Path;
use std::sync::{LazyLock, Mutex};
use std::time::Instant;

use pyo3::types::{
    PyAnyMethods, PyDict, PyDictMethods, PyModule, PyModuleMethods, PyTracebackMethods,
};
use pyo3::{Bound, Py, PyAny, PyErr, PyResult, Python, pyfunction, pymodule, wrap_pyfunction};
use tracing::{Level, event};

use crate::cfg::CONFIG;
use crate::functions;
use crate::functions::checkpoint::CheckPointPy;
use crate::functions::marker::MarkerPy;
use crate::functions::object::ObjectPy;
use crate::functions::pickup::PickupPy;
use crate::functions::player::PlayerPy;
use crate::functions::vehicle::VehiclePy;
use crate::pool::ENTITY_POOL;
use crate::py::callbacks::{PY_CALLBACK_MANAGER, PY_CALLBACK_STORAGE};
use crate::py::events::player::{
    PlayerConnectEvent, PlayerDisconnectEvent, PlayerRequestClassEvent, PlayerSpawnEvent,
};
use crate::py::events::server::{ServerInitialiseEvent, ServerReloadedEvent, ServerShutdownEvent};
use crate::py::events::{PyVcmpEvent, VcmpEvent};

pub mod callbacks;
pub mod events;
pub mod pool;
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

#[derive(Clone, Debug, Default)]
pub struct GlobalVar {
    pub need_reload: bool,
    pub capture_modules: Option<Vec<String>>,
    pub reload_var: Option<HashMap<String, Py<PyAny>>>,
    pub error_handler: Option<Py<PyAny>>,
}

/// 非常好的 CPython signal, 使我 OSError: Signal 2 ignored due to race condition
/// ^^
pub static IGNORE_MODULES: LazyLock<Vec<String>> =
    LazyLock::new(|| vec!["signal".to_string(), "_signal".to_string()]);

pub static GLOBAL_VAR: LazyLock<Mutex<GlobalVar>> =
    LazyLock::new(|| Mutex::new(GlobalVar::default()));

pub fn init_py_environment() {
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

    let pool_module = PyModule::new(py, "pool")?;
    pool::module_define(py, &pool_module)?;
    fix_module_name(py, &pool_module, "pool");
    m.add_submodule(&pool_module)?;

    m.add_function(wrap_pyfunction!(py_reload, m)?)?;
    m.add_function(wrap_pyfunction!(py_set_error_handler, m)?)?;
    m.add_function(wrap_pyfunction!(py_get_error_handler, m)?)?;

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

/// 获取 python 错误信息
///
/// 可以提供一个 gil 来减少 gil 获取次数
pub fn get_traceback(py_err: &PyErr, py: Option<Python<'_>>) -> String {
    let traceback = match py {
        Some(py) => match py_err.traceback(py) {
            Some(traceback) => traceback.format().unwrap_or_else(|e| format!("{e:?}")),
            None => "Traceback (most recent call last):\n  (Rust) Code\n\n".to_string(),
        },
        None => Python::with_gil(|py| match py_err.traceback(py) {
            Some(traceback) => traceback.format().unwrap_or_else(|e| format!("{e:?}")),
            None => "Traceback (most recent call last):\n  (Rust) Code\n\n".to_string(),
        }),
    };

    format!("{traceback}{py_err}")
}

/// repr 字节
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

pub fn capture_modules(py: Option<Python<'_>>) {
    let func = |py: Python<'_>| {
        let sys_modules = py.import("sys").unwrap().getattr("modules").unwrap();
        sys_modules
            .extract::<HashMap<String, Py<PyAny>>>()
            .unwrap()
            .iter()
            .map(|(k, _)| k.clone())
            .collect::<Vec<_>>()
    };
    let modules = match py {
        Some(py) => func(py),
        None => Python::with_gil(func),
    };

    event!(Level::DEBUG, "Capture modules: {:?}", modules.clone());

    GLOBAL_VAR.lock().unwrap().capture_modules = Some(modules.clone());
}

pub fn init_py() {
    init_py_module();
    init_py_environment();
    capture_modules(None);

    if CONFIG.get().unwrap().preloader {
        load_script();
    }
}

#[pyfunction]
#[pyo3(name = "reload", signature = (**kwargs))]
pub fn py_reload(kwargs: Option<HashMap<String, Py<PyAny>>>) {
    let var = GLOBAL_VAR.try_lock();
    if var.is_err() {
        event!(Level::ERROR, "Script reload failed, global var lock failed");
        return;
    }
    let mut var = var.unwrap();
    if var.need_reload {
        event!(Level::DEBUG, "Script already reloading");
        return;
    }
    event!(
        Level::DEBUG,
        "Script need reload, kwargs: {:?}",
        kwargs.clone()
    );
    var.need_reload = true;
    var.reload_var = kwargs;
}

#[pyfunction]
#[pyo3(name = "set_error_handler", signature = (handler))]
pub fn py_set_error_handler(handler: Py<PyAny>) {
    GLOBAL_VAR.lock().unwrap().error_handler = Some(handler);
}

#[pyfunction]
#[pyo3(name = "get_error_handler")]
pub fn py_get_error_handler() -> Option<Py<PyAny>> {
    GLOBAL_VAR.lock().unwrap().error_handler.clone()
}

pub fn reload() {
    // check if need reload
    {
        let var = GLOBAL_VAR.lock().unwrap();
        if !var.need_reload {
            return;
        }
    }
    event!(Level::DEBUG, "Script start reload");
    let start_time = Instant::now();

    let kwargs = {
        let mut var = GLOBAL_VAR.lock().unwrap();
        let kwargs = var.reload_var.take().unwrap_or_default();
        event!(Level::DEBUG, "Reload kwargs: {:?}", kwargs.clone());
        kwargs
    };
    let players = {
        let pool = ENTITY_POOL.lock().unwrap();
        pool.get_all_players().clone()
    };

    event!(Level::DEBUG, "Reload players: {:?}", players.len());

    let capture_modules = {
        GLOBAL_VAR
            .lock()
            .unwrap()
            .capture_modules
            .clone()
            .unwrap_or_default()
    };

    Python::with_gil(|py| {
        event!(Level::DEBUG, "Callback manager trigger player disconnect");
        for player in players.clone() {
            let _ = PY_CALLBACK_MANAGER.trigger(
                py,
                PyVcmpEvent::from(VcmpEvent::PlayerDisconnect(PlayerDisconnectEvent::new(
                    player, 1,
                )))
                .with_kwargs(kwargs.clone()),
            );
            {
                let mut pool = ENTITY_POOL.lock().unwrap();
                let player = pool.get_mut_player(player.get_id()).unwrap();
                player.set_var_reload_joined(false);
            }
        }

        event!(Level::DEBUG, "Callback manager trigger server shutdown");
        let _ = PY_CALLBACK_MANAGER.trigger(
            py,
            PyVcmpEvent::from(VcmpEvent::ServerShutdown(ServerShutdownEvent::default()))
                .with_kwargs(kwargs.clone()),
        );

        py.allow_threads(|| {
            let count = PY_CALLBACK_STORAGE.lock().unwrap().clear();
            event!(Level::DEBUG, "Cleared {count} callback(s)");
        });

        event!(Level::DEBUG, "Unload modules");
        {
            // 删 python 加载的模块
            // Copy 一份新的 IGNORE_MODULES 防止修改原数组
            let modules = IGNORE_MODULES
                .clone()
                .into_iter()
                .chain(capture_modules)
                .collect::<Vec<_>>();
            let py_sys_modules = py.import("sys").unwrap().getattr("modules").unwrap();
            let py_modules_unbind = py_sys_modules.extract::<Py<PyDict>>().unwrap();
            let py_modules = py_modules_unbind.bind(py);
            for m in py_modules.keys() {
                let m = m.extract::<String>().unwrap();
                if modules.contains(&m) || (&m).starts_with("vcmp") {
                    continue;
                }
                let _ = py_modules.del_item(m);
            }
        }

        event!(Level::DEBUG, "Reload script");
        py.allow_threads(|| {
            load_script();
        });
        event!(Level::DEBUG, "Reload script done");

        event!(Level::DEBUG, "Callback manager trigger server init");
        let _ = PY_CALLBACK_MANAGER.trigger(
            py,
            PyVcmpEvent::from(VcmpEvent::ServerInitialise(ServerInitialiseEvent::default()))
                .with_kwargs(kwargs.clone()),
        );

        // 重新获取玩家，防止玩家断开连接后，玩家列表为空
        let players = {
            let pool = ENTITY_POOL.lock().unwrap();
            pool.get_all_players().clone()
        };

        event!(Level::DEBUG, "Callback manager trigger player join");
        for player in players.clone() {
            {
                let mut pool = ENTITY_POOL.lock().unwrap();
                let player = pool.get_mut_player(player.get_id()).unwrap();
                player.set_var_reload_joined(true);
            }
            let _ = PY_CALLBACK_MANAGER.trigger(
                py,
                PyVcmpEvent::from(VcmpEvent::PlayerConnect(PlayerConnectEvent::new(player)))
                    .with_kwargs(kwargs.clone()),
            );
            if !player.get_var_loaded() {
                continue;
            }
            let _ = PY_CALLBACK_MANAGER.trigger(
                py,
                PyVcmpEvent::from(VcmpEvent::PlayerRequestClass(PlayerRequestClassEvent::new(
                    player,
                    player.get_class_id(py),
                )))
                .with_kwargs(kwargs.clone()),
            );
            if player.get_spawned() {
                let _ = PY_CALLBACK_MANAGER.trigger(
                    py,
                    PyVcmpEvent::from(VcmpEvent::PlayerSpawn(PlayerSpawnEvent::new(player)))
                        .with_kwargs(kwargs.clone()),
                );
            }
        }

        event!(Level::DEBUG, "Callback manager trigger server reloaded");
        let _ = PY_CALLBACK_MANAGER.trigger(
            py,
            PyVcmpEvent::from(VcmpEvent::ServerReloaded(ServerReloadedEvent::new(
                start_time.elapsed().as_secs_f64(),
            )))
            .with_kwargs(kwargs.clone()),
        );
    });

    event!(Level::INFO, "Script reloaded");

    {
        let mut var = GLOBAL_VAR.lock().unwrap();
        var.need_reload = false;
    }
}

pub fn load_script() {
    let script_path = CONFIG.get().unwrap().script_path.as_str();
    let script = Path::new(script_path);

    if !script.exists() {
        event!(Level::ERROR, "Script file not found: {}", script_path);
        return;
    }
    let code = CString::new(std::fs::read_to_string(script).unwrap_or_default())
        .expect("faild to create c string for code");
    let c_path = CString::new(script.to_str().unwrap_or_default())
        .expect("faild to create c string for path");
    Python::with_gil(|py| {
        let res = PyModule::from_code(
            py,
            &code,
            &c_path,
            &CString::new("__main__").expect("faild to create c string for module name"),
        );
        if let Err(e) = res {
            event!(
                Level::ERROR,
                "Failed to load script: {}",
                get_traceback(&e, Some(py))
            );
        } else {
            event!(Level::INFO, "Script loaded: {}", script_path);
        }
    });
}
