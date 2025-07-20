use pyo3::{
    Bound, PyResult, Python, pyfunction,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction,
};

use vcmp_bindings::{func::ServerMethods, vcmp_func};

use crate::py::types::ServerSettingsPy;

#[pyfunction]
pub fn set_servername(server_name: &str) {
    let _ = vcmp_func().set_server_name(server_name);
}

#[pyfunction]
pub fn get_servername() -> String {
    vcmp_func().get_server_name()
}

#[pyfunction]
pub fn set_gamemode(gamemode: &str) {
    let _ = vcmp_func().set_gamemode(gamemode);
}

#[pyfunction]
pub fn get_gamemode() -> String {
    vcmp_func().get_gamemode()
}

#[pyfunction]
pub fn set_password(password: &str) {
    let _ = vcmp_func().set_server_password(password);
}

#[pyfunction]
pub fn get_password() -> String {
    vcmp_func().get_server_password()
}

#[pyfunction]
pub fn set_max_players(max_players: u32) {
    let _ = vcmp_func().set_max_players(max_players);
}

#[pyfunction]
pub fn get_max_players() -> u32 {
    vcmp_func().get_max_players()
}

#[pyfunction]
pub fn get_server_version() -> u32 {
    vcmp_func().server_version()
}

#[pyfunction]
pub fn shutdown_server() {
    vcmp_func().shutdown();
}

#[pyfunction]
pub fn get_server_settings() -> ServerSettingsPy {
    ServerSettingsPy::from(vcmp_func().server_settings())
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(set_server_name, m)?)?;
    m.add_function(wrap_pyfunction!(get_server_name, m)?)?;
    m.add_function(wrap_pyfunction!(set_gamemode, m)?)?;
    m.add_function(wrap_pyfunction!(get_gamemode, m)?)?;
    m.add_function(wrap_pyfunction!(set_password, m)?)?;
    m.add_function(wrap_pyfunction!(get_password, m)?)?;
    m.add_function(wrap_pyfunction!(set_max_players, m)?)?;
    m.add_function(wrap_pyfunction!(get_max_players, m)?)?;
    m.add_function(wrap_pyfunction!(get_server_version, m)?)?;
    m.add_function(wrap_pyfunction!(shutdown_server, m)?)?;
    m.add_function(wrap_pyfunction!(get_server_settings, m)?)?;
    Ok(())
}
