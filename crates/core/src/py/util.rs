use std::ffi::CString;

use pyo3::prelude::*;

pub fn py_run(code: &str) {
    let c = CString::new(code).expect("Failed to convert code to CString");
    Python::with_gil(|py| {
        let r = py.run(&c, None, None);
        match r {
            Ok(_) => (),
            Err(e) => {
                e.print(py);
            }
        }
    })
}

pub fn create_submodule<'p>(
    py: Python<'p>,
    name: &str,
    func: fn(Python<'p>, &Bound<'_, PyModule>) -> PyResult<()>,
) -> Bound<'p, PyModule> {
    let submodule = PyModule::new(py, name).expect("Failed to create submodule");
    func(py, &submodule).expect("Failed to initialize submodule");
    submodule
}
