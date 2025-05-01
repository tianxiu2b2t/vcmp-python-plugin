import abc
import inspect
from typing import Type


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
    