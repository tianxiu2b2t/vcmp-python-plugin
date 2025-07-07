use pyo3::{
    Bound, PyResult, Python, pyfunction,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction,
};
use vcmp_bindings::{
    func::MiscMethods,
    vcmp_func,
};

use crate::{
    functions::player::PlayerPy,
    py::types::VectorPy,
};

#[pyfunction]
#[pyo3(signature = (world_id, explosion_type, pos, target = None, ground = false))]
pub fn create_explosion(
    py: Python<'_>,
    world_id: i32,
    explosion_type: i32,
    pos: VectorPy,
    target: Option<PlayerPy>,
    ground: bool,
) {
    py.allow_threads(|| {
        vcmp_func().create_explosion(
            world_id,
            explosion_type,
            pos.into(),
            target.map(|p| p.get_id()).unwrap_or(-1),
            ground,
        );
    })
}

#[pyfunction]
pub fn play_sound(world_id: i32, sound_id: i32, pos: VectorPy) {
    let position = pos.get_entity_pos();
    vcmp_func().play_sound(world_id, sound_id, position.x, position.y, position.z);
}

#[pyfunction]
pub fn hide_map_object(py: Python<'_>, object_id: i32, pos: VectorPy) {
    py.allow_threads(|| vcmp_func().hide_map_object(object_id, pos.into()))
}

#[pyfunction]
pub fn show_map_object(object_id: i32, pos: VectorPy) {
    vcmp_func().hide_map_object(object_id, pos.into());
}

#[pyfunction]
pub fn show_all_map_objects() {
    vcmp_func().show_all_map_objects();
}

#[pyfunction]
pub fn add_radio_stream(id: i32, name: &str, url: &str, can_select: bool) {
    vcmp_func().add_radio_stream(id, name, url, can_select);
}

#[pyfunction]
pub fn remove_radio_stream(id: i32) {
    vcmp_func().remove_radio_stream(id);
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_explosion, m)?)?;
    m.add_function(wrap_pyfunction!(play_sound, m)?)?;
    m.add_function(wrap_pyfunction!(hide_map_object, m)?)?;
    m.add_function(wrap_pyfunction!(show_map_object, m)?)?;
    m.add_function(wrap_pyfunction!(show_all_map_objects, m)?)?;
    m.add_function(wrap_pyfunction!(add_radio_stream, m)?)?;
    m.add_function(wrap_pyfunction!(remove_radio_stream, m)?)?;
    Ok(())
}
