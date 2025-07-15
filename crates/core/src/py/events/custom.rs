use pyo3::{
    Bound, Py, PyAny, PyClassInitializer, PyResult, Python, pyclass,
    types::{PyModule, PyModuleMethods},
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


pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<CustomEvent>()?;
    Ok(())
}
