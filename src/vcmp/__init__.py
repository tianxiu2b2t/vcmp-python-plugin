from typing import Any, Callable, Optional
from . import util
from . import streams
from . import functions
from . import callbacks
from . import events
from . import instance


def reload(**kwargs):
    """
    Reloads the module.

    Call this function to reload the module after next server frame.
    """
    pass


def set_error_handler(handler: Optional[Callable[[BaseException], Any]] = None):
    """
    Sets the error handler.

    The error handler is called when an error occurs in the module.

    Parameters
    ----------
    handler : Optional[Callable[BaseException], Any]
        The error handler. If None, the default message will be output to rust logger.
    """
    pass


def get_error_handler() -> Optional[Callable[[BaseException], Any]]:
    """
    Gets the error handler.

    Returns
    -------
    Optional[Callable[BaseException], Any]
        The error handler.
    """
    pass


__all__ = [
    "util",
    "streams",
    "functions",
    "callbacks",
    "events",
    "instance",
    "reload",
    "set_error_handler",
    "get_error_handler",
]
