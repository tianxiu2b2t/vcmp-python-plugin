from .abc import Event

class CheckpointEvent(Event):
    ...

class CheckpointEnteredEvent(CheckpointEvent):
    ...

class CheckpointExitedEvent(CheckpointEvent):
    ...