use pyo3::{pyclass, pymethods};

use vcmp_bindings::func::PlayerMethods;
use vcmp_bindings::vcmp_func;

use crate::pool::EntityPoolTrait;

#[pyclass]
#[pyo3(name = "Player")]
#[derive(Debug, Clone, Copy)]
pub struct PlayerPy {
    id: i32,
}

impl PlayerPy {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

impl EntityPoolTrait for PlayerPy {
    fn entity_id(&self) -> crate::consts::EntityId {
        self.id
    }

    fn entity_pool_type() -> vcmp_bindings::options::VcmpEntityPool {
        vcmp_bindings::options::VcmpEntityPool::Player
    }
}

#[pymethods]
impl PlayerPy {
    #[getter]
    fn get_name(&self) -> String {
        vcmp_func().get_player_name(self.id)
    }

    #[setter]
    fn set_name(&self, name: String) {
        vcmp_func().set_player_name(self.id, name.as_str());
    }

    #[getter]
    pub fn get_id(&self) -> i32 {
        self.id
    }
}
