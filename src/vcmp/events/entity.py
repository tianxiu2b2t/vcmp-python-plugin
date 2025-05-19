from vcmp.types import vcmpEntityPool
from vcmp.events.abc import Event

class EntityPoolChangeEvent(Event):
    def __init__(
        self,
        type: int,
        id: int,
        deleted: bool
    ):
        self.type = vcmpEntityPool(type)
        self.id = id
        self.deleted = deleted

