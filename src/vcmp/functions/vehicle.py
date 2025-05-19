from vcmp.util import get_vehicle_random_color
from vcmp.__export import funcs
from vcmp.vec import Vector
from vcmp.instance import Vehicle


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

def reset_all_vehicle_handlings():
    """
        Resets all vehicle handlings.
    """
    funcs.reset_all_vehicle_handlings()

def exists_handling_rule(model: int, rule: int):
    """
        Checks if a vehicle handling rule exists.

        :param model: The model of the vehicle.
        :param rule: The rule to check.
        :return: True if the rule exists, False otherwise.
    """
    return funcs.exists_handling_rule(model, rule)

def set_handling_rule(model: int, rule: int, value: float):
    """
        Sets a vehicle handling rule.

        :param model: The model of the vehicle.
        :param rule: The rule to set.
        :param value: The value to set the rule to.
    """
    funcs.set_handling_rule(model, rule, value)

def get_handling_rule(model: int, rule: int):
    """
        Gets a vehicle handling rule.

        :param model: The model of the vehicle.
        :param rule: The rule to get.
        :return: The value of the rule.
    """
    return funcs.get_handling_rule(model, rule)

def reset_handling_rule(model: int, rule: int):
    """
        Resets a vehicle handling rule.

        :param model: The model of the vehicle.
        :param rule: The rule to reset.
    """
    funcs.reset_handling_rule(model, rule)

def reset_handling(model: int):
    """
        Resets a vehicle handling.

        :param model: The model of the vehicle.
    """
    funcs.reset_handling(model)