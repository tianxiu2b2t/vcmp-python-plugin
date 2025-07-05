use pyo3::{
    Bound, PyResult, Python, pyclass, pymethods,
    types::{PyModule, PyModuleMethods},
};
use vcmp_bindings::{func::MarkerMethods, vcmp_func};

use crate::{
    pool::EntityPoolTrait,
    py::types::{EntityVectorType, RGBPy, VectorPy},
};

#[pyclass]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "Marker")]
pub struct MarkerPy {
    id: i32,
}

impl MarkerPy {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

impl EntityPoolTrait for MarkerPy {
    fn entity_id(&self) -> crate::consts::EntityId {
        self.id
    }
    fn entity_pool_type() -> vcmp_bindings::options::VcmpEntityPool {
        vcmp_bindings::options::VcmpEntityPool::Marker
    }
}

impl From<i32> for MarkerPy {
    fn from(val: i32) -> Self {
        MarkerPy::new(val)
    }
}

impl MarkerPy {
    pub fn _position(&self) -> VectorPy {
        VectorPy::from((EntityVectorType::MarkerPosition, self.id))
    }
}

#[pymethods]
impl MarkerPy {
    #[getter]
    pub fn get_id(&self) -> i32 {
        self.id
    }

    #[getter]
    pub fn get_color(&self) -> RGBPy {
        RGBPy::from(vcmp_func().get_marker_info(self.id).color)
    }

    #[getter]
    pub fn get_is_alive(&self) -> bool {
        vcmp_func().is_marker_alive(self.id)
    }

    #[getter]
    pub fn get_model(&self) -> i32 {
        vcmp_func().get_marker_info(self.id).sprite
    }

    #[getter]
    pub fn get_position(&self) -> VectorPy {
        VectorPy::from((EntityVectorType::MarkerPosition, self.id))
    }

    #[getter]
    pub fn get_scale(&self) -> i32 {
        vcmp_func().get_marker_info(self.id).scale
    }

    #[getter]
    pub fn get_world(&self) -> i32 {
        vcmp_func().get_marker_info(self.id).world
    }

    pub fn delete(&self) {
        let _ = vcmp_func().destory_marker(self.id);
    }
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MarkerPy>()?;
    Ok(())
}
