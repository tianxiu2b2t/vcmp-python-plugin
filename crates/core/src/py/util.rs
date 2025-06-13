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