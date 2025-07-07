use std::{
    default::Default, sync::{Arc, LazyLock, Mutex}, thread
};

use pyo3::{
    PyClass,
    exceptions::PyKeyboardInterrupt,
    prelude::*,
    types::{PyBool, PyCFunction, PyDict, PyFunction, PyTuple, PyType},
};
use tracing::{Level, event};
use vcmp_bindings::{func::ServerMethods, vcmp_func};

use crate::py::{exceptions::FinishedException, get_traceback};

#[derive(Debug, Clone)]
pub struct CallbackFunctionParameter {
    pub name: String,
    pub annotations: Vec<Py<PyType>>,
    pub required: bool,
    pub default: Option<Py<PyAny>>,
}

#[derive(Debug, Clone)]
pub struct CallbackFunction {
    pub func: Py<PyFunction>,
    pub priority: i32,
    pub params: Vec<CallbackFunctionParameter>,
}

#[pyclass]
#[pyo3(name = "Matcher")]
#[derive(Debug, Clone, Copy)]
pub struct Matcher {
    pub is_finished: bool,
    pub result: bool,
}

impl Default for Matcher {
    fn default() -> Self {
        Self {
            is_finished: false,
            result: true, // default
        }
    }
}

#[pymethods]
impl Matcher {
    pub fn finish(&mut self, py: Python<'_>, result: Option<Py<PyBool>>) -> PyResult<()> {
        // if result is None, then None
        // if result is not None, then convert to bool
        let result = result.unwrap().extract::<bool>(py).unwrap();
        self.is_finished = true;
        self.result = result;

        // 然后阻止 python 代码继续运行
        Err(PyErr::new::<FinishedException, _>("Finished exception"))
    }

    #[getter]
    pub fn is_finished(&self) -> bool {
        self.is_finished
    }

    pub fn cancel(&mut self, py: Python<'_>) -> PyResult<()> {
        self.finish(py, Some(PyBool::new(py, false).into()))
    }
}


pub fn increase_event_id() -> u32 {
    EVENT_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

#[pyclass]
#[pyo3(name = "CallbackManager")]
#[derive(Debug, Clone, Copy, Default)]
pub struct CallbackManager;

impl CallbackManager {
    pub fn call_func<T>(&self, event: T, _kwargs: Option<Py<PyDict>>) -> bool
    where
        T: PyClass + crate::py::events::PyBaseEvent,
    {
        let current_id = increase_event_id();
        let thread_id = thread::current().id();
        event!(
            Level::INFO,
            "Calling callbacks for event: (Thread({thread_id:?}) {current_id}) {:?}",
            event
        );

        let callbacks = CALLBACKS_STORE.lock().unwrap();
        let mut matcher = Matcher::default();
        Python::with_gil(|py| {
            let py_matcher = Py::new(py, matcher).unwrap();
            let instance = event.init(py).expect("Failed to initialize event");
            for callback in callbacks.iter() {
                event!(Level::INFO, "Matching callback: {:?}", callback);
                let origin_parameters = callback.params.clone();
                let py_kwargs = PyDict::new(py);

                let mut matched = true;

                for param in origin_parameters {
                    let name = param.name.clone();
                    for annotation in param.annotations {
                        let annotation = annotation.bind(py);
                        if instance.bind(py).is_instance(annotation).unwrap() {
                            py_kwargs.set_item(name.clone(), instance.clone()).unwrap();
                            break;
                        }
                        if py_matcher.bind(py).is_instance(annotation).unwrap() {
                            py_kwargs
                                .set_item(name.clone(), py_matcher.borrow_mut(py))
                                .unwrap();
                            break;
                        }
                    }
                    
                    //    if arg.required and arg.name not in params:
                    //    matched = False
                    //    break
                    //elif arg.name not in params:
                    //    params[arg.name] = arg.default 
                    if param.required && !py_kwargs.contains(name.clone()).unwrap() {
                        matched = false;
                        break;
                    } else if !py_kwargs.contains(name.clone()).unwrap() {
                        py_kwargs
                            .set_item(name.clone(), param.default.clone())
                            .unwrap();
                    }
                }
                if !matched {
                    continue;
                }

                event!(Level::INFO, "Matched callback: {:?}", callback);

                match callback.func.call(py, (), Some(&py_kwargs)) {
                    Ok(res) => {
                        // isinstance bool
                        let result = res.bind(py);
                        if let Ok(r) = result.downcast::<PyBool>() {
                            matcher.is_finished = true;
                            matcher.result = r.extract::<bool>().unwrap();
                        } else if result.is_none() {
                            matcher.is_finished = true;
                            matcher.result = true;
                        }
                    }
                    Err(e) => {
                        if e.is_instance_of::<PyKeyboardInterrupt>(py) {
                            vcmp_func().shutdown();
                            break;
                        } else if e.is_instance_of::<FinishedException>(py) {
                            break;
                        } else {
                            event!(
                                Level::ERROR,
                                "Callback event: {event:?} error: {}",
                                get_traceback(e)
                            );
                        }
                    }
                };
                if matcher.is_finished {
                    break;
                }
            }

            if matcher.is_finished {
                // matcher = *matcher_ref;
            }
        });
        event!(
            Level::INFO,
            "Finished calling callbacks for event: (Thread({thread_id:?}) {current_id}) {:?}",
            event
        );
        matcher.result
    }
}

#[pymethods]
impl CallbackManager {
    //pub fn trigger(&self, py: Python<'_>, event: Py<BaseEvent>, kwargs: Option<Py<PyDict>>) -> bool {
    //    let event_bind = event.bind(py);
    //    let event_type = event_bind.get_type().;
    //
    //    false
    //}
    #[pyo3(signature = (priority = 9999))]
    pub fn on<'a>(
        &mut self,
        py: Python<'a>,
        priority: Option<i32>,
    ) -> PyResult<pyo3::Bound<'a, pyo3::types::PyCFunction>> {
        let priority = priority.unwrap_or(9999);
        // we need return a function that can be called with the arguments
        // and then call the callback with the arguments
        PyCFunction::new_closure(
            py,
            None,
            None,
            move |args, _kwargs| -> PyResult<Py<PyFunction>> {
                let func = args
                    .get_item(0)
                    .unwrap()
                    .extract::<Py<PyFunction>>()
                    .unwrap();
                let py_clone_func = func.clone();

                let parameters = get_function_parameters(func.clone());

                let callback = CallbackFunction {
                    func: py_clone_func,
                    priority,
                    params: parameters,
                };
                CALLBACKS_STORE.lock().unwrap().push(callback);
                Ok(func)
            },
        )
    }
}

