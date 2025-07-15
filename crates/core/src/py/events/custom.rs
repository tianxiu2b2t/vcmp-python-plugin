use pyo3::{
    Bound, Py, PyAny, PyClassInitializer, PyErr, PyResult, Python, pyclass, pymethods,
    types::{PyModule, PyModuleMethods, PyTraceback},
};

use crate::py::events::abc::{BaseEvent, PyEvent};

#[derive(Debug, Clone, Default)]
#[pyclass(extends=BaseEvent, subclass)]
pub struct CustomEvent {}
impl CustomEvent {
    pub fn new() -> (Self, BaseEvent) {
        (Self {}, BaseEvent::default())
    }
}
impl PyEvent for CustomEvent {
    fn event_name(&self) -> String {
        "CustomEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(BaseEvent::default()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}

#[derive(Debug, Clone)]
#[pyclass(extends=CustomEvent, subclass)]
#[pyo3(name = "TracebackEvent")]
pub struct PyTracebackEvent {
    pub traceback: Py<PyTraceback>,
}
impl PyEvent for PyTracebackEvent {
    fn event_name(&self) -> String {
        "PyTracebackEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(
            py,
            PyClassInitializer::from(CustomEvent::new()).add_subclass(self.clone()),
        )
        .unwrap()
        .into_any()
    }
}
impl PyTracebackEvent {
    pub fn new(traceback: Py<PyTraceback>) -> Self {
        Self { traceback }
    }
    pub fn from_err(py: Option<Python<'_>>, err: Py<PyErr>) -> Self {
        match py {
            Some(py) => Self {
                traceback: err.extract::<Py<PyTraceback>>(py).unwrap(),
            },
            None => Python::with_gil(|py| Self {
                traceback: err.extract::<Py<PyTraceback>>(py).unwrap(),
            }),
        }
    }
}
#[pymethods]
impl PyTracebackEvent {
    #[getter]
    pub fn get_traceback(&self) -> Py<PyTraceback> {
        self.traceback.clone()
    }
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<CustomEvent>()?;
    m.add_class::<PyTracebackEvent>()?;
    Ok(())
}
