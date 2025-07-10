use std::{
    collections::HashMap,
    default::Default,
    sync::{LazyLock, Mutex},
};

use pyo3::{
    prelude::*,
    types::{PyCFunction, PyNone},
};
use tracing::event;
use vcmp_bindings::events::{VcmpEvent, VcmpEventType};

use crate::py::events::PyVcmpEvent;

#[derive(Debug, Clone)]
pub struct CallbackFunction {
    /// 万物皆可 Call
    /// ```python
    /// class A:
    ///     def __call__(self):
    ///         ...
    /// ```
    pub func: Py<PyAny>,
    /// 从 0 开始到最后的 9999
    pub priority: u16,
}

#[derive(Default)]
pub struct PyCallbackStorage {
    pub callbacks: HashMap<VcmpEventType, Vec<CallbackFunction>>,
}

impl PyCallbackStorage {
    pub fn register_func(&mut self, event_type: VcmpEventType, func: Py<PyAny>, priority: u16) {
        if !self.callbacks.contains_key(&event_type) {
            self.callbacks.insert(event_type.clone(), Vec::default());
        }
        self.callbacks
            .get_mut(&event_type)
            .unwrap()
            .push(CallbackFunction { func, priority });
    }

    pub fn get_handlers(&self, event_type: &VcmpEventType) -> Option<&Vec<CallbackFunction>> {
        self.callbacks.get(event_type)
    }
}

/// 全局 callback 存储
pub static PY_CALLBACK_STORAGE: LazyLock<Mutex<PyCallbackStorage>> =
    LazyLock::new(|| Mutex::new(PyCallbackStorage::default()));

#[derive(Clone, Default, Debug, Copy)]
#[pyclass]
pub struct PyCallbackManager {}

impl PyCallbackManager {
    pub fn handle(&self, event: PyVcmpEvent, abortable: bool) -> bool {
        Python::with_gil(|py| match self.py_handle(py, event) {
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
        })
    }
    fn py_handle(&self, py: Python<'_>, event: PyVcmpEvent) -> PyResult<Py<PyAny>> {
        let event_type = VcmpEventType::from(event.event_enum);
        let storage = PY_CALLBACK_STORAGE.lock().unwrap();
        let handlers = storage.get_handlers(&event_type);
        if handlers.is_none() {
            return Ok(PyNone::get(py)
                .downcast::<PyAny>()
                .unwrap()
                .clone()
                .unbind());
        }
        let handlers = handlers.unwrap();
        //let mut res = None;
        let need_arg = {
            match event_type {
                VcmpEventType::ServerInitialise => false,
                VcmpEventType::ServerShutdown => false,
                _ => true,
            }
        };
        let args = if need_arg {
            PyTuple::new(py, &[event.clone().into_py(py)])
        } else {
            PyTuple::new(py, &[])
        };
        for handler in handlers {
            let func = handler.func.clone();
            match func.call1(py, args) {
                Ok(res) => {
                    //event.abort = res.extract::<bool>(py).unwrap_or(false);
                }
                Err(e) => {
                    //event.abort = true;
                    //event.error = Some(e.to_string());
                }
            }
        }
        Ok(PyNone::get(py)
            .downcast::<PyAny>()
            .unwrap()
            .clone()
            .unbind())
    }

    pub fn register_func(
        &self,
        py: Python<'_>,
        event_type: VcmpEventType,
        func: Option<Py<PyAny>>,
        priority: u16,
    ) -> Py<PyAny> {
        if let Some(func) = func {
            PY_CALLBACK_STORAGE
                .lock()
                .unwrap()
                .register_func(event_type, func.clone(), priority);
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
}

#[pymethods]
impl PyCallbackManager {
    #[pyo3(signature = (priority = 9999, func = None))]
    pub fn on_server_initialise(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::ServerInitialise, func, priority)
    }

    #[pyo3(signature = (priority = 9999, func = None))]
    pub fn on_server_shutdown(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::ServerShutdown, func, priority)
    }

    #[pyo3(signature = (priority = 9999, func = None))]
    pub fn on_server_performance_report(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::ServerPerformanceReport, func, priority)
    }
    
    #[pyo3(signature = (priority = 9999, func = None))]
    pub fn on_server_frame(
        &self,
        py: Python<'_>,
        priority: u16,
        func: Option<Py<PyAny>>,
    ) -> Py<PyAny> {
        self.register_func(py, VcmpEventType::ServerFrame, func, priority)
    }

}

