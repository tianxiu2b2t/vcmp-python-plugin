import abc
from collections import defaultdict, deque
from dataclasses import dataclass, field
import inspect
from queue import Queue
import threading
from typing import Any, Awaitable, Callable, Literal, Type

import anyio
import anyio.abc
from tianxiu2b2t.utils import runtime

from .__abc import calls

from .__utils import camel_to_snake, snake_to_camel
from . import events
from .events import Event

@dataclass
class FunctionCallback:
    event: threading.Lock = field(default_factory=threading.Lock)
    result: Any = None

    def wait(self):
        self.event.acquire()
        self.event.release()
        return self.result
    
    def set_result(self, result: Any):
        self.result = result
        self.event.release()

@dataclass
class Callback:
    priority: int
    callback: Callable[..., Awaitable[Any] | Any]

class CallbackManager:
    def __init__(self):
        self.module_callbacks = calls
        self.callbacks: deque[Callback] = deque()
        self.queues = Queue(1)
        self.callback_queue: defaultdict[str, FunctionCallback] = defaultdict(FunctionCallback)
        self.events: dict[str, Type[Event]] = {}
        self._find_events()

    def _find_events(self):
        module = inspect.getmodule(events)
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
            self.callback_queue[event].result = None
            self.callback_queue[event].event.acquire()
            self.queues.put(
                (event, args, kwargs)
            )
            return self.callback_queue[event].wait()
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
                    event, args, kwargs = item
                    task_group.start_soon(
                        self._callback,
                        event,
                        args,
                        kwargs
                    )
                    await anyio.sleep(0)
                
    async def _callback(
        self,
        event: str,
        args: tuple[Any, ...],
        kwargs: dict[str, Any]
    ):
        if event not in self.events:
            self.callback_queue[event].set_result(1)
            return
        
        cls = self.events[event]
        fields = cls.__fields__

        instance = cls(*args, **kwargs)
        for idx, field in enumerate(fields):
            setattr(instance, field, args[idx])
            
        
        res = await self._handle(instance)
        self.callback_queue[event].set_result(res)
        
    async def _handle(
        self,
        event: Event
    ):
        if isinstance(event, (
            events.ServerFrameEvent,
            events.PlayerUpdateEvent
        )):
            return
        print(event, event._raw_args, event._raw_kwargs)
        

callbacks = CallbackManager()

__all__ = [
    "callbacks",
]