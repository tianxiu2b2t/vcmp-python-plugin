from . import abc
from . import checkpoint
from . import object
from . import pickup
from . import player
from . import server
from . import vehicle
from . import custom

from typing import Any, Optional, TYPE_CHECKING
from traceback import TracebackException

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
        checkpoint: CheckPoint,
        player: Player,  # noqa: F811
    ) -> "EventBuilder": ...
    @staticmethod
    def checkpoint_exited(checkpoint: CheckPoint, player: Player) -> "EventBuilder": ...  # noqa: F811

    # Object events
    @staticmethod
    def object_shot(
        object: Object,
        player: Player,
        weapon_id: int,  # noqa: F811
    ) -> "EventBuilder": ...
    @staticmethod
    def object_touched(object: Object, player: Player) -> "EventBuilder": ...  # noqa: F811

    # Pickup events
    @staticmethod
    def pickup_pick_attempt(pickup: Pickup, player: Player) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def pickup_picked(pickup: Pickup, player: Player) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def pickup_respawn(pickup: Pickup) -> "EventBuilder": ...  # noqa: F811

    # Player events
    @staticmethod
    def incoming_connection(
        ip: str, player_name: str, password: str
    ) -> "EventBuilder": ...
    @staticmethod
    def client_script_data(player: Player, data: bytes) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def player_connect(player: Player) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def player_disconnect(player: Player, reason: int) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def player_request_class(player: Player, class_id: int) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def player_spawn(player: Player) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def player_request_spawn(player: Player) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def player_death(
        player: Player,
        killer: Optional[Player],
        reason: int,
        body: int,  # noqa: F811
    ) -> "EventBuilder": ...
    @staticmethod
    def player_update(player: Player, update: int) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def player_request_enter_vehicle(
        player: Player,
        vehicle: Vehicle,
        slot_index: int,  # noqa: F811
    ) -> "EventBuilder": ...
    @staticmethod
    def player_enter_vehicle(
        player: Player,
        vehicle: Vehicle,
        slot_index: int,  # noqa: F811
    ) -> "EventBuilder": ...
    @staticmethod
    def player_exit_vehicle(player: Player, vehicle: Vehicle) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def player_name_change(
        player: Player,
        old_name: str,
        new_name: str,  # noqa: F811
    ) -> "EventBuilder": ...
    @staticmethod
    def player_state_change(
        player: Player,
        old_state: int,
        new_state: int,  # noqa: F811
    ) -> "EventBuilder": ...
    @staticmethod
    def player_action_change(
        player: Player,
        old_action: int,
        new_action: int,  # noqa: F811
    ) -> "EventBuilder": ...
    @staticmethod
    def player_on_fire_change(player: Player, is_on_fire: bool) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def player_crouch_change(player: Player, is_crouching: bool) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def player_game_keys_change(
        player: Player,
        old_keys: int,
        new_keys: int,  # noqa: F811
    ) -> "EventBuilder": ...
    @staticmethod
    def player_begin_typing(player: Player) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def player_end_typing(player: Player) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def player_away_change(player: Player, is_away: bool) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def player_message(player: Player, message: str) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def player_command(player: Player, command: str, text: str) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def player_private_message(
        player: Player,
        target: Player,
        message: str,  # noqa: F811
    ) -> "EventBuilder": ...
    @staticmethod
    def player_key_bind_down(player: Player, bind_id: int) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def player_key_bind_up(player: Player, bind_id: int) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def player_spectate(player: Player, target: Optional[Player]) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def player_crash_report(player: Player, report: str) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def player_module_list(player: Player, modules: str) -> "EventBuilder": ...  # noqa: F811

    # Player extra events
    @staticmethod
    def player_health_change(
        player: Player,
        old_health: float,
        new_health: float,  # noqa: F811
    ) -> "EventBuilder": ...
    @staticmethod
    def player_armour_change(
        player: Player,
        old_armour: float,
        new_armour: float,  # noqa: F811
    ) -> "EventBuilder": ...
    @staticmethod
    def player_weapon_change(
        player: Player,
        old_weapon: int,
        new_weapon: int,  # noqa: F811
    ) -> "EventBuilder": ...
    @staticmethod
    def player_ammo_change(
        player: Player,
        old_ammo: int,
        new_ammo: int,  # noqa: F811
    ) -> "EventBuilder": ...
    @staticmethod
    def player_move(
        player: Player,
        old_position: Vector,
        new_position: Vector,  # noqa: F811
    ) -> "EventBuilder": ...

    # Vehicle events
    @staticmethod
    def vehicle_update(vehicle: Vehicle, update_type: int) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def vehicle_explode(vehicle: Vehicle) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def vehicle_respawn(vehicle: Vehicle) -> "EventBuilder": ...  # noqa: F811

    # Vehicle extra events
    @staticmethod
    def vehicle_move(
        vehicle: Vehicle,
        old_position: Vector,
        new_position: Vector,  # noqa: F811
    ) -> "EventBuilder": ...
    @staticmethod
    def vehicle_health_change(
        vehicle: Vehicle,
        old_health: float,
        new_health: float,  # noqa: F811
    ) -> "EventBuilder": ...
    @staticmethod
    def custom(**kwargs) -> "EventBuilder": ...  # noqa: F811
    @staticmethod
    def traceback(traceback: TracebackException) -> "EventBuilder": ...

__all__ = [
    "EventBuilder",
    "abc",
    "checkpoint",
    "custom",
    "object",
    "pickup",
    "player",
    "server",
    "vehicle",
]
