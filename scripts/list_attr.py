import math
from typing import Optional


class CheckPoint:
    def __init__(self, id: int):
        self._id = id

    def __hash__(self) -> int:
        return hash(self._id)

    def __eq__(self, value: object) -> bool:
        if isinstance(value, CheckPoint):
            return self._id == value._id
        return False

    @property
    def id(self) -> int:
        return self._id

    @property
    def is_alive(self) -> bool:
        return funcs.check_entity_exists(
            vcmpEntityPool.vcmpEntityPoolCheckPoint, self._id
        )

    @property
    def sphere(self):
        return funcs.is_check_point_sphere(self._id)

    @property
    def world(self):
        return funcs.get_check_point_world(self._id)

    @world.setter
    def world(self, world: int):
        funcs.set_check_point_world(self._id, world)

    @property
    def position(self):
        return Vector(**funcs.get_check_point_position(self._id))

    @position.setter
    def position(self, position: "Vector"):
        funcs.set_check_point_position(self._id, position.x, position.y, position.z)

    @property
    def color(self) -> "RGB":
        return RGB(**funcs.get_check_point_colour(self._id))

    @color.setter
    def color(self, color: "RGB"):
        funcs.set_check_point_colour(
            self._id, color.red, color.green, color.blue, color.alpha
        )

    @property
    def radius(self):
        return funcs.get_check_point_radius(self._id)

    @radius.setter
    def radius(self, radius: float):
        funcs.set_check_point_radius(self._id, radius)

    @property
    def owner(self):
        return get_player_from_id(funcs.get_check_point_owner(self._id))

    def delete(self):
        """
        Delete the pickup
        """
        if not funcs.check_entity_exists(
            vcmpEntityPool.vcmpEntityPoolCheckPoint, self._id
        ):
            return
        funcs.delete_check_point(self._id)
        _checkpoints.remove(self)

    def is_streamed_for_player(self, player: "Player") -> bool:
        id = player if isinstance(player, int) else player.id
        return funcs.is_check_point_streamed_for_player(self._id, id)

    def add_position(self, position: "Vector"):
        """
        Add position to the pickup position
        """
        new_pos = Vector(
            self.position.x + position.x,
            self.position.y + position.y,
            self.position.z + position.z,
        )
        self.position = new_pos

    def __del__(self):
        self.delete()

    def __new__(cls, checkpoint_id: int):
        if not funcs.check_entity_exists(
            vcmpEntityPool.vcmpEntityPoolCheckPoint, checkpoint_id
        ):
            return None

        checkpoint = next(
            (
                checkpoint
                for checkpoint in _checkpoints
                if checkpoint.id == checkpoint_id
            ),
            None,
        )
        if checkpoint is None:
            checkpoint = super().__new__(cls)
            _checkpoints.append(checkpoint)
        return checkpoint


import inspect

for member in inspect.getmembers(object=CheckPoint):
    name, value = member
    if name.startswith("__") and name.endswith("__"):
        continue
    # if value is function
    if inspect.isfunction(value):
        print(f"fn {name}")
    else:
        print(f"fn prop_{name}")
