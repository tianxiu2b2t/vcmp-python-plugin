from dataclasses import dataclass

from ..__export import funcs
from .common import get_player_ids

def announce_all(
    message: str,
    type: int
) -> None:
    
    for id in get_player_ids():
        funcs.send_game_message(id, type, message)