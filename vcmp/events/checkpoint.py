from .abc import Event

class CheckpointEvent:
    ...

class CheckpointEnteredEvent(Event):
    ...

class CheckpointExitedEvent(Event):
    ...