from typing import Optional

from .player import Player
from vcmp.types import Vector

def create_explosion(
    world_id: int,
    explosion_type: int,
    pos: Vector,
    target: Optional[Player] = None,
    ground: bool = False,
) -> None: ...

def play_sound(
    world_id: int,
    sound_id: int,
    pos: Vector,
) -> None: ...

def hide_map_object(
    object_id: int,
    pos: Vector,
) -> None: ...

def show_map_object(
    object_id: int,
    pos: Vector,
) -> None: ...

def show_all_map_objects() -> None: ...

def add_radio_stream(
    id: int,
    name: str,
    url: str,
    can_select: bool,
) -> None: ...

def remove_radio_stream(id: int) -> None: ...