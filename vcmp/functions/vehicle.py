from ..util import get_vehicle_random_color
from ..__export import funcs
from ..vec import Vector


def create_vehicle(
    model: int,
    world: int,
    pos: Vector,
    angle: float,
    primary_color: int = -1,
    secondary_color: int = -1
):
    

    funcs.create_vehicle(
        model,
        world,
        pos.x,
        pos.y,
        pos.z,
        angle,
        get_vehicle_random_color(primary_color),
        get_vehicle_random_color(secondary_color)
    )