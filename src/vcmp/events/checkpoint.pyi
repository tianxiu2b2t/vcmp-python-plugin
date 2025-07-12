from typing import Optional

from .abc import Event
from vcmp.functions.checkpoint import CheckPoint
from .player import Player

class CheckpointEvent(Event):
    ...

class CheckpointEnteredEvent(CheckpointEvent):
    @property
    def checkpoint(self) -> CheckPoint: ...
    
    @property
    def player(self) -> Player: ...
    
    def __repr__(self) -> str: ...

class CheckpointExitedEvent(CheckpointEvent):
    @property
    def checkpoint(self) -> CheckPoint: ...
    
    @property
    def player(self) -> Player: ...
    
    def __repr__(self) -> str: ...