from vcmp.events.abc import Event

class PluginEvent(Event):
    ...

class PluginCommnadEvent(PluginEvent):
    __fields__ = (
        "command_identifier",
        "message"
    )
    command_identifier: str
    message: str