fn get_function_parameters(func: Py<PyFunction>) -> Vec<CallbackFunctionParameter> {
    let mut params = vec![];
    Python::with_gil(|py| {
        let py_inspect_module =
            PyModule::import(py, "inspect").expect("Failed to import inspect module");
        let py_getfullargspec_func = py_inspect_module
            .getattr("getfullargspec")
            .expect("Failed to get getfullargspec function");
        let py_getfullargspec = py_getfullargspec_func
            .call1((func,))
            .expect("Failed to call getfullargspec function");
        let py_args = py_getfullargspec
            .getattr("args")
            .unwrap()
            .extract::<Vec<String>>()
            .unwrap();
        let py_defaults: Vec<Bound<'_, PyAny>> = {
            let binding = py_getfullargspec.getattr("defaults").unwrap();
            if binding.is_none() {
                vec![]
            } else {
                let mut arr = Vec::new();
                for ele in binding.downcast::<PyTuple>().unwrap() {
                    arr.push(ele);
                }
                arr
            }
        };
        let py_annontations = {
            let binding = py_getfullargspec.getattr("annotations").unwrap();
            let dict = binding.downcast::<PyDict>().unwrap();
            // 创建新的PyDict
            let result = PyDict::new(py);
            // 复制所有项
            for (k, v) in dict.iter() {
                result.set_item(k, v).unwrap();
            }
            result
        };
        let offset = py_args.len() - py_defaults.len();
        //let mut args = Vec::new();
        for (i, arg) in py_args.iter().enumerate() {
            let annotations = {
                let mut arr = Vec::new();
                let binding = py_annontations.get_item(arg).unwrap().unwrap();
                if !binding.is_none() {
                    if binding.hasattr("__origin__").unwrap()
                        && binding
                            .getattr("__origin__")
                            .unwrap()
                            .extract::<String>()
                            .unwrap()
                            == "Union"
                    {
                        let typing_module = PyModule::import(py, "typing").unwrap();
                        let typing_module_get_args = typing_module.getattr("get_args").unwrap();
                        let typing_module_get_args =
                            typing_module_get_args.call1((binding,)).unwrap();
                        let typing_module_get_args =
                            typing_module_get_args.downcast::<PyTuple>().unwrap();
                        for ele in typing_module_get_args {
                            arr.push(ele.downcast::<PyType>().unwrap().clone().unbind());
                        }
                    } else {
                        arr.push(binding.downcast::<PyType>().unwrap().clone().unbind());
                    }
                }
                arr
            };
            let required = i < offset;
            let default = if (i as i32) - (offset as i32) >= 0 {
                Some(py_defaults[i - offset].clone().unbind())
            } else {
                None
            };
            let param = CallbackFunctionParameter {
                name: arg.clone(),
                annotations: annotations.clone(),
                required,
                default,
            };
            params.push(param);
        }
        params
    })
}

static CALLBACKS_STORE: LazyLock<Arc<Mutex<Vec<CallbackFunction>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(Vec::new())));

pub static CALLBACK: LazyLock<CallbackManager> = LazyLock::new(CallbackManager::default);

static EVENT_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

pub fn module_define(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<CallbackManager>()?;
    m.add_class::<Matcher>()?;
    m.add("callbacks", CALLBACK.into_pyobject(py)?)?;
    Ok(())
}
