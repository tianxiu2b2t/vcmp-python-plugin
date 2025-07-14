use crate::functions::marker::MarkerPy;
use crate::functions::object::ObjectPy;
use crate::functions::pickup::PickupPy;
use crate::functions::player::PlayerPy;
use crate::functions::{checkpoint::CheckPointPy, vehicle::VehiclePy};
use crate::pool::ENTITY_POOL;

use pyo3::types::PyModuleMethods;
use pyo3::{Bound, PyResult, Python, pyfunction, types::PyModule, wrap_pyfunction};

#[pyfunction]
pub fn get_players() -> Vec<PlayerPy> {
    let pool = ENTITY_POOL.lock().unwrap();
    pool.get_players()
}

#[pyfunction]
pub fn get_vehicles() -> Vec<VehiclePy> {
    let pool = ENTITY_POOL.lock().unwrap();
    pool.get_vehicles()
}

#[pyfunction]
pub fn get_pickups() -> Vec<PickupPy> {
    let pool = ENTITY_POOL.lock().unwrap();
    pool.get_pickups()
}

#[pyfunction]
pub fn get_objects() -> Vec<ObjectPy> {
    let pool = ENTITY_POOL.lock().unwrap();
    pool.get_objects()
}

#[pyfunction]
pub fn get_markers() -> Vec<MarkerPy> {
    let pool = ENTITY_POOL.lock().unwrap();
    pool.get_markers()
}

#[pyfunction]
pub fn get_checkpoints() -> Vec<CheckPointPy> {
    let pool = ENTITY_POOL.lock().unwrap();
    pool.get_checkpoints()
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_players, m)?)?;
    m.add_function(wrap_pyfunction!(get_vehicles, m)?)?;
    m.add_function(wrap_pyfunction!(get_pickups, m)?)?;
    m.add_function(wrap_pyfunction!(get_objects, m)?)?;
    m.add_function(wrap_pyfunction!(get_markers, m)?)?;
    m.add_function(wrap_pyfunction!(get_checkpoints, m)?)?;
    Ok(())
}
