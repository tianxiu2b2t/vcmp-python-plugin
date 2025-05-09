from collections import deque
from dataclasses import dataclass
import inspect
import re
from typing import Any, Callable, Type, Union, get_args, get_type_hints

from .logger import logger
from .exceptions import FinishedException
from .events import Event
from .__export import calls
from . import events

@dataclass
class CallbackArg:
    name: str
    annotations: tuple[Type, ...]
    required: bool
    default: Any = None


class Callback:
    def __init__(
        self,
        priority: int,
        callback: Callable[..., Any]
    ):
        self.priority = priority
        self.callback = callback
        
        handler_args = inspect.getfullargspec(callback)  
        annotations_params = get_type_hints(callback)  
        defaults = handler_args.defaults or ()
        offset = len(handler_args.args) - len(defaults)
        self.args = [  
            CallbackArg(name=param, annotations=self._get_annotations(annotations_params.get(param, Any)), default=defaults[i - offset] if i - offset >= 0 else None, required=i < offset)  
            for i, param in enumerate(handler_args.args)  
        ]  
        self.return_annotation = handler_args.annotations.get("return", Any)

    def _get_annotations(self, value: Type[Any]):
        if hasattr(value, "__origin__") and value.__origin__ is Union:
            return get_args(value)
        return (value, )

class CallbackManager:
    def __init__(self):
        self.module_callbacks = calls
        self.callbacks: deque[Callback] = deque()
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
            
            name = re.sub('([a-z0-9])([A-Z])', r'\1_\2', 
                re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name[:-5])
            ).lower()

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
            try:
                return self._callback(event, args, kwargs)
            except:
                ...
            return None
        setattr(self.module_callbacks, f"on_{event}", decorator)


    def on(
        self,
        priority: int = 0
    ):
        def decorator(
            callback: Callable[..., Any]
        ):
            self.callbacks.append(
                Callback(priority, callback)
            )
            return callback

        return decorator
                
    def _callback(
        self,
        event: str,
        args: tuple[Any, ...],
        kwargs: dict[str, Any]
    ):
        if event not in self.events:
            return
        
        cls = self.events[event]
        fields = cls.__fields__

        instance = cls(*args, **kwargs)
        for idx, field in enumerate(fields):
            setattr(instance, field, args[idx])
            
        #print(event, args, kwargs)
        res = self._handle(instance)
        if res is None:
            return 1
        return res

        
    def _handle(
        self,
        event: Event
    ):
        matcher = Matcher(event)
        result = None
        for callback in self.callbacks:
            params = {}
            matched = True
            for arg in callback.args:
                if isinstance(event, arg.annotations):
                    params[arg.name] = event
                elif isinstance(matcher, arg.annotations):
                    params[arg.name] = matcher

                if arg.required and arg.name not in params:
                    matched = False
                    break
                elif arg.name not in params:
                    params[arg.name] = arg.default

            if not matched:
                continue
            try:
                result = callback.callback(**params)
            except FinishedException:
                result = result or matcher._result
                break
            except:
                logger.traceback(f"Error in callback {callback.callback.__name__}")
        return result

        
class Matcher:
    def __init__(
        self,
        event: Event
    ):
        self.event = event
        self._finished = False
        self._result = None

    def send(
        self,
        message: str
    ):
        if isinstance(self.event, events.PlayerEvent):
            player = self.event.player

            player.send_message(message)

    
    def finish(
        self,
        result: Any = None
    ):
        self._finished = True
        self._result = result

        raise FinishedException()

    @property
    def finished(self):
        return self._finished
        

    

callbacks = CallbackManager()

__all__ = [
    "callbacks",
]