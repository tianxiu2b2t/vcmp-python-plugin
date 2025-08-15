use std::collections::HashMap;

use pyo3::{
    Bound, Py, PyAny, PyResult, Python, pyclass, pymethods,
    types::{PyModule, PyModuleMethods},
};

#[derive(Debug, Clone, Default)]
#[pyclass(subclass)]
#[pyo3(name = "Event")]
pub struct BaseEvent {
    kwargs: HashMap<String, Py<PyAny>>,
}

impl BaseEvent {
    pub fn kwargs(&mut self, kwargs: HashMap<String, Py<PyAny>>) -> Self {
        self.kwargs = kwargs;
        self.clone()
    }
}

#[pymethods]
impl BaseEvent {
    #[getter]
    pub fn get_kwargs(&self) -> &HashMap<String, Py<PyAny>> {
        &self.kwargs
    }

    #[setter]
    pub fn set_kwargs(&mut self, kwargs: HashMap<String, Py<PyAny>>) {
        self.kwargs = kwargs;
    }
}

pub trait PyEvent {
    fn event_name(&self) -> String;
    fn init(&self, py: Python<'_>) -> Py<PyAny>;
}

impl PyEvent for BaseEvent {
    fn event_name(&self) -> String {
        "BaseEvent".to_string()
    }

    fn init(&self, py: Python<'_>) -> Py<PyAny> {
        Py::new(py, self.clone()).expect("Failed to create event").into_any()
    }
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<BaseEvent>()?;
    Ok(())
}
