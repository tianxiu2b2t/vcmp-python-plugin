// finishedException

use pyo3::{Bound, PyResult};
use pyo3::{Python, pyclass};

use pyo3::types::{PyModule, PyModuleMethods};
#[pyclass(extends = pyo3::exceptions::PyException)]
#[pyo3(name = "FinishedException")]
#[derive(Debug, Default)]
pub struct FinishedException;

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FinishedException>()?;
    Ok(())
}
