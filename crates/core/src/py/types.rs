use pyo3::{
    Bound, PyResult, Python, pyclass, pymethods,
    types::{PyModule, PyModuleMethods},
};

use vcmp_bindings::setting::VcmpServerSettings;

#[derive(Clone)]
#[pyclass]
#[pyo3(name = "ServerSettings")]
pub struct ServerSettingsPy {
    pub inner: VcmpServerSettings,
}

impl ServerSettingsPy {
    pub fn from(value: VcmpServerSettings) -> Self {
        Self { inner: value }
    }
}

#[pymethods]
impl ServerSettingsPy {
    #[getter]
    pub fn get_server_name(&self) -> String {
        self.inner.server_name()
    }

    #[getter]
    pub fn get_max_players(&self) -> u32 {
        self.inner.max_players()
    }

    #[getter]
    pub fn get_port(&self) -> u32 {
        self.inner.port()
    }

    #[getter]
    pub fn get_flags(&self) -> u32 {
        self.inner.flags()
    }

    fn __repr__(&self) -> String {
        format!(
            r#"ServerSettings(server_name='{}', max_players={}, port={}, flags={})"#,
            self.inner.server_name(),
            self.inner.max_players(),
            self.inner.port(),
            self.inner.flags()
        )
    }
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ServerSettingsPy>()?;
    Ok(())
}
