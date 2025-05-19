from vcmp.types import vcmpVehicleUpdate
from vcmp.events.abc import Event
from vcmp.instance import get_vehicle_from_id

class VehicleEvent(Event):
    def __init__(
        self,
        vehicle_id: int,
        *args,
        **kwargs
    ):
        super().__init__(*args, **kwargs)
        self.vehicle_id = vehicle_id
        
        vehicle = get_vehicle_from_id(vehicle_id)
        assert vehicle is not None, f"Vehicle id {vehicle_id} does not exists."

        self.vehicle = vehicle

class VehicleUpdateEvent(VehicleEvent):
    def __init__(
        self,
        vehicle_id: int,
        type: int
    ):
        super().__init__(vehicle_id)

        self.type = vcmpVehicleUpdate(type)

class VehicleExplodeEvent(VehicleEvent):
    ...

class VehicleRespawnEvent(VehicleEvent):
    ...