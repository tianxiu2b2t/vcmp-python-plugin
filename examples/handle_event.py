from vcmp.callback import callbacks
from vcmp.events.server import ServerInitialiseEvent
from vcmp.events.custom import CustomEvent
from vcmp.events import EventBuilder


@callbacks.on_server_initialise()
def _(event: ServerInitialiseEvent):
    callbacks.trigger(
        EventBuilder.custom(**{"type": "message", "message": "Hello World!"})
    )


@callbacks.on_custom()
def _(event: CustomEvent):
    print(event.kwargs)
