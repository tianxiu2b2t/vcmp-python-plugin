from vcmp.__export import vcmpVehicleUpdate
from .abc import Event

class VehicleEvent(Event):
    ...

class VehicleUpdateEvent(VehicleEvent):
    __fields__ = (
        "vehicleid",
        "updatetype",
    )
    vehicleid: int
    updatetype: vcmpVehicleUpdate

class VehicleExplodeEvent(VehicleEvent):
    __fields__ = (
        "vehicleid",
    )
    vehicleid: int

class VehicleRespawnEvent(VehicleEvent):
    __fields__ = (
        "vehicleid",
    )
    vehicleid: int