/// 全局的 callback 管理器

pub static PY_CALLBACK_MANAGER: LazyLock<PyCallbackManager> =
    LazyLock::new(|| PyCallbackManager::default());

pub fn module_define(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCallbackManager>()?;
    m.add("callbacks", PY_CALLBACK_MANAGER.into_pyobject(py)?)?;
    Ok(())
}

// #[derive(Debug, Clone)]
// pub struct CallbackFunctionParameter {
//     pub name: String,
//     pub annotations: Vec<Py<PyType>>,
//     pub required: bool,
//     pub default: Option<Py<PyAny>>,
// }
// #[pyclass]
// #[pyo3(name = "Matcher")]
// #[derive(Debug, Clone, Copy)]
// pub struct Matcher {
//     pub is_finished: bool,
//     pub result: bool,
// }

// impl Default for Matcher {
//     fn default() -> Self {
//         Self {
//             is_finished: false,
//             result: true, // default
//         }
//     }
// }

// #[pymethods]
// impl Matcher {
//     #[pyo3(signature = (result = None))]
//     pub fn finish(&mut self, py: Python<'_>, result: Option<Py<PyBool>>) -> PyResult<()> {
//         // if result is None, then None
//         // if result is not None, then convert to bool
//         let res = {
//             match result {
//                 Some(result) => result.extract::<bool>(py).unwrap(),
//                 None => true,
//             }
//         };
//         self.is_finished = true;
//         self.result = res;

//         // 然后阻止 python 代码继续运行
//         Err(PyErr::new::<FinishedException, _>("Finished exception"))
//     }

//     #[getter]
//     pub fn is_finished(&self) -> bool {
//         self.is_finished
//     }

//     pub fn cancel(&mut self, py: Python<'_>) -> PyResult<()> {
//         self.finish(py, Some(PyBool::new(py, false).into()))
//     }
// }

// pub fn increase_event_id() -> u32 {
//     EVENT_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
// }

// #[derive(Default)]
// pub struct EventCallRefCounter {
//     pub counter: Arc<Mutex<i32>>,
// }

// impl EventCallRefCounter {
//     pub fn increase(&self) -> i32 {
//         let mut counter = self.counter.lock().unwrap();
//         *counter += 1;

//         *counter
//     }

//     pub fn decrease(&self) -> i32 {
//         let mut counter = self.counter.lock().unwrap();
//         *counter -= 1;

//         *counter
//     }

//     pub fn current(&self) -> i32 {
//         let counter = self.counter.lock().unwrap();
//         *counter
//     }
// }

// pub static IS_CALLING: LazyLock<EventCallRefCounter> = LazyLock::new(EventCallRefCounter::default);

// #[pyclass]
// #[pyo3(name = "CallbackManager")]
// #[derive(Debug, Clone, Copy, Default)]
// pub struct CallbackManager;

// impl CallbackManager {
//     pub fn call_func<T>(&self, event: T, _kwargs: Option<Py<PyDict>>, failed_result: bool) -> bool
//     where
//         T: PyClass + crate::py::events::PyBaseEvent + Clone + std::fmt::Debug,
//     {
//         let current_id = increase_event_id();
//         let current_ref = IS_CALLING.increase();
//         if current_ref >= 2 {
//             event!(
//                 Level::ERROR,
//                 "Too many callbacks are calling, current_ref: {current_ref}, {event:?}"
//             );
//             IS_CALLING.decrease();
//             return failed_result;
//         }
//         event!(
//             Level::DEBUG,
//             "Calling callbacks for event: ({current_id}, current_ref: {current_ref}) {:?}",
//             event
//         );

//         let callbacks = CALLBACKS_STORE.lock().unwrap();
//         let mut matcher = Matcher::default();
//         Python::with_gil(|py| {
//             let py_matcher = Py::new(py, matcher).unwrap();
//             let instance = event.init(py).expect("Failed to initialize event");
//             for callback in callbacks.iter() {
//                 //event!(Level::DEBUG, "Matching callback: {:?}", callback);
//                 let origin_parameters = callback.params.clone();
//                 let py_kwargs = PyDict::new(py);

//                 let mut matched = true;

