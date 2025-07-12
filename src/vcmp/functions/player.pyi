from typing import Optional

from vcmp.functions.object import Object
from vcmp.functions.vehicle import Vehicle
from vcmp.streams import WriteStream
from vcmp.types import RGB, Vector

class Player:
    def __hash__(self) -> int: ...
    def __eq__(self, other: object) -> bool: ...

    @property
    def id(self) -> int:
        ...
    
    @property
    def action(self) -> int:
        ...
    
    @property
    def admin(self) -> bool:
        ...
    
    @admin.setter
    def admin(self, admin: bool) -> None:
        ...
    
    @property
    def aim_direction(self) -> Vector:
        ...
    
    @property
    def aim_position(self) -> Vector:
        ...
    
    @property
    def alpha(self) -> int:
        ...
    
    @property
    def angle(self) -> float:
        ...
    
    @angle.setter
    def angle(self, angle: float) -> None:
        ...
    
    @property
    def armour(self) -> float:
        ...
    
    @armour.setter
    def armour(self, armour: float) -> None:
        ...
    
    @property
    def away(self) -> bool:
        ...
    
    @property
    def camera_locked(self) -> bool:
        ...
    
    @property
    def can_attack(self) -> bool:
        ...
    
    @can_attack.setter
    def can_attack(self, can_attack: bool) -> None:
        ...
    
    @property
    def cash(self) -> int:
        ...
    
    @cash.setter
    def cash(self, cash: int) -> None:
        ...
    
    @property
    def chat_tags_enabled(self) -> bool:
        ...
    
    @chat_tags_enabled.setter
    def chat_tags_enabled(self, chat_tags_enabled: bool) -> None:
        ...
    
    @property
    def class_id(self) -> int:
        ...
    
    @property
    def color(self) -> RGB:
        ...
    
    @color.setter
    def color(self, value: RGB) -> None:
        ...
    
    @property
    def controllable(self) -> bool:
        ...
    
    @controllable.setter
    def controllable(self, controllable: bool) -> None:
        ...
    
    @property
    def frozen(self) -> bool:
        ...
    
    @frozen.setter
    def frozen(self, frozen: bool) -> None:
        ...
    
    @property
    def crouching(self) -> bool:
        ...
    
    @property
    def drive_by(self) -> bool:
        ...
    
    @drive_by.setter
    def drive_by(self, drive_by: bool) -> None:
        ...
    
    @property
    def drunk_effects(self) -> bool:
        ...
    
    @drunk_effects.setter
    def drunk_effects(self, drunk_effects: bool) -> None:
        ...
    
    @property
    def fps(self) -> float:
        ...
    
    @property
    def game_keys(self) -> int:
        ...
    
    @property
    def weapon(self) -> int:
        ...
    
    @property
    def weapon_ammo(self) -> int:
        ...
    
    @property
    def weapon_slot(self) -> int:
        ...
    
    @property
    def green_scanlines(self) -> bool:
        ...
    
    @green_scanlines.setter
    def green_scanlines(self, green_scanlines: bool) -> None:
        ...
    
    @property
    def has_marker(self) -> bool:
        ...
    
    @has_marker.setter
    def has_marker(self, has_marker: bool) -> None:
        ...
    
    @property
    def health(self) -> float:
        ...
    
    @health.setter
    def health(self, health: float) -> None:
        ...
    
    @property
    def immunity(self) -> int:
        ...
    
    @immunity.setter
    def immunity(self, flags: int) -> None:
        ...
    
    @property
    def ip(self) -> str:
        ...
    
    @property
    def is_alive(self) -> bool:
        ...
    
    @property
    def key(self) -> int:
        ...
    
    @property
    def name(self) -> str:
        ...
    
    @name.setter
    def name(self, name: str) -> None:
        ...
    
    @property
    def on_fire(self) -> bool:
        ...
    
    @property
    def ping(self) -> int:
        ...
    
    @property
    def position(self) -> Vector:
        ...
    
    @position.setter
    def position(self, position: Vector) -> None:
        ...
    
    @property
    def score(self) -> int:
        ...
    
    @score.setter
    def score(self, score: int) -> None:
        ...
    
    @property
    def sec_world(self) -> int:
        ...
    
    @sec_world.setter
    def sec_world(self, sec_world: int) -> None:
        ...
    
    @property
    def world(self) -> int:
        ...
    
    @world.setter
    def world(self, world: int) -> None:
        ...
    
    @property
    def unique_world(self) -> int:
        ...
    
    @property
    def speed(self) -> Vector:
        ...
    
    @speed.setter
    def speed(self, speed: Vector) -> None:
        ...
    
    @property
    def show_markers(self) -> bool:
        ...
    
    @show_markers.setter
    def show_markers(self, show_markers: bool) -> None:
        ...
    
    @property
    def skin(self) -> int:
        ...
    
    @skin.setter
    def skin(self, skin: int) -> None:
        ...
    
    @property
    def spawned(self) -> bool:
        ...
    
    @property
    def spectate_target(self) -> Optional[Player]:
        ...
    
    @property
    def standing_on_object(self) -> Optional[Object]:
        ...
    
    @property
    def standing_vehicle(self) -> Optional[Vehicle]:
        ...
    
    @property
    def state(self) -> int:
        ...
    
    @property
    def team(self) -> int:
        ...
    
    @team.setter
    def team(self, team: int) -> None:
        ...
    
    @property
    def is_typing(self) -> bool:
        ...
    
    @property
    def unique_id(self) -> str:
        ...
    
    @property
    def unique_id2(self) -> str:
        ...
    
    @property
    def vehicle(self) -> Optional[Vehicle]:
        ...
    
    @vehicle.setter
    def vehicle(self, vehicle: Optional[Vehicle]) -> None:
        ...
    
    @property
    def vehicle_status(self) -> int:
        ...
    
    @property
    def wanted_level(self) -> int:
        ...
    
    @wanted_level.setter
    def wanted_level(self, wanted_level: int) -> None:
        ...
    
    @property
    def white_scanlines(self) -> bool:
        ...
    
    @white_scanlines.setter
    def white_scanlines(self, white_scanlines: bool) -> None:
        ...
    
    @property
    def widescreen(self) -> bool:
        ...
    
    @widescreen.setter
    def widescreen(self, widescreen: bool) -> None:
        ...
    
    @property
    def bleeding(self) -> bool:
        ...
    
    @bleeding.setter
    def bleeding(self, bleeding: bool) -> None:
        ...

    # 玩家操作方法
    def add_position(self, pos: Vector) -> None:
        ...
    
    def add_speed(self, speed: Vector) -> None:
        ...
    
    def set_alpha(self, alpha: int, fade_time: int) -> None:
        ...
    
    def ban(self, message: Optional[str] = None) -> None:
        ...
    
    def clear_weapons(self) -> None:
        ...
    
    def disarm(self) -> None:
        ...
    
    def get_weapon_ammo_at_slot(self, slot: int) -> int:
        ...
    
    def get_weapon_at_slot(self, slot: int) -> int:
        ...
    
    def give_weapon(self, weapon: int, ammo: int) -> None:
        ...
    
    def is_streamed_for_target(self, player: int) -> bool:
        ...
    
    def kick(self, message: Optional[str] = None) -> None:
        ...
    
    def kill(self) -> None:
        ...
    
    def play_animation(self, group_id: int, animation_id: int) -> None:
        ...
    
    def play_sound(self, sound: int, position: Optional[Vector] = None) -> None:
        ...
    
    def redirect(self, ip: str, port: int, nick: str, password: str, user_password: str) -> None:
        ...
    
    def remove_weapon(self, weapon: int) -> None:
        ...
    
    def request_module_list(self) -> None:
        ...
    
    def restore_camera(self) -> None:
        ...
    
    def select(self) -> None:
        ...
    
    def send_data(self, data: WriteStream) -> None:
        ...
    
    def send_raw_message(self, color: RGB, message: str) -> None:
        ...
    
    def send_message(self, message: str) -> None:
        ...
    
    def send_announce(self, announce_type: int, message: str) -> None:
        ...
    
    def set_camera_position(self, position: Vector, look_at: Vector) -> None:
        ...
    
    def set_camera(self, position: Vector, look_yaw: float, look_pitch: float, range: Optional[float] = None) -> None:
        ...
    
    def set_vehicle_slot(self, vehicle: Optional[Vehicle], slot: int) -> None:
        ...
    
    def set_weapon(self, weapon: int, ammo: int) -> None:
        ...
    
    def set_weapon_slot(self, slot: int) -> None:
        ...
    
    def spawn(self) -> None:
        ...
    
    def is_world_compatible(self, world: int) -> bool:
        ...

    @property
    def drunk_handling(self) -> int:
        ...

    @drunk_handling.setter
    def drunk_handling(self, drunk_handling: int) -> None:
        ...

    @property
    def drunk_visuals(self) -> bool:
        ...

    @drunk_visuals.setter
    def drunk_visuals(self, drunk: bool) -> None:
        ...

    def set_3d_arrow_for_target(self, target: Player, show: bool):
        ...

    def is_3d_arrow_show_for_target(self, target: Player) -> bool:
        ...

    def interpolate_camera_look_at(self, look_at: Vector, time: int) -> None:
        ...