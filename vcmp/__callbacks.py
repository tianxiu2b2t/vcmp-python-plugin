import abc
from collections import defaultdict, deque
from dataclasses import dataclass
import inspect
from queue import Queue
import threading
from typing import Any, Awaitable, Callable, Literal, Type

import anyio
import anyio.abc
from tianxiu2b2t.utils import runtime
from .__abc import calls

from .__utils import camel_to_snake, snake_to_camel

CallbackEvent = Literal[
    "server_initialise",
    "server_shutdown",
    "server_frame"
]

class Event(
    metaclass=abc.ABCMeta
):
    __fields__ = ()
    _raw_args = ()
    _raw_kwargs = {}

    def __init__(self):
        ...

    def __repr__(self):
        return f"{self.__class__.__name__}({', '.join([f'{field}={getattr(self, field)}' for field in self.__fields__])})"

class ServerFrameEvent(Event):
    __fields__ = (
        "elapsed_time",
    )
    elapsed_time: float

class ServerShutdownEvent(Event):
    ...

@dataclass
class Callback:
    priority: int
    callback: Callable[..., Awaitable[Any] | Any]

class CallbackManager:
    def __init__(self):
        self.module_callbacks = calls
        self.callbacks: deque[Callback] = deque()
        self.queues = Queue(1)
        self.events: dict[str, Type[Event]] = {}
        self._find_events()

    def _find_events(self):
        frame = inspect.currentframe()
        module = inspect.getmodule(frame)
        classes = inspect.getmembers(module, inspect.isclass)
        cls: Type[Event]

        for name, cls in classes:
            if not name.endswith("Event"):
                continue
            
            if cls == Event:
                continue
            
            # rename to lowercast and remove Event suffix and add to events dict and rename like ServerFrame to server_frame
            name = name[:-5]
            # rename ServerFrame to server_frame
            name = camel_to_snake(name)

            self.events[name] = cls
    
            self._set_callback(name)
            

    def _set_callback(
        self,
        event: str,
    ):
        def decorator(
            *args,
            **kwargs
        ):
            self.queues.put(
                (runtime.perf_counter(), event, args, kwargs)
            )
            return decorator
        setattr(self.module_callbacks, f"on_{event}", decorator)


    def on(
        self,
        priority: int = 0
    ):
        def decorator(
            callback: Callable[..., Awaitable[Any] | Any]
        ):
            self.callbacks.append(
                Callback(priority, callback)
            )
            return callback

        return decorator

    async def __call__(self) -> Any:
        async with anyio.create_task_group() as task_group:
            while 1:
                while not self.queues.empty():
                    item = self.queues.get(False)
                    running, event, args, kwargs = item
                    task_group.start_soon(
                        self._callback,
                        event,
                        args,
                        kwargs
                    )
                    await anyio.sleep(0)
                
    async def _callback(
        self,
        event: CallbackEvent,
        args: tuple[Any, ...],
        kwargs: dict[str, Any]
    ):
        if event not in self.events:
            return
        
        cls = self.events[event]
        fields = cls.__fields__

        instance = cls()
        instance._raw_args = args
        instance._raw_kwargs = kwargs
        for idx, field in enumerate(fields):
            setattr(instance, field, args[idx])
        
        await self._handle(instance)
        
    async def _handle(
        self,
        event: Event
    ):
        print(event)

callbacks = CallbackManager()

__all__ = [
    "callbacks",
    "Event",
] + [
    snake_to_camel(name + "_event") for name in callbacks.events.keys()
] # type: ignore