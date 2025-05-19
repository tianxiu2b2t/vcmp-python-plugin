from vcmp.events.abc import Event

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

class ServerPerformanceReportEvent(ServerEvent):
    __fields__ = (
        "count",
        "descriptions",
        "times",
    )
    count: int
    descriptions: list[str]
    times: list[int]