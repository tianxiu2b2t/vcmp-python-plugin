from dataclasses import dataclass
from typing import Optional
from .__export import calls, funcs

@dataclass
class RGB:
    red: int
    green: int
    blue: int
    alpha: int = 255

    def to_int(self) -> int:
        return (self.red << 16) | (self.green << 8) | self.blue
    
    def to_alpha(self) -> int:
        return (self.red << 24) | (self.green << 16) | (self.blue << 8) | self.alpha
    
    def to_hex(self) -> str:
        return f"#{self.red:02x}{self.green:02x}{self.blue:02x}"
    
    def to_alpha_hex(self) -> str:
        return f"#{self.red:02x}{self.green:02x}{self.blue:02x}{self.alpha:02x}"
    
    @staticmethod
    def from_int(value: int) -> "RGB":
        # if is rgba
        if value > 0xFFFFFF:
            return RGB(
                red=(value >> 24) & 0xFF,
                green=(value >> 16) & 0xFF,
                blue=(value >> 8) & 0xFF,
                alpha=value & 0xFF
            )
        else:
            return RGB(
                red=(value >> 16) & 0xFF,
                green=(value >> 8) & 0xFF,
                blue=value & 0xFF
            )
    
    
    

class Player:
    def __init__(
        self,
        id: int
    ):
        self._id: int = id

    # use id as hash
    def __hash__(self) -> int:
        return self._id

    @property
    def id(self) -> int:
        return self._id

    @property
    def is_alive(self) -> bool:
        return funcs.is_player_connected(self._id)
    
    @property
    def name(self) -> str:
        return funcs.get_player_name(self._id)
    
    @name.setter
    def name(self, name: str):
        funcs.set_player_name(self._id, name)

    @property
    def ip(self) -> str:
        return funcs.get_player_ip(self._id)

    @property
    def ping(self) -> int:
        return funcs.get_player_ping(self._id)
    
    @property
    def team(self) -> int:
        return funcs.get_player_team(self._id)

    @team.setter
    def team(self, team: int):
        funcs.set_player_team(self._id, team)

    @property
    def score(self) -> int:
        return funcs.get_player_score(self._id)

    @score.setter
    def score(self, score: int):
        funcs.set_player_score(self._id, score)
    
    @property
    def color(self) -> RGB:
        return RGB.from_int(funcs.get_player_colour(self._id))

    @color.setter
    def color(self, color: RGB):
        funcs.set_player_colour(self._id, color.to_alpha())

    def kick(self, reason: Optional[str] = None):
        if reason is not None:
            self.send_message(reason)
        funcs.kick_player(self._id)

    def send_message(self, message: str):
        funcs.send_client_message(self._id, RGB(0x0b, 0x5f, 0xa5).to_alpha(), message)

    def send_announce(self, type: int, message: str):
        funcs.send_game_message(self._id, type, message)

    
_players: list[Player] = []

def get_player_from_id(
    player_id: int
) -> Optional[Player]:
    return next((player for player in _players if player.id == player_id), None)

def _on_pre_player_connect(
    player_id: int
):
    _players.append(Player(player_id))

def _on_post_player_disconnect(
    player_id: int,
    **args
):
    instance = next((player for player in _players if player.id == player_id), None)
    if instance is not None:
        _players.remove(instance)

setattr(calls, "on_pre_player_connect", _on_pre_player_connect)
setattr(calls, "on_post_player_connect", _on_post_player_disconnect)