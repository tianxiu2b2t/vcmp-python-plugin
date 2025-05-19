from vcmp.__export import funcs
from vcmp.vec import Vector
from vcmp.instance import Pickup

def create_pickup(
    model: int,
    world: int,
    quantity: int,
    pos: Vector,
    alpha: int,
    automatic: bool
):
    """
    Creates a pickup.

    :param model: The model of the pickup.
    :param world: The world the pickup will be in.
    :param quantity: The quantity of the pickup.
    :param pos: The position of the pickup.
    :param alpha: The alpha of the pickup.
    :param automatic: Whether the pickup is automatic or not.
    :return: The pickup ID.
    """
    id = funcs.create_pickup(model, world, quantity, pos.x, pos.y, pos.z, alpha, automatic)
    return Pickup(id)