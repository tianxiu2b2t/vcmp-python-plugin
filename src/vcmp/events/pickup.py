from vcmp.events.abc import Event
from vcmp.instance import Pickup, get_player_from_id

class PickupEvent(Event):
    def __init__(
        self,
        pickup_id: int,
        *args,
        **kwargs
    ):
        super().__init__(*args, **kwargs)

        self.pickup_id = pickup_id
        
        pickup = Pickup(pickup_id)
        
        assert pickup is not None, f"Pickup id {self.pickup_id} does not exist"
        self.pickup = pickup
        

class PickupPickEvent(PickupEvent):
    def __init__(
        self,
        pickup_id: int,
        player_id: int,
        *args,
        **kwargs
    ):
        super().__init__(pickup_id, *args, **kwargs)

        self.player_id = player_id

        player = get_player_from_id(player_id)
        assert player is not None, f"Player id {self.player_id} does not exist"
        self.player = player

class PickupPickAttemptEvent(PickupPickEvent):
    ...

class PickupPickedEvent(PickupPickEvent):
    ...

class PickupRespawnEvent(PickupEvent):
    ...