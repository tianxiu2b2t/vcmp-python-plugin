import math
from vcmp.__export import funcs

def play_sound_for_world(
    world_id: int,
    sound_id: int,
):
    """
    Play a sound for the entire world.

    :param world_id: The world ID.
    :param sound_id: The sound ID.
    """
    return funcs.play_sound(world_id, sound_id, math.nan, math.nan, math.nan)


def play_sound_for_position(
    world_id: int,
    sound_id: int,
    x: float,
    y: float,
    z: float,
):
    """
    Play a sound for a specific position in the world.

    :param world_id: The world ID.
    :param sound_id: The sound ID.
    :param x: The X coordinate.
    :param y: The Y coordinate.
    :param z: The Z coordinate.
    """
    return funcs.play_sound(world_id, sound_id, x, y, z)

