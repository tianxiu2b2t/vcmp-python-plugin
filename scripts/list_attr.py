import math
from typing import Optional


class Pickup:
    def __init__(self, pickup_id: int):
        self._id = pickup_id

    def __hash__(self) -> int:
        return hash(self._id)

    def __eq__(self, value: object) -> bool:
        if isinstance(value, Pickup):
            return self._id == value._id
        return False

    @property
    def id(self) -> int:
        return self._id

    @property
    def is_alive(self) -> bool:
        return funcs.check_entity_exists(vcmpEntityPool.vcmpEntityPoolPickup, self._id)

    @property
    def world(self):
        return funcs.get_pickup_world(self._id)
    
    @world.setter
    def world(self, world: int):
        funcs.set_pickup_world(self._id, world)

    @property
    def alpha(self):
        return funcs.get_pickup_alpha(self._id)

    @alpha.setter
    def alpha(self, alpha: int):
        funcs.set_pickup_alpha(self._id, alpha)

    @property
    def automatic(self):
        return funcs.is_pickup_automatic(self._id)

    @automatic.setter
    def automatic(self, automatic: bool):
        funcs.set_pickup_is_automatic(self._id, automatic)

    @property
    def timer(self):
        """
        Auto timer
        """
        return funcs.get_pickup_auto_timer(self._id)

    @timer.setter
    def timer(self, timer: int):
        """
        Auto timer
        """
        funcs.set_pickup_auto_timer(self._id, timer)

    @property
    def position(self):
        return Vector(
            **funcs.get_pickup_position(self._id)
        )
    
    @position.setter
    def position(self, position: 'Vector'):
        funcs.set_pickup_position(self._id, position.x, position.y, position.z)

    @property
    def model(self):
        return funcs.get_pickup_model(self._id)

    @property
    def quantity(self):
        return funcs.get_pickup_quantity(self._id)

    @property
    def single_use(self):
        return funcs.get_pickup_option(self._id, 0)
    
    @single_use.setter
    def single_use(self, single_use: bool):
        funcs.set_pickup_option(self._id, 0, single_use)

    def refresh(self):
        """
        Refresh the pickup
        """
        funcs.refresh_pickup(self._id)
        
    def is_streamed_for_player(self, player: 'Player') -> bool:
        """
        Check if the pickup is streamed for the player
        """
        id = player if isinstance(player, int) else player.id
        return funcs.is_pickup_streamed_for_player(self._id, id)
    
    def add_position(self, position: 'Vector'):
        """
        Add position to the pickup position
        """
        new_pos = Vector(
            self.position.x + position.x,
            self.position.y + position.y,
            self.position.z + position.z
        )
        self.position = new_pos

    def delete(self):
        """
        Delete the pickup
        """
        if not funcs.check_entity_exists(vcmpEntityPool.vcmpEntityPoolPickup, self._id):
            return
        funcs.delete_pickup(self._id)
        _pickups.remove(self)

    def __del__(self):
        self.delete()

    def __new__(cls, pickup_id: int):
        if not funcs.check_entity_exists(vcmpEntityPool.vcmpEntityPoolPickup, pickup_id):
            return None
        pickup = next((pickup for pickup in _pickups if pickup.id == pickup_id), None)
        if pickup is None:
            pickup = super().__new__(cls)
            _pickups.append(pickup)
        return pickup


import inspect

for member in inspect.getmembers(object=Pickup):
    name, value = member
    if name.startswith("__") and name.endswith("__"):
        continue
    # if value is function
    if inspect.isfunction(value):
        print(f"fn {name}")
    else:
        print(f"fn prop_{name}")