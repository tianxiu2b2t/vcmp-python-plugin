/*

class Event(
    metaclass=abc.ABCMeta
):
    __fields__ = ()
    _raw_args = ()
    _raw_kwargs = {}

    def __init__(
        self,
        *args,
        **kwargs
    ):
        self._raw_args = args
        self._raw_kwargs = kwargs

    def __repr__(self):
        return f"{self.__class__.__name__}({', '.join([f'{field}={getattr(self, field)}' for field in self.__fields__])})"

    @property
    def raw_args(self):
        return self._raw_args

    @property
    def raw_kwargs(self):
        return self._raw_kwargs
*
// 不是,所以你是要用来干啥
//
// 转成rust侧来干


use pyo3::prelude::*;
use pyo3::{pyclass, pymethods, types::{PyDict, PyTuple}, Python};

#[pyclass]
pub struct ABCEvent {
    #[pyo3(get, set)]
    pub __fields__: ,
    #[pyo3(get, set)]
    pub _raw_args: PyTuple,
    #[pyo3(get, set)]
    pub _raw_kwargs: PyDict,
}

#[pymethods]
impl ABCEvent {
    #[new]
    #[pyo3(signature = *args, **kwargs)]
    fn new(py: Python<'_>, args: &PyTuple, kwargs: &PyDict) -> Self {
        Self {
            __fields__: PyTuple::new(py, &[]),
            _raw_args: args.clone(),
            _raw_kwargs: kwargs.clone(),
        }
    }
}*/
