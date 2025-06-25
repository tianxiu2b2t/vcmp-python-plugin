/*use pyo3::types::PyCFunction;
use pyo3::{pymethods, pyclass, types::PyFunction, Py, PyAny};
use pyo3::prelude::*;


pub struct CallbackParameter {
    pub name: String,
    pub annontations: Vec<Py<PyAny>>,
    pub default: Option<PyAny>,
    pub required: bool,
}

pub struct CallbackFunction {
    pub func: Py<PyFunction>,
    pub args: Vec<CallbackParameter>,
    pub poriority: i32,
}

#[pyclass]
#[derive(Debug, Clone, Copy)]
pub struct CallbackManager {
    callbacks: Vec<CallbackFunction>
}

impl CallbackManager {
    pub fn new() -> Self {
        Self {
            callbacks: Vec::new()
        }
    }
}

#[pymethods]
impl CallbackManager {

}*/


use std::{convert::Infallible, sync::LazyLock};

use pyo3::{prelude::*, types::{PyCFunction, PyFunction}};
use tracing::{event, Level};

#[derive(Debug, Clone)]
pub struct CallbackFunction<'py> {
    pub func: Bound<'py, Py<PyAny>>,
    pub priority: i32,
}

#[pyclass]
#[pyo3(name = "CallbackManager")]
#[derive(Debug, Clone)]
pub struct CallbackManager<'py> {
    pub callbacks: Vec<CallbackFunction<'py>>
}
impl CallbackManager {
    pub fn new() -> Self {
        Self {
            callbacks: Vec::new()
        }
    }
}

#[pymethods]
impl CallbackManager {
    pub fn on<'a>(&self, py: Python<'a>, priority: Option<i32>) -> PyResult<pyo3::Bound<'a, pyo3::types::PyCFunction>> {
        let priority = priority.unwrap_or(9999);
        event!(Level::DEBUG, "CallbackManager.on called with priority: {priority}");
        // we need return a function that can be called with the arguments
        // and then call the callback with the arguments
        PyCFunction::new_closure(py, None, None, move |args, _kwargs| -> PyResult<Py<PyAny>> {
            let func = args.get_item(0).unwrap().extract::<Py<PyAny>>().unwrap();
            event!(Level::DEBUG, "CallbackManager.on called with function: {func:?}");
            Ok(func)
        })
    }

}

pub static CALLBACK: LazyLock<CallbackManager> = LazyLock::new(CallbackManager::new);

pub fn module_define(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<CallbackManager>()?;
    m.add("callbacks", CALLBACK.into_pyobject(py)?)?;
    Ok(())
}
