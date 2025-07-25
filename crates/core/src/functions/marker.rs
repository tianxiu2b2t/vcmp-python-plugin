use std::fmt::Display;

use pyo3::{
    Bound, PyResult, Python, pyclass, pyfunction, pymethods,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction,
};
use vcmp_bindings::{func::MarkerMethods, vcmp_func};

use crate::{
    pool::{ENTITY_POOL, EntityPoolTrait},
    py::types::{EntityVectorType, RGBPy, VectorPy},
};

#[pyclass]
#[derive(Debug, Clone, Copy)]
#[pyo3(name = "Marker")]
pub struct MarkerPy {
    id: i32,
}

impl Display for MarkerPy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Marker(id={})", self.id)
    }
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

    fn __hash__(&self) -> i32 {
        self.id
    }

    fn __eq__(&self, other: &MarkerPy) -> bool {
        self.id == other.id
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
        vcmp_func().destory_marker(self.id);
    }

    fn __repr__(&self) -> String {
        format!("Marker(id={})", self.id)
    }
}

#[pyfunction]
pub fn create_marker(
    model: i32,
    world: i32,
    position: VectorPy,
    scale: i32,
    color: RGBPy,
) -> MarkerPy {
    let id = vcmp_func().create_marker(world, position.into(), scale, color.into(), model, None);

    let pool = ENTITY_POOL.lock().unwrap();

    pool.get_marker(id).copied().unwrap_or(MarkerPy::new(id))
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MarkerPy>()?;
    m.add_function(wrap_pyfunction!(create_marker, m)?)?;
    Ok(())
}
