use pyo3::{prelude::*};
use pyo3::types::{PyDict, PyTuple};
use pyo3::{
    pymethods
};

#[pyclass(subclass)]
#[derive(Debug)]
pub struct Event {
    raw_args: Py<PyTuple>,
    raw_kwargs: Py<PyDict>,
}

#[pymethods]
impl Event {
    #[new]
    #[pyo3(signature = (*args, **kwargs))]
    fn new(py: Python<'_>, args: Py<PyTuple>, kwargs: Option<Py<PyDict>>) -> Self {
        let raw_args = args;
        let raw_kwargs = kwargs.unwrap_or_else(|| PyDict::new(py).unbind());
        
        Event { raw_args, raw_kwargs }
    }

    #[getter]
    fn raw_args(&self, py: Python<'_>) -> Py<PyTuple> {
        self.raw_args.clone_ref(py)
    }

    #[getter]
    fn raw_kwargs(&self, py: Python<'_>) -> Py<PyDict> {
        self.raw_kwargs.clone_ref(py)
    }

    /*fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        let self_type = self.clone().into_pyobject(py).unwrap().get_type();
        let class_name = self_type.name().unwrap().unbind();
        
        // 尝试获取 __fields__ 属性
        let fields = match self_type.getattr("__fields__") {
            Ok(fields_attr) => {
                // 假设 __fields__ 是一个元组或列表
                let fields_list: Vec<String> = fields_attr.extract()?;
                fields_list
            }
            Err(_) => Vec::new(),
        };

        let mut field_values = Vec::new();
        for field in &fields {
            if let Ok(value) = self.clone().into_pyobject(py).unwrap().getattr(field) {
                let repr_str = value.into_pyobject(py).unwrap().repr()?.to_string();
                field_values.push(format!("{}={}", field, repr_str));
            }
        }

        Ok(format!("{}({})", class_name, field_values.join(", ")))
    }*/

    // 添加类属性支持
    #[classattr]
    fn __fields__() -> PyObject {
        Python::with_gil(|py| PyTuple::empty(py).into())
    }
}
