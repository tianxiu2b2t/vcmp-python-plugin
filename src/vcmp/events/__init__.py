from . import abc
from . import checkpoint
from . import object
from . import pickup
from . import player
from . import server
from . import vehicle
from . import custom

from typing import Any, Optional, TYPE_CHECKING

if TYPE_CHECKING:
    from vcmp.types import Vector
    from vcmp.functions.checkpoint import CheckPoint
    from vcmp.functions.object import Object
    from vcmp.functions.pickup import Pickup
    from vcmp.functions.player import Player
    from vcmp.functions.vehicle import Vehicle


class EventBuilder:
    def with_kwargs(self, kwargs: dict[str, Any]) -> "EventBuilder": ...
    @staticmethod
    def server_initialise() -> "EventBuilder": ...
    @staticmethod
    def server_shutdown() -> "EventBuilder": ...
    @staticmethod
    def server_frame(elapsed_time: float) -> "EventBuilder": ...
    @staticmethod
    def server_performance_report(
        descriptions: list[str], times: list[int], entry_count: Optional[int] = None
    ) -> "EventBuilder": ...
    @staticmethod
    def server_reloaded(elapsed_time: float) -> "EventBuilder": ...

    # Checkpoint events
    @staticmethod
    def checkpoint_entered(
        checkpoint: CheckPoint, player: Player
    ) -> "EventBuilder": ...
    @staticmethod
    def checkpoint_exited(checkpoint: CheckPoint, player: Player) -> "EventBuilder": ...

    # Object events
    @staticmethod
    def object_shot(
        object: Object, player: Player, weapon_id: int
    ) -> "EventBuilder": ...
    @staticmethod
    def object_touched(object: Object, player: Player) -> "EventBuilder": ...

    # Pickup events
    @staticmethod
    def pickup_pick_attempt(pickup: Pickup, player: Player) -> "EventBuilder": ...
    @staticmethod
    def pickup_picked(pickup: Pickup, player: Player) -> "EventBuilder": ...
    @staticmethod
    def pickup_respawn(pickup: Pickup) -> "EventBuilder": ...

    # Player events
    @staticmethod
    def incoming_connection(
        ip: str, player_name: str, password: str
    ) -> "EventBuilder": ...
    @staticmethod
    def client_script_data(player: Player, data: bytes) -> "EventBuilder": ...
    @staticmethod
    def player_connect(player: Player) -> "EventBuilder": ...
    @staticmethod
    def player_disconnect(player: Player, reason: int) -> "EventBuilder": ...
    @staticmethod
    def player_request_class(player: Player, class_id: int) -> "EventBuilder": ...
    @staticmethod
    def player_spawn(player: Player) -> "EventBuilder": ...
    @staticmethod
    def player_request_spawn(player: Player) -> "EventBuilder": ...
    @staticmethod
    def player_death(
        player: Player, killer: Optional[Player], reason: int, body: int
    ) -> "EventBuilder": ...
    @staticmethod
    def player_update(player: Player, update: int) -> "EventBuilder": ...
    @staticmethod
    def player_request_enter_vehicle(
        player: Player, vehicle: Vehicle, slot_index: int
    ) -> "EventBuilder": ...
    @staticmethod
    def player_enter_vehicle(
        player: Player, vehicle: Vehicle, slot_index: int
    ) -> "EventBuilder": ...
    @staticmethod
    def player_exit_vehicle(player: Player, vehicle: Vehicle) -> "EventBuilder": ...
    @staticmethod
    def player_name_change(
        player: Player, old_name: str, new_name: str
    ) -> "EventBuilder": ...
    @staticmethod
    def player_state_change(
        player: Player, old_state: int, new_state: int
    ) -> "EventBuilder": ...
    @staticmethod
    def player_action_change(
        player: Player, old_action: int, new_action: int
    ) -> "EventBuilder": ...
    @staticmethod
    def player_on_fire_change(player: Player, is_on_fire: bool) -> "EventBuilder": ...
    @staticmethod
    def player_crouch_change(player: Player, is_crouching: bool) -> "EventBuilder": ...
    @staticmethod
    def player_game_keys_change(
        player: Player, old_keys: int, new_keys: int
    ) -> "EventBuilder": ...
    @staticmethod
    def player_begin_typing(player: Player) -> "EventBuilder": ...
    @staticmethod
    def player_end_typing(player: Player) -> "EventBuilder": ...
    @staticmethod
    def player_away_change(player: Player, is_away: bool) -> "EventBuilder": ...
    @staticmethod
    def player_message(player: Player, message: str) -> "EventBuilder": ...
    @staticmethod
    def player_command(player: Player, command: str, text: str) -> "EventBuilder": ...
    @staticmethod
    def player_private_message(
        player: Player, target: Player, message: str
    ) -> "EventBuilder": ...
    @staticmethod
    def player_key_bind_down(player: Player, bind_id: int) -> "EventBuilder": ...
    @staticmethod
    def player_key_bind_up(player: Player, bind_id: int) -> "EventBuilder": ...
    @staticmethod
    def player_spectate(player: Player, target: Optional[Player]) -> "EventBuilder": ...
    @staticmethod
    def player_crash_report(player: Player, report: str) -> "EventBuilder": ...
    @staticmethod
    def player_module_list(player: Player, modules: str) -> "EventBuilder": ...

    # Player extra events
    @staticmethod
    def player_health_change(
        player: Player, old_health: float, new_health: float
    ) -> "EventBuilder": ...
    @staticmethod
    def player_armour_change(
        player: Player, old_armour: float, new_armour: float
    ) -> "EventBuilder": ...
    @staticmethod
    def player_weapon_change(
        player: Player, old_weapon: int, new_weapon: int
    ) -> "EventBuilder": ...
    @staticmethod
    def player_ammo_change(
        player: Player, old_ammo: int, new_ammo: int
    ) -> "EventBuilder": ...
    @staticmethod
    def player_move(
        player: Player, old_position: Vector, new_position: Vector
    ) -> "EventBuilder": ...

    # Vehicle events
    @staticmethod
    def vehicle_update(vehicle: Vehicle, update_type: int) -> "EventBuilder": ...
    @staticmethod
    def vehicle_explode(vehicle: Vehicle) -> "EventBuilder": ...
    @staticmethod
    def vehicle_respawn(vehicle: Vehicle) -> "EventBuilder": ...

    # Vehicle extra events
    @staticmethod
    def vehicle_move(
        vehicle: Vehicle, old_position: Vector, new_position: Vector
    ) -> "EventBuilder": ...
    @staticmethod
    def vehicle_health_change(
        vehicle: Vehicle, old_health: float, new_health: float
    ) -> "EventBuilder": ...
    @staticmethod
    def custom(**kwargs) -> "EventBuilder": ...
