from vcmp.__export import vcmpEntityPool
from .abc import Event

class EntityPoolChangeEvent(Event):
    __fields__ = (
        "entitytype",
        "entityid",
        "isdeleted",
    )
    entitytype: vcmpEntityPool
    entityid: int
    isdeleted: int