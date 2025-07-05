from vcmp.functions.player import Player
from vcmp.types import RGB


WEAPON_NAMES: dict[int, str] = ...
WEAPON_MODELS: dict[int, int] = ...

VEHICLE_NAMES: dict[int, str] = ...
VEHICLE_CLASSIC_CAR: list[int] = ...
VEHICLE_CLASSIC_BOAT: list[int] = ...
VEHICLE_CLASSIC_AIR: list[int] = ...
VEHICLE_CLASSIC_BIKE: list[int] = ...
VEHICLE_CLASSIC_RC: list[int] = ...

SKINS: dict[int, str] = ...

def distance_from_point(
    x: float,
    y: float,
    x1: float,
    y1: float,
) -> float:
    """Returns the distance between two points."""
    ...

def get_district_name(
    x: float,
    y: float,
) -> str:
    """Returns the district name of the given coordinates."""
    ...

def in_poly(
    x: float,
    y: float,
    *polies: tuple[float, float],
) -> bool:
    """Returns True if the given point is inside the polygon."""
    ...

def get_players() -> list[Player]:
    ...

def announce_all(
    announce_type: int,
    message: str
):
    ...

def message_all(message: str):
    ...

def raw_message_all(
    color: RGB,
    message: str
):
    ...