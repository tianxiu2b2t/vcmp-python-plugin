use std::{
    collections::HashMap,
    default::Default,
    sync::{LazyLock, Mutex},
};

use pyo3::{
    exceptions::PyKeyboardInterrupt,
    prelude::*,
    types::{PyCFunction, PyNone},
};
use tracing::{Level, event};
use vcmp_bindings::{func::ServerMethods, vcmp_func};

use crate::py::{
    GLOBAL_VAR,
    events::{PyVcmpEvent, VcmpEvent, VcmpEventType, abc::PyEvent},
    get_traceback,
};

pub const DEFAULT_CALLBACK_PRIORITY: u16 = 65535;

#[derive(Debug, Clone)]
pub struct CallbackFunction {
    /// 万物皆可 Call
    /// ```python
    /// class A:
    ///     def __call__(self):
    ///         ...
    /// ```
    pub func: Py<PyAny>,
    /// 从 0 开始到最后的 65535
    pub priority: u16,
    pub tag: Option<String>,
}

#[derive(Default)]
pub struct PyCallbackStorage {
    pub callbacks: HashMap<VcmpEventType, Vec<CallbackFunction>>,
}

impl PyCallbackStorage {
    pub fn register_func(
        &mut self,
        event_type: VcmpEventType,
        func: Py<PyAny>,
        priority: u16,
        tag: Option<String>,
    ) {
        if !self.callbacks.contains_key(&event_type) {
            self.callbacks.insert(event_type.clone(), Vec::default());
        }
        // 根据优先级来排列
        // 最高的优先，最后的最后来执行

        let handlers = self.callbacks.get_mut(&event_type).unwrap();
        let mut i = 0;
        while i < handlers.len() {
            if handlers[i].priority > priority {
                // fuck CodeGeeX why 1 < 500 ?  [500, 1]
                handlers.insert(
                    i,
                    CallbackFunction {
                        func,
                        priority,
                        tag,
                    },
                );
                return;
            }
            i += 1;
        }
        handlers.push(CallbackFunction {
            func,
            priority,
            tag,
        });
    }

    pub fn get_handlers(&self, event_type: VcmpEventType) -> Option<&Vec<CallbackFunction>> {
        self.callbacks.get(&event_type)
    }

    pub fn clear(&mut self) -> usize {
        let count = self.callbacks.iter().map(|(_, v)| v.len()).sum::<usize>();
        self.callbacks.clear();
        count
    }

    pub fn size(&self) -> usize {
        self.callbacks.iter().map(|(_, v)| v.len()).sum::<usize>()
    }

    pub fn get_handlers_by_tag(
        &self,
        event_type: &VcmpEventType,
        tag: Option<String>,
    ) -> Vec<&CallbackFunction> {
        let mut handlers = Vec::new();
        if let Some(callback_handlers) = self.callbacks.get(&event_type) {
            for handler in callback_handlers {
                if handler.tag == tag {
                    handlers.push(handler);
                }
            }
        }
        // sort by priority
        // 1 is higher
        // 65535 is lower
        handlers.sort_by_key(|handler| handler.priority);
        handlers
    }
}

/// 全局 callback 存储
pub static PY_CALLBACK_STORAGE: LazyLock<Mutex<PyCallbackStorage>> =
    LazyLock::new(|| Mutex::new(PyCallbackStorage::default()));

#[derive(Clone, Default, Debug, Copy)]
#[pyclass]
#[pyo3(name = "CallbackManager")]
pub struct PyCallbackManager {}

impl PyCallbackManager {
    pub fn handle(&self, event: VcmpEvent, abortable: bool) -> bool {
        event!(Level::TRACE, "Handling event: {:?}", event);
        let event_id = callback_utils::increase_event_id();
        event!(
            Level::TRACE,
            "Python with gil before counter: {:?}(ID: {event_id})",
            callback_utils::PY_GIL_REF_COUNTER.current()
        );

        let res = Python::with_gil(|py| {
            event!(
                Level::TRACE,
                "Python with gil after counter: {:?}(ID: {event_id})",
                callback_utils::PY_GIL_REF_COUNTER.increase()
            );
            match self.py_handle(py, PyVcmpEvent::from(event)) {
                Ok(res) => {
                    if res.is_none(py) {
                        abortable
                    } else {
                        match res.extract::<bool>(py) {
                            Ok(res) => res,
                            Err(_) => abortable,
                        }
                    }
                }
                Err(_) => abortable,
            };
            res
        });
        event!(
            Level::TRACE,
            "Python with gil after counter: {:?}(ID: {event_id})",
            callback_utils::PY_GIL_REF_COUNTER.decrease()
        );
        res
    }

