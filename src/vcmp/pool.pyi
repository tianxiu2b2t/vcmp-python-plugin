from typing import Optional
from vcmp.functions.player import Player
from vcmp.functions.vehicle import Vehicle
from vcmp.functions.pickup import Pickup
from vcmp.functions.checkpoint import CheckPoint
from vcmp.functions.marker import Marker
from vcmp.functions.object import Object

def get_players(all: bool = False) -> list[Player]:
    """
    Get all players in the server.

    :param all: If True, returns all players, including fake disconnected ones. Default is False.
    :type all: bool, optional
    :return: A list of players.
    :rtype: list[Player]
    """

def get_vehicles() -> list[Vehicle]: ...
def get_pickups() -> list[Pickup]: ...
def get_checkpoints() -> list[CheckPoint]: ...
def get_markers() -> list[Marker]: ...
def get_objects() -> list[Object]: ...
def clear_vehicles() -> int: ...
def clear_pickups() -> int: ...
def clear_checkpoints() -> int: ...
def clear_markers() -> int: ...
def clear_objects() -> int: ...
def clear_all() -> int: ...
def find_player(value: str | int) -> Optional[Player]: ...
