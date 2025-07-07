use pyo3::{
    Bound, PyResult, Python, pyclass, pyfunction, pymethods,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction,
};
use vcmp_bindings::{func::KeybindMethods, vcmp_func};

use crate::py::types::KeyCode;

#[pyclass]
#[pyo3(name = "KeyBind")]
pub struct KeyBindPy {
    slot: i32,
}

impl KeyBindPy {
    pub fn new(slot: i32) -> Self {
        Self { slot }
    }
}

#[pymethods]
impl KeyBindPy {
    #[getter]
    pub fn get_slot(&self) -> i32 {
        self.slot
    }

    #[getter]
    pub fn can_release(&self) -> bool {
        let res = vcmp_func().get_key_bind_data(self.slot);
        res.map(|r| r.can_release).unwrap_or(false)
    }

    #[getter]
    pub fn get_key(&self) -> i32 {
        let res = vcmp_func().get_key_bind_data(self.slot);
        res.map(|r| r.key).unwrap_or(0)
    }

    #[getter]
    pub fn get_key2(&self) -> i32 {
        let res = vcmp_func().get_key_bind_data(self.slot);
        res.map(|r| r.key2).unwrap_or(0)
    }

    #[getter]
    pub fn get_key3(&self) -> i32 {
        let res = vcmp_func().get_key_bind_data(self.slot);
        res.map(|r| r.key3).unwrap_or(0)
    }
}

#[pyfunction]
#[pyo3(name = "bindkey", signature = (can_release, key, key2=None, key3=None))]
pub fn bindkey(can_release: bool, key: i32, key2: Option<i32>, key3: Option<i32>) -> KeyBindPy {
    let keybind = vcmp_func().register_key_bind(can_release, key, key2, key3);
    KeyBindPy::new(keybind.slot)
}

#[pyfunction]
#[pyo3(name = "bindkey", signature = (can_release, key, key2=None, key3=None))]
pub fn bindkey1(
    can_release: bool,
    key: KeyCode,
    key2: Option<KeyCode>,
    key3: Option<KeyCode>,
) -> KeyBindPy {
    let keybind = vcmp_func().register_key_bind(
        can_release,
        key.into(),
        key2.map(|k| k.into()),
        key3.map(|k| k.into()),
    );
    KeyBindPy::new(keybind.slot)
}

#[pyfunction]
pub fn get_bindkey(slot: i32) -> Option<KeyBindPy> {
    if let Ok(keybind) = vcmp_func().get_key_bind_data(slot) {
        Some(KeyBindPy::new(keybind.slot))
    } else {
        None
    }
}

#[pyfunction]
pub fn remove_bind_key(slot: i32) {
    vcmp_func().remove_key_bind(slot);
}

#[pyfunction]
pub fn remove_all_key_binds() {
    vcmp_func().remove_all_key_binds();
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<KeyBindPy>()?;
    m.add_function(wrap_pyfunction!(bindkey, m)?)?;
    m.add_function(wrap_pyfunction!(get_bindkey, m)?)?;
    m.add_function(wrap_pyfunction!(remove_bind_key, m)?)?;
    m.add_function(wrap_pyfunction!(remove_all_key_binds, m)?)?;
    Ok(())
}
