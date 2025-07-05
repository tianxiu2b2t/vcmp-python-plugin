use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{Bound, PyResult, Python};

use crate::py::fix_module_name;

pub mod checkpoint;
pub mod environment;
pub mod marker;
pub mod object;
pub mod pickup;
pub mod player;
pub mod server;
pub mod vehicle;
pub mod keybind;
pub mod misc;


pub fn module_define(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let server_module = PyModule::new(py, "server")?;
    server::module_define(py, &server_module)?;
    fix_module_name(py, &server_module, "functions.server");
    m.add_submodule(&server_module)?;

    let player_module = PyModule::new(py, "player")?;
    player::module_define(py, &player_module)?;
    fix_module_name(py, &player_module, "functions.player");
    m.add_submodule(&player_module)?;

    let vehicle_module = PyModule::new(py, "vehicle")?;
    vehicle::module_define(py, &vehicle_module)?;
    fix_module_name(py, &vehicle_module, "functions.vehicle");
    m.add_submodule(&vehicle_module)?;

    let environment_module = PyModule::new(py, "environment")?;
    environment::module_define(py, &environment_module)?;
    fix_module_name(py, &environment_module, "functions.environment");
    m.add_submodule(&environment_module)?;

    let pickup_module = PyModule::new(py, "pickup")?;
    pickup::module_define(py, &pickup_module)?;
    fix_module_name(py, &pickup_module, "functions.pickup");
    m.add_submodule(&pickup_module)?;

    let object_module = PyModule::new(py, "object")?;
    object::module_define(py, &object_module)?;
    fix_module_name(py, &object_module, "functions.object");
    m.add_submodule(&object_module)?;

    let marker_module = PyModule::new(py, "marker")?;
    marker::module_define(py, &marker_module)?;
    fix_module_name(py, &marker_module, "functions.marker");
    m.add_submodule(&marker_module)?;

    let checkpoint_module = PyModule::new(py, "checkpoint")?;
    checkpoint::module_define(py, &checkpoint_module)?;
    fix_module_name(py, &checkpoint_module, "functions.checkpoint");
    m.add_submodule(&checkpoint_module)?;

    let keybind_module = PyModule::new(py, "keybind")?;
    keybind::module_define(py, &keybind_module)?;
    fix_module_name(py, &keybind_module, "functions.keybind");
    m.add_submodule(&keybind_module)?;

    let misc_module = PyModule::new(py, "misc")?;
    misc::module_define(py, &misc_module)?;
    fix_module_name(py, &misc_module, "functions.misc");
    m.add_submodule(&misc_module)?;

    Ok(())
}
