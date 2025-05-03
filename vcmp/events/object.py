from .abc import Event

class ObjectEvent(Event):
    ...

class ObjectShotEvent(ObjectEvent):
    __fields__ = (
        "objectid",
        "id",
        "weaponid",
    )
    objectid: int
    id: int
    weaponid: int

class ObjectTouchedEvent(ObjectEvent):
    __fields__ = (
        "objectid",
        "id",
    )
    objectid: int
    id: int