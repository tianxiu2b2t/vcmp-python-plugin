// https://bitbucket.org/stormeus/0.4-squirrel/src/master/CallbackHandler.cpp

use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{Bound, PyResult, Python, pyclass, pymethods};

pub mod server;

pub fn module_define(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let server_module = PyModule::new(py, "server")?;
    server::module_define(py, &server_module)?;
    m.add_submodule(&server_module)?;
    Ok(())
}
