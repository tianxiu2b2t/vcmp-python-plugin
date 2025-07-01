use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{Bound, PyResult, Python};

pub mod object;
pub mod pickup;
pub mod player;
pub mod server;
pub mod vehicle;

pub fn module_define(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let server_module = PyModule::new(py, "server")?;
    server::module_define(py, &server_module)?;
    m.add_submodule(&server_module)?;

    let player_module = PyModule::new(py, "player")?;
    player::module_define(py, &player_module)?;
    m.add_submodule(&player_module)?;

    Ok(())
}
