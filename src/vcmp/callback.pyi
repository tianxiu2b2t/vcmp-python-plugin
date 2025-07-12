from typing import Callable, Optional, Any

from vcmp.events.abc import Event


class CallbackManager:
    def trigger(self, event: Event) -> Callable[..., Any]: ...
    
    def on_server_initialise(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_server_shutdown(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_server_performance_report(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_server_frame(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_incoming_connection(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_client_script_data(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_connect(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_disconnect(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_request_class(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_spawn(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_request_spawn(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_death(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_update(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_request_enter_vehicle(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_enter_vehicle(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_exit_vehicle(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_name_change(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_state_change(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_action_change(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_on_fire_change(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_crouch_change(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_game_keys_change(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_begin_typing(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_end_typing(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_away_change(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_message(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_command(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_private_message(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_key_bind_down(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_key_bind_up(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_spectate(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_crash_report(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_module_list(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_health_change(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_armour_change(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_weapon_change(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_ammo_change(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_player_move(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_pickup_pick_attempt(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_pickup_picked(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_pickup_respawn(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_checkpoint_entered(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_checkpoint_exited(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_object_shot(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_object_touched(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_vehicle_explode(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_vehicle_respawn(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_vehicle_update(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_vehicle_move(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...
    
    def on_vehicle_health_change(
        self,
        priority: int = 9999,
        func: Optional[Callable[..., Any]] = None
    ) -> Callable[..., Any]: ...

callbacks: CallbackManager = ...