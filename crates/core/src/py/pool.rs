use crate::functions::{
    checkpoint::CheckPointPy, marker::MarkerPy, object::ObjectPy, pickup::PickupPy,
    player::PlayerPy, vehicle::VehiclePy,
};
use crate::pool::ENTITY_POOL;

use pyo3::types::PyModuleMethods;
use pyo3::{Bound, PyResult, Python, pyfunction, types::PyModule, wrap_pyfunction};
use pyo3::{Py, PyAny};

#[pyfunction]
#[pyo3(signature = (all = false))]
pub fn get_players(all: bool) -> Vec<PlayerPy> {
    let pool = ENTITY_POOL.lock().unwrap();
    if all {
        pool.get_all_players()
    } else {
        pool.get_players()
    }
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

#[pyfunction]
pub fn clear_vehicles() -> usize {
    let size = get_vehicles().len();
    for vehicle in get_vehicles() {
        vehicle.delete();
    }
    size
}

#[pyfunction]
pub fn clear_objects() -> usize {
    let size = get_objects().len();
    for object in get_objects() {
        object.delete();
    }
    size
}

#[pyfunction]
pub fn clear_markers() -> usize {
    let size = get_markers().len();
    for marker in get_markers() {
        marker.delete();
    }
    size
}

#[pyfunction]
pub fn clear_checkpoints() -> usize {
    let size = get_checkpoints().len();
    for checkpoint in get_checkpoints() {
        checkpoint.delete();
    }
    size
}

#[pyfunction]
pub fn clear_pickups() -> usize {
    let size = get_pickups().len();
    for pickup in get_pickups() {
        pickup.delete();
    }
    size
}

#[pyfunction]
pub fn clear_all() -> usize {
    clear_vehicles() + clear_objects() + clear_markers() + clear_checkpoints() + clear_pickups()
}

#[pyfunction]
pub fn find_player(py: Python<'_>, value: Py<PyAny>) -> Option<PlayerPy> {
    if let Ok(id) = value.extract::<i32>(py) {
        get_players(true).iter().find(|p| p.get_id() == id).cloned()
    } else if let Ok(name) = value.extract::<String>(py) {
        get_players(true)
            .iter()
            .find(|p| p.get_name() == name)
            .cloned()
    } else {
        None
    }
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_players, m)?)?;
    m.add_function(wrap_pyfunction!(get_vehicles, m)?)?;
    m.add_function(wrap_pyfunction!(get_pickups, m)?)?;
    m.add_function(wrap_pyfunction!(get_objects, m)?)?;
    m.add_function(wrap_pyfunction!(get_markers, m)?)?;
    m.add_function(wrap_pyfunction!(get_checkpoints, m)?)?;
    m.add_function(wrap_pyfunction!(clear_vehicles, m)?)?;
    m.add_function(wrap_pyfunction!(clear_objects, m)?)?;
    m.add_function(wrap_pyfunction!(clear_markers, m)?)?;
    m.add_function(wrap_pyfunction!(clear_checkpoints, m)?)?;
    m.add_function(wrap_pyfunction!(clear_pickups, m)?)?;
    m.add_function(wrap_pyfunction!(clear_all, m)?)?;
    m.add_function(wrap_pyfunction!(find_player, m)?)?;
    Ok(())
}
