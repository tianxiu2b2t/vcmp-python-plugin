from dataclasses import dataclass

from vcmp.__utils import gbk_to_utf8, utf8_to_gbk

from ..__abc import funcs, vcmpServerOption

@dataclass
class ServerSettings:
    max_players: int
    port: int
    servername: str
    flags: int

def get_server_settings() -> ServerSettings: 
    return ServerSettings(**funcs.get_server_settings())
    
def get_server_version() -> int: 
    return funcs.get_server_version()

def set_servername(servername: str) -> None:
    funcs.set_server_name(servername) # type: ignore : vcmp server default use gbk encoding

def set_max_players(max_players: int) -> None:
    funcs.set_max_players(max(1, min(100, max_players)))

def set_gamemode(gamemode: str) -> None:
    funcs.set_game_mode_text(gamemode)

def get_servername() -> str:
    return funcs.get_server_name()

def get_max_players() -> int:
    return funcs.get_max_players()

def get_gamemode() -> str:
    return funcs.get_game_mode_text()

"""
Server Options
"""

def set_sync_frame_limiter(value: bool) -> None: 
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionSyncFrameLimiter, value)

def set_frame_limiter(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionFrameLimiter, value)

def set_taxi_boost_jump(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionTaxiBoostJump, value)

def set_drive_on_water(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionDriveOnWater, value)

def set_fast_switch(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionFastSwitch, value)

def set_friendly_fire(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionFriendlyFire, value)

def set_disable_drive_by(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionDisableDriveBy, value)

def set_perfect_handling(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionPerfectHandling, value)

def set_flying_cars(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionFlyingCars, value)

def set_jump_switch(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionJumpSwitch, value)

def set_show_markers(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionShowMarkers, value)

def set_only_show_team_markers(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionOnlyShowTeamMarkers, value)

def set_stunt_bike(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionStuntBike, value)

def set_shoot_in_air(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionShootInAir, value)

def set_show_name_tags(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionShowNameTags, value)

def set_join_messages(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionJoinMessages, value)

def set_death_messages(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionDeathMessages, value)

def set_chat_tags_enabled(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionChatTagsEnabled, value)

def set_use_classes(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionUseClasses, value)

def set_wall_glitch(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionWallGlitch, value)

def set_disable_backface_culling(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionDisableBackfaceCulling, value)

def set_disable_heli_blade_damage(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionDisableHeliBladeDamage, value)
