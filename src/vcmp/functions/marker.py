from vcmp.__export import funcs
from vcmp.vec import Vector
from vcmp.instance import RGB, Marker

def create_marker(
    model: int,
    world: int,
    pos: Vector,
    scale: int,
    color: RGB
):
    """

    Create a marker

    :param model: The model of the object.
    :param world: The world to create the object in.
    :param pos: The position of the object.
    :param alpha: The alpha of the object.
    :return: The object created.
    """
    return Marker(
        funcs.create_coord_blip(-1, world, pos.x, pos.y, pos.z, scale, color.to_alpha(), model)
    )