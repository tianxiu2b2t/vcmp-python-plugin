import inspect
from typing import Type
from .abc import Event

from .checkpoint import *
from .entity import *
from .object import *
from .pickup import *
from .player import *
from .server import *
from .vehicle import *
from .plugin import *

def get_current_events():
    frame = inspect.currentframe()
    module = inspect.getmodule(frame)
    classes = inspect.getmembers(module, inspect.isclass)
    cls: Type[Event]

    for name, cls in classes:
        if not name.endswith("Event"):
            continue
        
        if cls == Event:
            continue
        yield cls

modules = [
    "Event",
] + [cls.__name__ for cls in get_current_events()]

__all__ = modules # type: ignore