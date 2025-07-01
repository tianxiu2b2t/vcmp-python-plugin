use pyo3::{pyclass, pymethods};

use crate::pool::EntityPoolTrait;

#[pyclass]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "Object")]
pub struct ObjectPy {
    id: i32,
}

impl ObjectPy {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

impl EntityPoolTrait for ObjectPy {
    fn entity_id(&self) -> crate::consts::EntityId {
        self.id
    }
    fn entity_pool_type() -> vcmp_bindings::options::VcmpEntityPool {
        vcmp_bindings::options::VcmpEntityPool::Object
    }
}

impl From<i32> for ObjectPy {
    fn from(val: i32) -> Self {
        ObjectPy::new(val)
    }
}

#[pymethods]
impl ObjectPy {
    #[getter]
    pub fn get_id(&self) -> i32 {
        self.id
    }
}
