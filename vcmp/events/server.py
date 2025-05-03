from .abc import Event

class ServerEvent(Event):
    ...

class ServerInitialiseEvent(ServerEvent):
    ...

class ServerFrameEvent(ServerEvent):
    __fields__ = (
        "elapsed_time",
    )
    elapsed_time: float

class ServerShutdownEvent(ServerEvent):
    ...

class ServerPerformanceReport(ServerEvent):
    __fields__ = (
        "descriptions",
        "times",
    )
    descriptions: str
    times: int