//                 for param in origin_parameters {
//                     let name = param.name.clone();
//                     for annotation in param.annotations {
//                         let annotation = annotation.bind(py);
//                         if instance.bind(py).is_instance(annotation).unwrap() {
//                             py_kwargs.set_item(name.clone(), instance.clone()).unwrap();
//                             break;
//                         }
//                         if py_matcher.bind(py).is_instance(annotation).unwrap() {
//                             py_kwargs
//                                 .set_item(name.clone(), py_matcher.borrow_mut(py))
//                                 .unwrap();
//                             break;
//                         }
//                     }

//                     if param.required && !py_kwargs.contains(name.clone()).unwrap() {
//                         matched = false;
//                         break;
//                     } else if !py_kwargs.contains(name.clone()).unwrap() {
//                         py_kwargs
//                             .set_item(name.clone(), param.default.clone())
//                             .unwrap();
//                     }
//                 }
//                 if !matched {
//                     continue;
//                 }

//                 event!(Level::DEBUG, "Matched callback: {:?} {:?}", callback, event);

//                 match callback.func.call(py, (), Some(&py_kwargs)) {
//                     Ok(res) => {
//                         event!(Level::DEBUG, "Callback result: {:?}", &res);
//                         // isinstance bool
//                         let result = res.bind(py);
//                         if let Ok(r) = result.downcast::<PyBool>() {
//                             matcher.is_finished = true;
//                             matcher.result = r.extract::<bool>().unwrap();
//                         }
//                     }
//                     Err(e) => {
//                         let traceback = get_traceback(&e, Some(py));
//                         if e.is_instance_of::<PyKeyboardInterrupt>(py) {
//                             event!(Level::DEBUG, traceback);
//                             event!(Level::DEBUG, "KeyboardInterrupt");
//                             vcmp_func().shutdown();
//                             break;
//                         }
//                         event!(
//                             Level::ERROR,
//                             "Callback event: {event:?} error: {}",
//                             traceback
//                         );
//                         //if e.is_instance_of::<PyKeyboardInterrupt>(py) {
//                         //    vcmp_func().shutdown();
//                         //    break;
//                         //} else if e.is_instance_of::<FinishedException>(py) {
//                         //    break;
//                         //} else {
//                         //    //event!(
//                         //    //    Level::ERROR,
//                         //    //    "Callback event: {event:?} error: {}",
//                         //    //    get_traceback(e, Some(py))
//                         //    //);
//                         //}
//                     }
//                 };
//                 if matcher.is_finished {
//                     break;
//                 }
//             }

//             if let Err(e) = py.check_signals() {
//                 let traceback = get_traceback(&e, Some(py));
//                 event!(Level::DEBUG, traceback);
//                 vcmp_func().shutdown();
//             }
//         });
//         event!(
//             Level::DEBUG,
//             "Finished calling callbacks for event: ({current_id}, current_ref: {current_ref}, global_ref: {}) {:?}",
//             IS_CALLING.current(),
//             event
//         );
//         IS_CALLING.decrease();
//         matcher.result
//     }
// }

// #[pymethods]
// impl CallbackManager {
//     //pub fn trigger(&self, py: Python<'_>, event: Py<BaseEvent>, kwargs: Option<Py<PyDict>>) -> bool {
//     //    let event_bind = event.bind(py);
//     //    let event_type = event_bind.get_type().;
//     //
//     //    false
//     //}
//     #[pyo3(signature = (priority = 9999))]
//     pub fn on<'a>(
//         &mut self,
//         py: Python<'a>,
//         priority: Option<i32>,
//     ) -> PyResult<pyo3::Bound<'a, pyo3::types::PyCFunction>> {
//         let priority = priority.unwrap_or(9999);
//         // we need return a function that can be called with the arguments
//         // and then call the callback with the arguments
//         PyCFunction::new_closure(
//             py,
//             None,
//             None,
//             move |args, _kwargs| -> PyResult<Py<PyFunction>> {
//                 let func = args
//                     .get_item(0)
//                     .unwrap()
//                     .extract::<Py<PyFunction>>()
//                     .unwrap();
//                 let py_clone_func = func.clone();

//                 let parameters = get_function_parameters(func.clone());

//                 let callback = CallbackFunction {
//                     func: py_clone_func,
//                     priority,
//                     params: parameters,
//                 };
//                 CALLBACKS_STORE.lock().unwrap().push(callback);
//                 Ok(func)
//             },
//         )
//     }
// }

