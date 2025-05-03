from .abc import Event

class CheckpointEvent(Event):
    ...

class CheckpointEnteredEvent(CheckpointEvent):
    __fields__ = (
        "checkpointid",
        "id",
    )
    checkpointid: int
    id: int

class CheckpointExitedEvent(CheckpointEvent):
    __fields__ = (
        "checkpointid",
        "id",
    )
    checkpointid: int
    id: int