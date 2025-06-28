use std::{
    default::Default,
    sync::{Arc, LazyLock, Mutex},
};

use pyo3::{
    PyClass,
    prelude::*,
    types::{PyCFunction, PyDict, PyFunction},
};
use tracing::{Level, event};

#[derive(Debug, Clone)]
pub struct CallbackFunction {
    pub func: Py<PyFunction>,
    pub priority: i32,
}

#[pyclass]
#[pyo3(name = "CallbackManager")]
#[derive(Debug, Clone, Copy, Default)]
pub struct CallbackManager;

impl CallbackManager {
    pub fn call_func<T>(&self, event: T)
    where
        T: PyClass + crate::py::events::PyBaseEvent,
    {
        event!(
            Level::DEBUG,
            "CallbackManager.call_func called with event: {event:?}"
        );
        let callbacks = CALLBACKS_STORE.lock().unwrap();
        Python::with_gil(|py| {
            let instance = event.init(py).expect("Failed to initialize event");
            for callback in callbacks.iter() {
                let res = callback.func.call1(py, (instance.clone(),)).unwrap();
                event!(
                    Level::DEBUG,
                    "CallbackManager.call_func called with callback: {callback:?}, result: {res:?}"
                )
            }
        });
    }
}

#[pymethods]
impl CallbackManager {
    pub fn on<'a>(
        &mut self,
        py: Python<'a>,
        priority: Option<i32>,
    ) -> PyResult<pyo3::Bound<'a, pyo3::types::PyCFunction>> {
        let priority = priority.unwrap_or(9999);
        event!(
            Level::DEBUG,
            "CallbackManager.on called with priority: {priority}"
        );
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
                event!(
                    Level::DEBUG,
                    "CallbackManager.on called with function: {func:?}"
                );
                let py_clone_func = func.clone();

                // print
                let py_getfullargspec = get_annontations(func.clone());
                event!(
                    Level::DEBUG,
                    "CallbackManager.on called with function args: {py_getfullargspec:?}"
                );

                let callback = CallbackFunction {
                    func: py_clone_func,
                    priority,
                };
                CALLBACKS_STORE.lock().unwrap().push(callback);
                Ok(func)
            },
        )
    }
}

fn get_annontations(func: Py<PyFunction>) -> Py<PyDict> {
    Python::with_gil(|py| {
        let py_inspect_module =
            PyModule::import(py, "inspect").expect("Failed to import inspect module");
        let py_getfullargspec_func = py_inspect_module
            .getattr("getfullargspec")
            .expect("Failed to get getfullargspec function");
        let py_getfullargspec = py_getfullargspec_func
            .call1((func,))
            .expect("Failed to call getfullargspec function");
        event!(
            Level::DEBUG,
            "CallbackManager.on called with function args: {py_getfullargspec:?}"
        );
        PyDict::new(py).unbind()
    })
}

static CALLBACKS_STORE: LazyLock<Arc<Mutex<Vec<CallbackFunction>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(Vec::new())));

pub static CALLBACK: LazyLock<CallbackManager> = LazyLock::new(CallbackManager::default);

pub fn module_define(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<CallbackManager>()?;
    m.add("callbacks", CALLBACK.into_pyobject(py)?)?;
    Ok(())
}
