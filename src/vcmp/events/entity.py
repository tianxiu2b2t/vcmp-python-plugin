from vcmp.__export import vcmpEntityPool
from .abc import Event

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