// fn get_function_parameters(func: Py<PyFunction>) -> Vec<CallbackFunctionParameter> {
//     let mut params = vec![];
//     Python::with_gil(|py| {
//         let py_inspect_module =
//             PyModule::import(py, "inspect").expect("Failed to import inspect module");
//         let py_getfullargspec_func = py_inspect_module
//             .getattr("getfullargspec")
//             .expect("Failed to get getfullargspec function");
//         let py_getfullargspec = py_getfullargspec_func
//             .call1((func,))
//             .expect("Failed to call getfullargspec function");
//         let py_args = py_getfullargspec
//             .getattr("args")
//             .unwrap()
//             .extract::<Vec<String>>()
//             .unwrap();
//         let py_defaults: Vec<Bound<'_, PyAny>> = {
//             let binding = py_getfullargspec.getattr("defaults").unwrap();
//             if binding.is_none() {
//                 vec![]
//             } else {
//                 let mut arr = Vec::new();
//                 for ele in binding.downcast::<PyTuple>().unwrap() {
//                     arr.push(ele);
//                 }
//                 arr
//             }
//         };
//         let py_annontations = {
//             let binding = py_getfullargspec.getattr("annotations").unwrap();
//             let dict = binding.downcast::<PyDict>().unwrap();
//             // 创建新的PyDict
//             let result = PyDict::new(py);
//             // 复制所有项
//             for (k, v) in dict.iter() {
//                 result.set_item(k, v).unwrap();
//             }
//             result
//         };
//         let offset = py_args.len() - py_defaults.len();
//         //let mut args = Vec::new();
//         for (i, arg) in py_args.iter().enumerate() {
//             let annotations = {
//                 let mut arr = Vec::new();
//                 let binding = py_annontations.get_item(arg).unwrap().unwrap();
//                 if !binding.is_none() {
//                     if binding.hasattr("__origin__").unwrap()
//                         && binding
//                             .getattr("__origin__")
//                             .unwrap()
//                             .extract::<String>()
//                             .unwrap()
//                             == "Union"
//                     {
//                         let typing_module = PyModule::import(py, "typing").unwrap();
//                         let typing_module_get_args = typing_module.getattr("get_args").unwrap();
//                         let typing_module_get_args =
//                             typing_module_get_args.call1((binding,)).unwrap();
//                         let typing_module_get_args =
//                             typing_module_get_args.downcast::<PyTuple>().unwrap();
//                         for ele in typing_module_get_args {
//                             arr.push(ele.downcast::<PyType>().unwrap().clone().unbind());
//                         }
//                     } else {
//                         arr.push(binding.downcast::<PyType>().unwrap().clone().unbind());
//                     }
//                 }
//                 arr
//             };
//             let required = i < offset;
//             let default = if (i as i32) - (offset as i32) >= 0 {
//                 Some(py_defaults[i - offset].clone().unbind())
//             } else {
//                 None
//             };
//             let param = CallbackFunctionParameter {
//                 name: arg.clone(),
//                 annotations: annotations.clone(),
//                 required,
//                 default,
//             };
//             params.push(param);
//         }
//         params
//     })
// }

// static CALLBACKS_STORE: LazyLock<Arc<Mutex<Vec<CallbackFunction>>>> =
//     LazyLock::new(|| Arc::new(Mutex::new(Vec::new())));

// pub static CALLBACK: LazyLock<CallbackManager> = LazyLock::new(CallbackManager::default);

// static EVENT_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

// pub fn module_define(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_class::<CallbackManager>()?;
//     m.add_class::<Matcher>()?;
//     m.add("callbacks", CALLBACK.into_pyobject(py)?)?;
//     Ok(())
// }

// // ===

// pub mod event_helper {
//     use std::sync::{Arc, Mutex, atomic::AtomicU64};

//     static EVENT_COUNTER: AtomicU64 = AtomicU64::new(0);

//     #[derive(Default)]
//     pub struct EventCallRefCounter {
//         pub counter: Arc<Mutex<i32>>,
//     }

//     impl EventCallRefCounter {
//         pub fn increase(&self) -> i32 {
//             let mut counter = self.counter.lock().unwrap();
//             *counter += 1;

//             *counter
//         }

//         pub fn decrease(&self) -> i32 {
//             let mut counter = self.counter.lock().unwrap();
//             *counter -= 1;

//             *counter
//         }

//         pub fn current(&self) -> i32 {
//             let counter = self.counter.lock().unwrap();
//             *counter
//         }
//     }
// }
