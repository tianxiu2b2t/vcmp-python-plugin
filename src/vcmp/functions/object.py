from vcmp.__export import funcs
from vcmp.vec import Vector
from vcmp.instance import Object

def create_object(
    model: int,
    world: int,
    pos: Vector,
    alpha: int
):
    """

    Creates an object.

    :param model: The model of the object.
    :param world: The world to create the object in.
    :param pos: The position of the object.
    :param alpha: The alpha of the object.
    :return: The object created.
    """
    return Object(funcs.create_object(model, world, pos.x, pos.y, pos.z, alpha))