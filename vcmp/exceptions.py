class FinishedException(Exception):
    ...

"""

	{vcmpErrorNoSuchEntity, "No such entity."},
	{vcmpErrorBufferTooSmall, "Buffer too small."},
	{vcmpErrorTooLargeInput, "Too large input."},
	{vcmpErrorArgumentOutOfBounds, "Argument out of bounds."},
	{vcmpErrorNullArgument, "Null argument."},
	{vcmpErrorPoolExhausted, "Pool exhausted."},
	{vcmpErrorInvalidName, "Invalid name."},
	{vcmpErrorRequestDenied, "Request denied."},
	{forceSizeVcmpError, "Unknown Error"}
"""

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
    message = e.args[0]
    type, message = message.split(".", 1)
    return mappings[type + "."](message)