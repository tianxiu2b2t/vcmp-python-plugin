use pyo3::{types::PyAnyMethods, Python};
use squirrel_ffi::{get_sq, init_squirrel, raw::HSQUIRRELVM, register_func};
use squirrel_ffi_macros::sq_function;
use tracing::{event, Level};

use crate::py::get_traceback;

pub fn init() {
    match init_squirrel() {
        Ok(_) => {
            event!(Level::INFO, "Squirrel initialized successfully");
        },
        Err(e) => {
            event!(Level::ERROR, "Failed to initialize squirrel: {e}");
        }
    }
    register_functions();
}

pub fn register_functions() {
    register_func("execPython", exec_python, "s");
    register_func("evalPython", eval_python, "s");
}

#[sq_function]
pub fn exec_python(vm: HSQUIRRELVM) -> i64 {
    let api = get_sq().get_api();
    let args_count = api.get_top(vm);
    if args_count != 2 {
        return api.throw_error(vm, "execPython expects exactly one argument");
    }

    let script = api.get_string(vm, 2);
    event!(Level::DEBUG, "Executing Python script: {script}");

    let c_str = std::ffi::CString::new(script).unwrap();

    let res = Python::with_gil(|py| {
        py.run(&c_str, None, None)
    });

    if let Err(e) = res {
        let traceback = get_traceback(&e, None);
        event!(Level::ERROR, "Failed to execute Python script: {traceback}");
        return api.throw_error(vm, &format!("Failed to execute Python script: {e}"));
    }

    1
}

#[sq_function]
pub fn eval_python(vm: HSQUIRRELVM) -> i64 {
    let api = get_sq().get_api();
    let args_count = api.get_top(vm);
    if args_count != 2 {
        return api.throw_error(vm, "evalPython expects exactly one argument");
    }

    let script = api.get_string(vm, 2);
    event!(Level::DEBUG, "Executing Python script: {script}");

    let c_str = std::ffi::CString::new(script).unwrap();

    let result = match Python::with_gil(|py| {
        py.eval(&c_str, None, None).map(|res| res.unbind())
    }) {
        Ok(r) => r,
        Err(e) => {
            let traceback = get_traceback(&e, None);
            event!(Level::ERROR, "Failed to execute Python script: {traceback}");
            return api.throw_error(vm, &format!("Failed to execute Python script: {e}"));
        }
    };

    Python::with_gil(|py| {
        let value = result.bind(py);
        if value.is_none() {
            api.push_null(vm);
        } else if let Ok(res) = value.extract::<String>() {
            api.push_string(vm, res.as_str());
        } else if let Ok(res) = value.extract::<f32>() {
            api.push_float(vm, res);
        } else if let Ok(res) = value.extract::<i64>() {
            api.push_integer(vm, res);
        } else if let Ok(res) = value.extract::<bool>() {
            api.push_bool(vm, res);
        } else {
            api.push_user_pointer(vm, value.as_ptr() as *mut std::ffi::c_void);
        }
    });


    1
}