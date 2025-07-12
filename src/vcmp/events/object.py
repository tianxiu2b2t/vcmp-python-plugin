from .abc import Event

from vcmp.functions.object import Object
from vcmp.functions.player import Player


class ObjectEvent(Event):
    ...

class ObjectShotEvent(ObjectEvent):
    @property
    def object(self) -> Object: ...
    
    @property
    def player(self) -> Player: ...
    
    @property
    def weapon_id(self) -> int: ...
    
    def __repr__(self) -> str: ...

class ObjectTouchedEvent(ObjectEvent):
    @property
    def object(self) -> Object: ...
    
    @property
    def player(self) -> Player: ...
    
    def __repr__(self) -> str: ...