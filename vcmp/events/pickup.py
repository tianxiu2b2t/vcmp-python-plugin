from .abc import Event

class PickupEvent(Event):
	...

class PickupPickAttemptEvent(PickupEvent):
    __fields__ = (
        "pickupid",
        "id",
    )
    pickupid: int
    id: int

class PickupPickedEvent(PickupEvent):
    __fields__ = (
        "pickupid",
        "id",
    )
    pickupid: int
    id: int

class PickupRespawnEvent(PickupEvent):
    __fields__ = (
        "pickupid",
    )
    pickupid: int