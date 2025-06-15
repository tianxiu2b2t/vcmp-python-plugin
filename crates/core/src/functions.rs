use pyo3::{
    Python, Bound, PyResult
};
use pyo3::types::PyModule;
use pyo3::prelude::*;

pub mod callbacks;
pub mod player;
pub mod vehicle;
pub mod server;

pub fn module_define(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    
    let server_module = PyModule::new(py, "server")?;
    server::module_define(py, &server_module)?;
    m.add_submodule(&server_module)?;
    
    Ok(())

}
