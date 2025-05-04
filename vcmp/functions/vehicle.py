from ..util import get_vehicle_random_color
from ..__export import funcs
from ..vec import Vector
from ..instance import Vehicle


def create_vehicle(
    model: int,
    world: int,
    pos: Vector,
    angle: float,
    primary_color: int = -1,
    secondary_color: int = -1
):
    """
        Creates a vehicle.

        :param model: The model of the vehicle.
        :param world: The world to create the vehicle in.
        :param pos: The position to create the vehicle at.
        :param angle: The angle to create the vehicle at.
        :param primary_color: The primary color of the vehicle.
        :param secondary_color: The secondary color of the vehicle.
        :return: The vehicle.
    """
    veh_id = funcs.create_vehicle(
        model,
        world,
        pos.x,
        pos.y,
        pos.z,
        angle,
        get_vehicle_random_color(primary_color),
        get_vehicle_random_color(secondary_color)
    )

    return Vehicle(veh_id)