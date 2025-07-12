// finishedException

use pyo3::{Bound, PyResult, pymethods};
use pyo3::{Python, pyclass};

use pyo3::types::{PyModule, PyModuleMethods};
#[pyclass(extends = pyo3::exceptions::PyException)]
#[pyo3(name = "FinishedException")]
#[derive(Debug, Default)]
pub struct FinishedException;

#[pymethods]
impl FinishedException {
    #[new]
    fn new(_message: &str) -> Self {
        Self
    }
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FinishedException>()?;
    Ok(())
}
