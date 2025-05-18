from functools import wraps
from typing import Callable


class FinishedException(Exception):
    ...

class VCMPError(Exception):
    ...

class VCMPErrorNoSuchEntity(VCMPError):
    ...

class VCMPErrorBufferTooSmall(VCMPError):
    ...

class VCMPErrorTooLargeInput(VCMPError):
    ...

class VCMPErrorArgumentOutOfBounds(VCMPError):
    ...

class VCMPErrorNullArgument(VCMPError):
    ...

class VCMPErrorPoolExhausted(VCMPError):
    ...

class VCMPErrorInvalidName(VCMPError):
    ...

class VCMPErrorRequestDenied(VCMPError):
    ...

class VCMPErrorUnknown(VCMPError):
    ...

mappings = {
    "No such entity.": VCMPErrorNoSuchEntity,
    "Buffer too small.": VCMPErrorBufferTooSmall,
    "Too large input.": VCMPErrorTooLargeInput,
    "Argument out of bounds.": VCMPErrorArgumentOutOfBounds,
    "Null argument.": VCMPErrorNullArgument,
    "Pool exhausted.": VCMPErrorPoolExhausted,
    "Invalid name.": VCMPErrorInvalidName,
    "Request denied.": VCMPErrorRequestDenied,
    "Unknown Error": VCMPErrorUnknown
}

def from_vcmp_exception(
    e: ValueError
):
    message: str = e.args[0]
    type, message = message.split(".", 1)
    type = f"{type}."
    if type not in mappings:
        return e
    return mappings[type](message)

def wrapper_exception(
):
    def decorator(func: Callable):
        @wraps(func)
        def wrapper(*args, **kwargs):
            try:
                return func(*args, **kwargs)
            except ValueError as e:
                raise from_vcmp_exception(e) from e
            except Exception as e:
                raise VCMPErrorUnknown(f"Unknown error: {e}") from e
        return wrapper
    return decorator
    