    fn py_handle(&self, py: Python<'_>, event: PyVcmpEvent) -> PyResult<Py<PyAny>> {
        if let Err(e) = py.check_signals() {
            // 正常都是 Error 的
            event!(
                Level::DEBUG,
                "Failed to check signals: {}",
                get_traceback(&e, Some(py))
            );
            if e.is_instance_of::<PyKeyboardInterrupt>(py) {
                vcmp_func().shutdown();
            }
        }

        let kwargs = event.kwargs;
        let event_type = VcmpEventType::from(event.event_type.clone());
        let event = event.event_type;
        let handlers = {
            let storage = PY_CALLBACK_STORAGE.lock().unwrap();
            storage.get_handlers(event_type).cloned()
        };
        if handlers.is_none() {
            return Ok(PyNone::get(py)
                .downcast::<PyAny>()
                .unwrap()
                .clone()
                .unbind());
        }
        let handlers = handlers.unwrap();

        let mut result = None;

        let py_event = self.get_py_event(py, event.clone());

        // convert BaseEvent to set_kwargs and convert origin event
        let _ = py_event.setattr(py, "kwargs", kwargs);

        for handler in handlers {
            let func = handler.func.clone();
            match func.call1(py, (py_event.clone(),)) {
                Ok(res) => {
                    if res.is_none(py) {
                        continue;
                    }
                    result = Some(res.clone());
                    if let Ok(res) = res.extract::<bool>(py) && res {
                            break;
                        }
                    }
                }
                Err(e) => {
                    if e.is_instance_of::<PyKeyboardInterrupt>(py) {
                        event!(
                            Level::DEBUG,
                            "Failed to call callback: {}",
                            get_traceback(&e, Some(py))
                        );
                        vcmp_func().shutdown();
                        break;
                    } else {
                        let error_handler = { GLOBAL_VAR.lock().unwrap().error_handler.clone() };
                        if let Some(error_handler) = error_handler {
                            match error_handler.call1(py, (e.clone_ref(py),)) {
                                Ok(_) => {}
                                Err(handler_err) => {
                                    event!(
                                        Level::ERROR,
                                        "Failed to call error handler: {}",
                                        get_traceback(&handler_err, Some(py))
                                    );
                                    event!(
                                        Level::ERROR,
                                        "Failed to call callback: {}",
                                        get_traceback(&e, Some(py))
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(result.unwrap_or(
            PyNone::get(py)
                .downcast::<PyAny>()
                .unwrap()
                .clone()
                .unbind(),
        ))
    }

    pub fn register_func(
        &self,
        py: Python<'_>,
        event_type: VcmpEventType,
        func: Option<Py<PyAny>>,
        priority: u16,
        tag: Option<String>,
    ) -> Py<PyAny> {
        if let Some(func) = func {
            PY_CALLBACK_STORAGE.lock().unwrap().register_func(
                event_type,
                func.clone(),
                priority,
                tag.clone(),
            );
            func
        } else {
            PyCFunction::new_closure(
                py,
                None,
                None,
                move |args, _kwargs| -> PyResult<Py<PyAny>> {
                    let func = args.get_item(0).unwrap().extract::<Py<PyAny>>().unwrap();
                    PY_CALLBACK_STORAGE.lock().unwrap().register_func(
                        event_type,
                        func.clone(),
                        priority,
                        tag.clone(),
                    );
                    Ok(func)
                },
            )
            .unwrap()
            .unbind()
            .extract::<Py<PyAny>>(py)
            .unwrap()
        }
    }

    pub fn get_py_event(&self, py: Python<'_>, event: VcmpEvent) -> Py<PyAny> {
        match event {
            VcmpEvent::ServerInitialise(event) => event.init(py),
            VcmpEvent::ServerShutdown(event) => event.init(py),
            VcmpEvent::ServerFrame(event) => event.init(py),
            VcmpEvent::ServerPerformanceReport(event) => event.init(py),
            VcmpEvent::ServerReloaded(event) => event.init(py),
            VcmpEvent::IncomingConnection(event) => event.init(py),
            VcmpEvent::ClientScriptData(event) => event.init(py),
            VcmpEvent::PlayerConnect(event) => event.init(py),
            VcmpEvent::PlayerDisconnect(event) => event.init(py),
            VcmpEvent::PlayerRequestClass(event) => event.init(py),
            VcmpEvent::PlayerSpawn(event) => event.init(py),
            VcmpEvent::PlayerRequestSpawn(event) => event.init(py),
            VcmpEvent::PlayerDeath(event) => event.init(py),
            VcmpEvent::PlayerUpdate(event) => event.init(py),
            VcmpEvent::PlayerRequestEnterVehicle(event) => event.init(py),
            VcmpEvent::PlayerEnterVehicle(event) => event.init(py),
            VcmpEvent::PlayerExitVehicle(event) => event.init(py),
            VcmpEvent::PlayerNameChange(event) => event.init(py),
            VcmpEvent::PlayerStateChange(event) => event.init(py),
            VcmpEvent::PlayerActionChange(event) => event.init(py),
            VcmpEvent::PlayerOnFireChange(event) => event.init(py),
            VcmpEvent::PlayerCrouchChange(event) => event.init(py),
            VcmpEvent::PlayerGameKeysChange(event) => event.init(py),
            VcmpEvent::PlayerBeginTyping(event) => event.init(py),
            VcmpEvent::PlayerEndTyping(event) => event.init(py),
            VcmpEvent::PlayerAwayChange(event) => event.init(py),
            VcmpEvent::PlayerMessage(event) => event.init(py),
            VcmpEvent::PlayerCommand(event) => event.init(py),
            VcmpEvent::PlayerPrivateMessage(event) => event.init(py),
            VcmpEvent::PlayerKeyBindDown(event) => event.init(py),
            VcmpEvent::PlayerKeyBindUp(event) => event.init(py),
            VcmpEvent::PlayerSpectate(event) => event.init(py),
            VcmpEvent::PlayerCrashReport(event) => event.init(py),
            VcmpEvent::PlayerModuleList(event) => event.init(py),
            VcmpEvent::PlayerHealthChange(event) => event.init(py),
            VcmpEvent::PlayerArmourChange(event) => event.init(py),
            VcmpEvent::PlayerWeaponChange(event) => event.init(py),
            VcmpEvent::PlayerAmmoChange(event) => event.init(py),
            VcmpEvent::PlayerMove(event) => event.init(py),
            VcmpEvent::PickupPickAttempt(event) => event.init(py),
            VcmpEvent::PickupPicked(event) => event.init(py),
            VcmpEvent::PickupRespawn(event) => event.init(py),
            VcmpEvent::CheckpointEntered(event) => event.init(py),
            VcmpEvent::CheckpointExited(event) => event.init(py),
            VcmpEvent::ObjectShot(event) => event.init(py),
            VcmpEvent::ObjectTouched(event) => event.init(py),
            VcmpEvent::VehicleExplode(event) => event.init(py),
            VcmpEvent::VehicleRespawn(event) => event.init(py),
            VcmpEvent::VehicleUpdate(event) => event.init(py),
            VcmpEvent::VehicleMove(event) => event.init(py),
            VcmpEvent::VehicleHealthChange(event) => event.init(py),
            VcmpEvent::Custom(event) => event.init(py),
        }
    }
}

#[pymethods]
impl PyCallbackManager {
    pub fn trigger(&self, py: Python<'_>, event: PyVcmpEvent) -> PyResult<Py<PyAny>> {
        self.py_handle(py, event)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_server_initialise(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::ServerInitialise, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_server_shutdown(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::ServerShutdown, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_server_performance_report(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(
            py,
            VcmpEventType::ServerPerformanceReport,
            func,
            priority,
            tag,
        )
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_server_frame(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::ServerFrame, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_server_reloaded(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::ServerReloaded, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_incoming_connection(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::IncomingConnection, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_client_script_data(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::ClientScriptData, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_connect(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerConnect, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_disconnect(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerDisconnect, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_request_class(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerRequestClass, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_spawn(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerSpawn, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_request_spawn(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerRequestSpawn, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_death(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerDeath, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_update(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerUpdate, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_request_enter_vehicle(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(
            py,
            VcmpEventType::PlayerRequestEnterVehicle,
            func,
            priority,
            tag,
        )
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_enter_vehicle(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerEnterVehicle, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_exit_vehicle(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerExitVehicle, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_name_change(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerNameChange, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_state_change(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerStateChange, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_action_change(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerActionChange, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_on_fire_change(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerOnFireChange, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_crouch_change(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerCrouchChange, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_game_keys_change(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerGameKeysChange, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_begin_typing(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerBeginTyping, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_end_typing(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerEndTyping, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_away_change(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerAwayChange, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_message(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerMessage, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_command(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerCommand, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_private_message(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerPrivateMessage, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_key_bind_down(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerKeyBindDown, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_key_bind_up(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerKeyBindUp, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_spectate(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerSpectate, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_crash_report(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerCrashReport, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_module_list(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerModuleList, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_health_change(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerHealthChange, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_armour_change(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerArmourChange, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_weapon_change(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerWeaponChange, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_ammo_change(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerAmmoChange, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_player_move(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PlayerMove, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_pickup_pick_attempt(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PickupPickAttempt, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_pickup_picked(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PickupPicked, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_pickup_respawn(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::PickupRespawn, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_checkpoint_entered(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::CheckpointEntered, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_checkpoint_exited(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::CheckpointExited, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_object_shot(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::ObjectShot, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_object_touched(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::ObjectTouched, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_vehicle_explode(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::VehicleExplode, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_vehicle_respawn(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::VehicleRespawn, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_vehicle_update(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::VehicleUpdate, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_vehicle_move(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::VehicleMove, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_vehicle_health_change(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::VehicleHealthChange, func, priority, tag)
    }

    #[pyo3(signature = (priority = DEFAULT_CALLBACK_PRIORITY, func = None, tag = None))]
    pub fn on_custom(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
        tag: Option<String>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::Custom, func, priority, tag)
    }

    #[pyo3(signature = (event_type, tag = None))]
    pub fn get_register_callbacks(
        &self,
        py: Python<'_>,
        event_type: VcmpEventType,
        tag: Option<String>,
    ) -> PyResult<Vec<Py<PyAny>>> {
        let storage = PY_CALLBACK_STORAGE.lock().unwrap();
        let handlers = storage.get_handlers_by_tag(&event_type, tag);
        Ok(handlers.into_iter().map(|h| h.func.clone_ref(py)).collect())
    }
}

/// 全局的 callback 管理器
pub static PY_CALLBACK_MANAGER: LazyLock<PyCallbackManager> =
    LazyLock::new(PyCallbackManager::default);

pub fn module_define(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCallbackManager>()?;
    m.add("callbacks", PY_CALLBACK_MANAGER.into_pyobject(py)?)?;
    m.add("DEFAULT_PRIORITY", DEFAULT_CALLBACK_PRIORITY)?;
    Ok(())
}

/// 为了调试callback的
pub mod callback_utils {
    use std::sync::{Arc, LazyLock, Mutex, atomic::AtomicUsize};
    #[derive(Debug, Clone, Default)]
    pub struct PyGILRefCounter {
        pub counter: Arc<Mutex<i32>>,
    }

    impl PyGILRefCounter {
        pub fn increase(&self) -> i32 {
            let mut counter = self.counter.lock().unwrap();
            *counter += 1;

            *counter
        }

        pub fn decrease(&self) -> i32 {
            let mut counter = self.counter.lock().unwrap();
            *counter -= 1;

            *counter
        }

        pub fn current(&self) -> i32 {
            let counter = self.counter.lock().unwrap();
            *counter
        }
    }

    pub static PY_GIL_REF_COUNTER: LazyLock<PyGILRefCounter> =
        LazyLock::new(|| PyGILRefCounter::default());

    pub static EVENT_ID: AtomicUsize = AtomicUsize::new(0);

    pub fn increase_event_id() -> usize {
        EVENT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
}
