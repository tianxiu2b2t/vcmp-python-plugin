from dataclasses import dataclass
import io
import struct
from typing import Optional

from vcmp.vec import Vector
from .__export import calls, funcs, vcmpEntityPool, vcmpPlayerOption, vcmpPlayerState

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
    
class Stream:
    def __init__(
        self
    ):
        self._buffer = io.BytesIO()

class WriteStream(Stream):
    def write(self, data: bytes):
        self._buffer.write(data)

    def write_long(self, value: int):
        datum = (value << 1) ^ (value >> 63)
        while (datum & ~0x7F) != 0:
            self.write(bytearray([(datum & 0x7F) | 0x80]))
            datum >>= 7
        self.write(bytearray([datum]))

    def write_string(self, value: str, encoding = "gbk"):
        data = value.encode(encoding)
        self.write_long(len(data))
        self.write(data)

    def write_boolean(self, value: bool):
        self.write(bytearray([1 if value else 0]))

    def write_float(self, value: float):
        self.write(struct.pack("f", value))
    
class ReadStream(Stream):
    def read(self, length: int) -> bytes:
        return self._buffer.read(length)

    def read_long(self) -> int:
        b = ord(self.read(1))
        n = b & 0x7F
        shift = 7
        while (b & 0x80) != 0:
            b = ord(self.read(1))
            n |= (b & 0x7F) << shift
            shift += 7
        datum = (n >> 1) ^ -(n & 1)
        return datum
    
    def read_string(self, encoding = "gbk") -> str:
        length = self.read_long()
        return self.read(length).decode(encoding)
        
    def read_boolean(self) -> bool:
        return self.read(1)[0] != 0

    def read_float(self) -> float:
        return struct.unpack("f", self.read(4))[0]
    

    

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
    def admin(self):
        return funcs.is_player_admin(self._id)

    @admin.setter
    def admin(self, admin: bool):
        funcs.set_player_admin(self._id, admin)
    
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
    def unique_id(self) -> str:
        return funcs.get_player_uid(self._id)
    
    @property
    def unique_id2(self) -> str:
        return funcs.get_player_uid2(self._id)

    @property
    def ping(self) -> int:
        return funcs.get_player_ping(self._id)
    
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

    @property
    def key(self):
        """
            Player key (I didn't know what it is)
        """
        return funcs.get_player_key(self._id)

    @property
    def state(self):
        state = funcs.get_player_state(self._id)
        return vcmpPlayerState(state)
    

        #  Player Options

    @property
    def controllable(self) -> bool:
        return funcs.get_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionControllable)

    @controllable.setter
    def controllable(self, controllable: bool):
        funcs.set_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionControllable, controllable)

    @property
    def drive_by(self) -> bool:
        return funcs.get_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionDriveBy)

    @drive_by.setter
    def drive_by(self, drive_by: bool):
        funcs.set_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionDriveBy, drive_by)

    @property
    def white_scanlines(self) -> bool:
        return funcs.get_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionWhiteScanlines)

    @white_scanlines.setter
    def white_scanlines(self, white_scanlines: bool):
        funcs.set_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionWhiteScanlines, white_scanlines)

    @property
    def green_scanlines(self) -> bool:
        return funcs.get_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionGreenScanlines)

    @green_scanlines.setter
    def green_scanlines(self, green_scanlines: bool):
        funcs.set_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionGreenScanlines, green_scanlines)

    @property
    def widescreen(self) -> bool:
        return funcs.get_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionWidescreen)

    @widescreen.setter
    def widescreen(self, widescreen: bool):
        funcs.set_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionWidescreen, widescreen)

    @property
    def show_markers(self) -> bool:
        return funcs.get_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionShowMarkers)

    @show_markers.setter
    def show_markers(self, show_markers: bool):
        funcs.set_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionShowMarkers, show_markers)

    @property
    def can_attack(self) -> bool:
        return funcs.get_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionCanAttack)

    @can_attack.setter
    def can_attack(self, can_attack: bool):
        funcs.set_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionCanAttack, can_attack)

    @property
    def has_marker(self) -> bool:
        return funcs.get_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionHasMarker)

    @has_marker.setter
    def has_marker(self, has_marker: bool):
        funcs.set_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionHasMarker, has_marker)

    @property
    def chat_tags_enabled(self) -> bool:
        return funcs.get_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionChatTagsEnabled)
    
    @chat_tags_enabled.setter
    def chat_tags_enabled(self, chat_tags_enabled: bool):
        funcs.set_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionChatTagsEnabled, chat_tags_enabled)

    @property
    def drunk_effects(self) -> bool:
        return funcs.get_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionDrunkEffects)

    @drunk_effects.setter
    def drunk_effects(self, drunk_effects: bool):
        funcs.set_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionDrunkEffects, drunk_effects)

    @property
    def world(self) -> int:
        return funcs.get_player_world(self._id)
    
    @world.setter
    def world(self, world: int):
        funcs.set_player_world(self._id, world)

    @property
    def secondary_world(self) -> int:
        return funcs.get_player_secondary_world(self._id)

    @secondary_world.setter
    def secondary_world(self, secondary_world: int):
        funcs.set_player_secondary_world(self._id, secondary_world)

    @property
    def unique_world(self) -> int:
        return funcs.get_player_unique_world(self._id)

    @property
    def clazz(self) -> int:
        return funcs.get_player_class(self._id)

    @property
    def team(self) -> int:
        return funcs.get_player_team(self._id)

    @team.setter
    def team(self, team: int):
        funcs.set_player_team(self._id, team)

    @property
    def skin(self) -> int:
        return funcs.get_player_skin(self._id)

    @skin.setter
    def skin(self, skin: int):
        funcs.set_player_skin(self._id, skin)

    @property
    def spawned(self):
        return funcs.is_player_spawned(self._id)

    @property
    def typing(self):
        return funcs.is_player_typing(self._id)

    @property
    def cash(self):
        return funcs.get_player_money(self._id)

    @cash.setter
    def cash(self, cash: int):
        funcs.set_player_money(self._id, cash)

    @property
    def wanted_level(self):
        return funcs.get_player_wanted_level(self._id)

    @wanted_level.setter
    def wanted_level(self, wanted_level: int):
        funcs.set_player_wanted_level(self._id, wanted_level)

    @property
    def fps(self):
        return funcs.get_player_fps(self._id)
    
    @property
    def health(self):
        return funcs.get_player_health(self._id)

    @health.setter
    def health(self, health: int):
        funcs.set_player_health(self._id, health)

    @property
    def armour(self):
        return funcs.get_player_armour(self._id)

    @armour.setter
    def armour(self, armour: int):
        funcs.set_player_armour(self._id, armour)

    @property
    def immunity(self):
        return funcs.get_player_immunity_flags(self._id)
    
    @immunity.setter
    def immunity(self, immunity: int):
        funcs.set_player_immunity_flags(self._id, immunity)

    @property
    def position(self):
        return Vector(
            **funcs.get_player_position(self._id)
        )
    
    @position.setter
    def position(self, position: Vector):
        funcs.set_player_position(self._id, position.x, position.y, position.z)

    @property
    def speed(self):
        return Vector(
            **funcs.get_player_speed(self._id)
        )

    @speed.setter
    def speed(self, speed: Vector):
        funcs.set_player_speed(self._id, speed.x, speed.y, speed.z)

    @property
    def heading(self):
        return funcs.get_player_heading(self._id)
    
    @heading.setter
    def heading(self, heading: int):
        funcs.set_player_heading(self._id, heading)

    @property
    def alpha(self):
        return funcs.get_player_alpha(self._id)
    
    def set_alpha(self, alpha: int, fade: int):
        funcs.set_player_alpha(self._id, alpha, fade)

    @property
    def aim_position(self):
        return Vector(
            **funcs.get_player_aim_position(self._id)
        )
    
    @property
    def aim_direction(self):
        return Vector(
            **funcs.get_player_aim_direction(self._id)
        )
    
    @property
    def on_fire(self):
        """
        :return: True if the player is firing, not weapon firing
        """
        return funcs.is_player_on_fire(self._id)
    
    @property
    def crouching(self):
        return funcs.is_player_crouching(self._id)
    
    @property
    def action(self):
        return funcs.get_player_action(self._id)
    
    @property
    def game_keys(self):
        return funcs.get_player_game_keys(self._id)
    
    @property
    def vehicle(self):
        return Vehicle(funcs.get_player_vehicle_id(self._id))

    def kick(self, reason: Optional[str] = None):
        """
            Kick the player

            :param reason: The reason for the kick, optional
        """
        if reason is not None:
            self.send_message(reason)
        funcs.kick_player(self._id)

    def ban(self, reason: Optional[str] = None):
        """
            Ban the player

            :param reason: The reason for the ban, optional
        """
        if reason is not None:
            self.send_message(reason)
        funcs.ban_player(self._id)

    def send_message(self, message: str):
        """
        Send message to the player
        """
        funcs.send_client_message(self._id, RGB(0x0b, 0x5f, 0xa5).to_alpha(), message)

    def send_announce(self, type: int, message: str):
        """
        Send announce message to the player
        """
        funcs.send_game_message(self._id, type, message)

    def send_data(self, stream: WriteStream):
        """
        Send stream data to the player
        """
        buffer = stream._buffer.getvalue()
        funcs.send_client_script_data(self._id, buffer)

    def is_player_streamed_for_target(
        self,
        target: 'Player'
    ) -> bool:
        """
        Check if the player is streamed for the target player
        """
        return funcs.is_player_streamed_for_player(self._id, target.id)

    def spawn(self):
        """
        Spawn the player
        """
        funcs.force_player_spawn(self._id)

    def select(self):
        """
        Select the player
        """
        funcs.force_player_select(self._id)

class Vehicle:
    # if vehicle in _vehicles, use it, else create new
    def __init__(self, vehicle_id: int):
        self._id = vehicle_id


    @property
    def id(self):
        return self._id

    @property
    def model(self):
        return funcs.get_vehicle_model(self._id)

    def __new__(cls, vehicle_id: int):
        if not funcs.check_entity_exists(vcmpEntityPool.vcmpEntityPoolVehicle, vehicle_id):
            return None
        vehicle = next((vehicle for vehicle in _vehicles if vehicle.id == vehicle_id), None)
        if vehicle is None:
            vehicle = super().__new__(cls)
            _vehicles.append(vehicle)
        return vehicle


_players: list[Player] = []
_vehicles: list[Vehicle] = []

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