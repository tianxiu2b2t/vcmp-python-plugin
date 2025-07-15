from .abc import Event

from traceback import TracebackException


class CustomEvent(Event): ...


class TracebackEvent(CustomEvent):
    @property
    def traceback(self) -> "TracebackException": ...
