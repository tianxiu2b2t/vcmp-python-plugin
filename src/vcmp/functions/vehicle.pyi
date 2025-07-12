from vcmp.functions.player import Player
from vcmp.types import Quaternion, Vector


class Vehicle:
    def __hash__(self) -> int: ...
    def __eq__(self, other: 'Vehicle') -> bool: ...

    @property
    def id(self) -> int:
        ...
    
    @property
    def alarm(self) -> bool:
        ...
    
    @alarm.setter
    def alarm(self, alarm: bool) -> None:
        ...
    
    @property
    def color(self) -> tuple[int, int]:
        ...
    
    @color.setter
    def color(self, color: tuple[int, int]) -> None:
        ...
    
    @property
    def primary_color(self) -> int:
        ...
    
    @primary_color.setter
    def primary_color(self, color: int) -> None:
        ...
    
    @property
    def secondary_color(self) -> int:
        ...
    
    @secondary_color.setter
    def secondary_color(self, color: int) -> None:
        ...
    
    @property
    def damage(self) -> int:
        ...
    
    @damage.setter
    def damage(self, damage: int) -> None:
        ...
    
    @property
    def doors_locked(self) -> bool:
        ...
    
    @doors_locked.setter
    def doors_locked(self, doors_locked: bool) -> None:
        ...
    
    @property
    def ghost(self) -> bool:
        ...
    
    @ghost.setter
    def ghost(self, ghost: bool) -> None:
        ...
    
    @property
    def health(self) -> float:
        ...
    
    @health.setter
    def health(self, health: float) -> None:
        ...
    
    @property
    def idle_respawn_timer(self) -> int:
        ...
    
    @idle_respawn_timer.setter
    def idle_respawn_timer(self, idle_respawn_timer: int) -> None:
        ...
    
    @property
    def immunity(self) -> int:
        ...
    
    @immunity.setter
    def immunity(self, immunity: int) -> None:
        ...
    
    @property
    def is_alive(self) -> bool:
        ...
    
    @property
    def lights(self) -> bool:
        ...
    
    @lights.setter
    def lights(self, lights: bool) -> None:
        ...
    
    @property
    def lights_data(self) -> int:
        ...
    
    @lights_data.setter
    def lights_data(self, lights_data: int) -> None:
        ...
    
    @property
    def model(self) -> int:
        ...
    
    @property
    def position(self) -> Vector:
        ...
    
    @position.setter
    def position(self, position: Vector) -> None:
        ...
    
    @property
    def radio(self) -> int:
        ...
    
    @radio.setter
    def radio(self, radio: int) -> None:
        ...
    
    @property
    def radio_locked(self) -> bool:
        ...
    
    @radio_locked.setter
    def radio_locked(self, radio_locked: bool) -> None:
        ...
    
    @property
    def relative_speed(self) -> Vector:
        ...
    
    @relative_speed.setter
    def relative_speed(self, relative_speed: Vector) -> None:
        ...
    
    @property
    def relative_turn_speed(self) -> Vector:
        ...
    
    @relative_turn_speed.setter
    def relative_turn_speed(self, relative_turn_speed: Vector) -> None:
        ...
    
    @property
    def rotation(self) -> Quaternion:
        ...
    
    @rotation.setter
    def rotation(self, rotation: Quaternion) -> None:
        ...
    
    @property
    def rotation_euler(self) -> Vector:
        ...
    
    @rotation_euler.setter
    def rotation_euler(self, rotation_euler: Vector) -> None:
        ...
    
    @property
    def single_use(self) -> bool:
        ...
    
    @single_use.setter
    def single_use(self, single_use: bool) -> None:
        ...
    
    @property
    def siren(self) -> bool:
        ...
    
    @siren.setter
    def siren(self, siren: bool) -> None:
        ...
    
    @property
    def spawn_position(self) -> Vector:
        ...
    
    @property
    def spawn_rotation(self) -> Quaternion:
        ...
    
    @property
    def spawn_rotation_euler(self) -> Vector:
        ...
    
    @property
    def speed(self) -> Vector:
        ...
    
    @property
    def sync_source(self) -> int:
        ...
    
    @property
    def sync_type(self) -> int:
        ...
    
    @property
    def turn_speed(self) -> Vector:
        ...
    
    @turn_speed.setter
    def turn_speed(self, turn_speed: Vector) -> None:
        ...
    
    @property
    def turret_rotation(self) -> tuple[float, float]:
        ...
    
    @property
    def world(self) -> int:
        ...
    
    @property
    def wrecked(self) -> bool:
        ...

    # 操作方法
    def add_position(self, pos: Vector) -> None:
        ...
    
    def add_relative_speed(self, speed: Vector) -> None:
        ...
    
    def add_relative_turn_speed(self, speed: Vector) -> None:
        ...
    
    def add_rotation_euler(self, rotation: Vector) -> None:
        ...
    
    def add_rotation(self, rotation: Quaternion) -> None:
        ...
    
    def add_spawn_position(self, pos: Vector) -> None:
        ...
    
    def add_spawn_rotation_euler(self, rotation: Vector) -> None:
        ...
    
    def add_spawn_rotation(self, rotation: Quaternion) -> None:
        ...
    
    def add_speed(self, speed: Vector) -> None:
        ...
    
    def add_turn_speed(self, speed: Vector) -> None:
        ...
    
    def delete(self) -> None:
        ...
    
    def exists_handling_rule(self, rule_index: int) -> bool:
        ...
    
    def explode(self) -> None:
        ...
    
    def fix(self) -> None:
        ...
    
    def get_handling_rule(self, rule_index: int) -> float:
        ...
    
    def get_occupant(self, seat: int) -> int:
        ...
    
    def get_part_status(self, part: int) -> bool:
        ...
    
    def get_tyre_status(self, tyre: int) -> bool:
        ...
    
    def is_streamed_for_player(self, player: Player) -> bool:
        ...
    
    def kill(self) -> None:
        ...
    
    def set_position(self, position: Vector, remove_occupants: bool) -> None:
        ...
    
    def reset_handling(self) -> None:
        ...
    
    def reset_handling_rule(self, rule_index: int) -> None:
        ...
    
    def respawn(self) -> None:
        ...
    
    def set_handling_rule(self, rule_index: int, rule_value: float) -> None:
        ...
    
    def set_part_status(self, part_index: int, status: int) -> None:
        ...
    
    def set_tyre_status(self, tyre_index: int, status: int) -> None:
        ...
    
    def set_spawn_position(self, pos: Vector) -> None:
        ...
    
    def set_spawn_rotation(self, rot: Quaternion) -> None:
        ...
    
    def set_spawn_rotation_euler(self, rot: Vector) -> None:
        ...
    
    def set_speed(self, speed: Vector) -> None:
        ...

def create_vehicle(
    model: int,
    world: int,
    pos: Vector,
    angle: float = 0,
    primary_color: int = -1,
    secondary_color: int = -1,
) -> Vehicle:
    ...