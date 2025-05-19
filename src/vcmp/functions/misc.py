import math
from typing import Optional
from vcmp.vec import Vector
from vcmp.__export import funcs

def create_explosion(
    world_id: int,
    type: int,
    pos: Vector,
    target: Optional[int] = None,
    ground: bool = False
):
    """
        Creates an explosion at the given position.
        :param world_id: The world id.
        :param type: The type of explosion.
        :param pos: The position of the explosion.
        :param target: The responsible player id, can be null.
        :param ground: Whether the explosion should be on the ground.
    """
    target = target if target is not None else -1
    if target != -1 and not funcs.is_player_connected(target):
        target = -1
    funcs.create_explosion(world_id, type, pos.x, pos.y, pos.z, target, ground)

def play_sound(
    world_id: int,
    sound_Id: int,
    pos: Vector,
):
    """
        Plays a sound at the given position.
        :param world_id: The world id.
        :param sound_Id: The sound id.
        :param pos: The position of the sound.
    """
    funcs.play_sound(world_id, sound_Id, pos.x, pos.y, pos.z)

def hide_map_object(
    object_id: int,
    pos: Vector,
):
    """
        Hides a map object.
        :param object_id: The object id.
        :param pos: The position of the object.
    """
    # from squirrel plugin
    # (floor( x * 10.0f ) + 0.5f)
    x = int(math.floor( pos.x * 10.0) + 0.5)
    y = int(math.floor( pos.y * 10.0) + 0.5)
    z = int(math.floor( pos.z * 10.0) + 0.5)

    funcs.hide_map_object(object_id, x, y, z)

def show_map_object(
    object_id: int,
    pos: Vector,
):
    """
        Shows a map object.
        :param object_id: The object id.
        :param pos: The position of the object.
    """
    # from squirrel plugin
    # (floor( x * 10.0f ) + 0.5f)
    x = int(math.floor( pos.x * 10.0) + 0.5)
    y = int(math.floor( pos.y * 10.0) + 0.5)
    z = int(math.floor( pos.z * 10.0) + 0.5)

    funcs.show_map_object(object_id, x, y, z)

def show_all_map_objects():
    """
        Shows all map objects.
    """
    funcs.show_all_map_objects()

def add_radio_stream(
    id: int,
    name: str,
    url: str,
    can_select: bool
):
    """
        Adds a radio stream.
        :param id: The id of the stream.
        :param name: The name of the stream.
        :param url: The url of the stream.
        :param can_select: Whether the stream can be selected.
    """
    try:
        funcs.add_radio_stream(id, name, url, can_select)
        return True
    except:
        pass
    return False

def remove_radio_stream(
    id: int
):
    """
        Removes a radio stream.
        :param id: The id of the stream.
    """
    try:
        funcs.remove_radio_stream(id)
        return True
    except:
        pass
    return False

def get_last_error():
    """
        Returns the last error.
    """
    return funcs.get_last_error()