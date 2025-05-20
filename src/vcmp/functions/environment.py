import math
from typing import overload
from vcmp.vec import Vector
from vcmp.instance import RGB
from vcmp.__export import funcs
from vcmp.types import (
    WastedSettings,
    vcmpServerOption,
    Weather,
    INT32_MAX
)

def set_taxi_boost_jump(value: bool) -> None:
    """
    Set the state of the taxi boost jump server option.

    Args:
        value (bool): True to enable the taxi boost jump feature, False to disable.
    """
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionTaxiBoostJump, value)

def get_taxi_boost_jump() -> bool:
    """
    Get the current state of the taxi boost jump server option.

    Returns:
        bool: True if the feature is enabled, False otherwise.
    """
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionTaxiBoostJump)

def set_drive_on_water(value: bool) -> None:
    """
    Set the state of the drive on water server option.

    Args:
        value (bool): True to allow vehicles to drive on water, False to disable.
    """
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionDriveOnWater, value)

def get_drive_on_water() -> bool:
    """
    Get the current state of the drive on water server option.

    Returns:
        bool: True if vehicles can drive on water, False otherwise.
    """
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionDriveOnWater)

def set_fast_switch(value: bool) -> None:
    """
    Set the state of the fast weapon switching server option.

    Args:
        value (bool): True to enable fast weapon switching, False to disable.
    """
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionFastSwitch, value)

def get_fast_switch() -> bool:
    """
    Get the current state of the fast weapon switching server option.

    Returns:
        bool: True if fast weapon switching is enabled, False otherwise.
    """
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionFastSwitch)

def set_friendly_fire(value: bool) -> None:
    """
    Set the state of the friendly fire server option.

    Args:
        value (bool): True to allow friendly fire, False to disable.
    """
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionFriendlyFire, value)

def get_friendly_fire() -> bool:
    """
    Get the current state of the friendly fire server option.

    Returns:
        bool: True if friendly fire is allowed, False otherwise.
    """
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionFriendlyFire)

def set_disable_drive_by(value: bool) -> None:
    """
    Set the state of the disable drive-by shooting server option.

    Args:
        value (bool): True to disable drive-by shooting, False to allow.
    """
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionDisableDriveBy, value)

def get_disable_drive_by() -> bool:
    """
    Get the current state of the disable drive-by shooting server option.

    Returns:
        bool: True if drive-by shooting is disabled, False otherwise.
    """
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionDisableDriveBy)

def set_perfect_handling(value: bool) -> None:
    """
    Set the state of the perfect vehicle handling server option.

    Args:
        value (bool): True to enable perfect handling, False to disable.
    """
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionPerfectHandling, value)

def get_perfect_handling() -> bool:
    """
    Get the current state of the perfect vehicle handling server option.

    Returns:
        bool: True if perfect handling is enabled, False otherwise.
    """
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionPerfectHandling)

def set_flying_cars(value: bool) -> None:
    """
    Set the state of the flying cars server option.

    Args:
        value (bool): True to allow cars to fly, False to disable.
    """
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionFlyingCars, value)

def get_flying_cars() -> bool:
    """
    Get the current state of the flying cars server option.

    Returns:
        bool: True if cars can fly, False otherwise.
    """
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionFlyingCars)

def set_jump_switch(value: bool) -> None:
    """
    Set the state of the jump switch server option.

    Args:
        value (bool): True to enable jump switching, False to disable.
    """
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionJumpSwitch, value)

def get_jump_switch() -> bool:
    """
    Get the current state of the jump switch server option.

    Returns:
        bool: True if jump switching is enabled, False otherwise.
    """
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionJumpSwitch)

def set_show_markers(value: bool) -> None:
    """
    Set the state of the show markers server option.

    Args:
        value (bool): True to display markers, False to hide.
    """
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionShowMarkers, value)

def get_show_markers() -> bool:
    """
    Get the current state of the show markers server option.

    Returns:
        bool: True if markers are displayed, False otherwise.
    """
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionShowMarkers)

def set_only_show_team_markers(value: bool) -> None:
    """
    Set the state of the only show team markers server option.

    Args:
        value (bool): True to only show team markers, False to show all.
    """
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionOnlyShowTeamMarkers, value)

def get_only_show_team_markers() -> bool:
    """
    Get the current state of the only show team markers server option.

    Returns:
        bool: True if only team markers are shown, False otherwise.
    """
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionOnlyShowTeamMarkers)

def set_stunt_bike(value: bool) -> None:
    """
    Set the state of the stunt bike server option.

    Args:
        value (bool): True to enable stunt bike features, False to disable.
    """
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionStuntBike, value)

def get_stunt_bike() -> bool:
    """
    Get the current state of the stunt bike server option.

    Returns:
        bool: True if stunt bike features are enabled, False otherwise.
    """
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionStuntBike)

def set_shoot_in_air(value: bool) -> None:
    """
    Set the state of the shoot in air server option.

    Args:
        value (bool): True to allow shooting while airborne, False to disable.
    """
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionShootInAir, value)

def get_shoot_in_air() -> bool:
    """
    Get the current state of the shoot in air server option.

    Returns:
        bool: True if shooting in air is allowed, False otherwise.
    """
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionShootInAir)

def set_show_name_tags(value: bool) -> None:
    """
    Set the state of the show name tags server option.

    Args:
        value (bool): True to display player name tags, False to hide.
    """
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionShowNameTags, value)

def get_show_name_tags() -> bool:
    """
    Get the current state of the show name tags server option.

    Returns:
        bool: True if name tags are displayed, False otherwise.
    """
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionShowNameTags)

def set_join_messages(value: bool) -> None:
    """
    Set the state of the join messages server option.

    Args:
        value (bool): True to display join messages, False to disable.
    """
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionJoinMessages, value)

def get_join_messages() -> bool:
    """
    Get the current state of the join messages server option.

    Returns:
        bool: True if join messages are displayed, False otherwise.
    """
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionJoinMessages)

def set_death_messages(value: bool) -> None:
    """
    Set the state of the death messages server option.

    Args:
        value (bool): True to display death messages, False to disable.
    """
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionDeathMessages, value)

def get_death_messages() -> bool:
    """
    Get the current state of the death messages server option.

    Returns:
        bool: True if death messages are displayed, False otherwise.
    """
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionDeathMessages)

def set_chat_tags_enabled(value: bool) -> None:
    """
    Set the state of the chat tags server option.

    Args:
        value (bool): True to enable chat tags, False to disable.
    """
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionChatTagsEnabled, value)

def get_chat_tags_enabled() -> bool:
    """
    Get the current state of the chat tags server option.

    Returns:
        bool: True if chat tags are enabled, False otherwise.
    """
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionChatTagsEnabled)

def set_use_classes(value: bool) -> None:
    """
    Set the state of the use classes server option.

    Args:
        value (bool): True to enable class system, False to disable.
    """
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionUseClasses, value)

def get_use_classes() -> bool:
    """
    Get the current state of the use classes server option.

    Returns:
        bool: True if class system is enabled, False otherwise.
    """
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionUseClasses)

def set_wall_glitch(value: bool) -> None:
    """
    Set the state of the wall glitch server option.

    Args:
        value (bool): True to allow wall glitching, False to disable.
    """
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionWallGlitch, value)

def get_wall_glitch() -> bool:
    """
    Get the current state of the wall glitch server option.

    Returns:
        bool: True if wall glitching is allowed, False otherwise.
    """
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionWallGlitch)

def set_disable_backface_culling(value: bool) -> None:
    """
    Set the state of the disable backface culling server option.

    Args:
        value (bool): True to disable backface culling, False to enable.
    """
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionDisableBackfaceCulling, value)

def get_disable_backface_culling() -> bool:
    """
    Get the current state of the disable backface culling server option.

    Returns:
        bool: True if backface culling is disabled, False otherwise.
    """
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionDisableBackfaceCulling)

def set_disable_heli_blade_damage(value: bool) -> None:
    """
    Set the state of the disable helicopter blade damage server option.

    Args:
        value (bool): True to disable damage from helicopter blades, False to enable.
    """
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionDisableHeliBladeDamage, value)

def get_disable_heli_blade_damage() -> bool:
    """
    Get the current state of the disable helicopter blade damage server option.

    Returns:
        bool: True if helicopter blade damage is disabled, False otherwise.
    """
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionDisableHeliBladeDamage)

def get_time() -> int:
    """
        Get the current time of the game world

        Returns:
            int: The current time of the game world in seconds.
    """

    return funcs.get_hour() * 60 + funcs.get_minute()

def set_time(value: int) -> None:
    """
    Set the current time of the game world.

    Args:
        value (int): The new time of the game world in seconds.
    """

    hour = value // 60
    minute = value % 60

    funcs.set_hour(hour)
    funcs.set_minute(minute)

def get_hour() -> int:
    """
    Get the current hour of the game world.

    Returns:
        int: The current hour of the game world.
    """
    return funcs.get_hour()

def set_hour(value: int) -> None:
    """
    Set the current hour of the game world.

    Args:
        value (int): The new hour of the game world.
    """
    funcs.set_hour(value)

def get_minute() -> int:
    """
    Get the current minute of the game world.

    Returns:
        int: The current minute of the game world.
    """
    return funcs.get_minute()

def set_minute(value: int) -> None:
    """
    Set the current minute of the game world.

    Args:
        value (int): The new minute of the game world.
    """

    funcs.set_minute(value)

def get_weather() -> int:
    """
    Get the current weather of the game world.

    Returns:
        int: The current weather of the game world.
    """
    return funcs.get_weather()

def set_weather(value: int | Weather) -> None:
    """
    Set the current weather of the game world.

    Args:
        value (int): The new weather of the game world.
    """
    funcs.set_weather(value)

def get_time_rate() -> int:
    """
    Get the current time rate of the game world.

    time rate is a milisecond multiplier for the game world.

    Returns:
        int: The current time rate of the game world.
    """
    return funcs.get_time_rate()

def set_time_rate(value: int) -> None:
    """
    Set the current time rate of the game world.

    time rate is a milisecond multiplier for the game world.

    Args:
        value (int): The new time rate of the game world.
    """

    funcs.set_time_rate(value)

def get_gravity() -> float:
    """
    Get the current gravity of the game world.

    Returns:
        float: The current gravity of the game world.
    """
    return funcs.get_gravity()

def set_gravity(value: float) -> None:
    """
    Set the current gravity of the game world.

    Args:
        value (float): The new gravity of the game world.
    """

    funcs.set_gravity(value)

def get_gamespeed() -> float:
    """
    Get the current gamespeed of the game world.

    Returns:
        float: The current gamespeed of the game world.
    """
    return funcs.get_game_speed()

def set_gamespeed(value: float) -> None:
    """
    Set the current gamespeed of the game world.

    Args:
        value (float): The new gamespeed of the game world.
    """

    funcs.set_game_speed(value)

def get_water_level() -> float:
    """
    Get the current water level of the game world.

    Returns:
        float: The current water level of the game world.
    """
    return funcs.get_water_level()

def set_water_level(value: float) -> None:
    """
    Set the current water level of the game world.

    Args:
        value (float): The new water level of the game world.

    """

    funcs.set_water_level(value)

def get_max_flight_altitude() -> float:
    """
    Get the current maximum flight altitude of the game world.

    Returns:
        float: The current maximum flight altitude of the game world.
    """
    return funcs.get_maximum_flight_altitude()

def set_max_flight_altitude(value: float) -> None:
    """
    Set the current maximum flight altitude of the game world.

    Args:
        value (float): The new maximum flight altitude of the game world.
    """

    funcs.set_maximum_flight_altitude(value)

def set_kill_command_delay(value: int) -> None:
    """
    Set the delay before a player can use the kill command.

    Args:
        value (int): The new delay before a player can use the kill command.
    """

    funcs.set_kill_command_delay(value)

def get_kill_command_delay() -> int:
    """
    Get the delay before a player can use the kill command.

    Returns:
        int: The delay before a player can use the kill command.
    """
    return funcs.get_kill_command_delay()

def disable_kill_command():
    """
    Disable the kill command for all players.
    """
    funcs.set_kill_command_delay(INT32_MAX)

def get_vehicles_forced_respawn_height() -> float:
    """
    Get the height at which vehicles are forced to respawn.

    Returns:
        float: The height at which vehicles are forced to respawn.
    """
    return funcs.get_vehicles_forced_respawn_height()

def set_vehicles_forced_respawn_height(value: float) -> None:
    """
    Set the height at which vehicles are forced to respawn.

    Args:
        value (float): The new height at which vehicles are forced to respawn.
    """

    funcs.set_vehicles_forced_respawn_height(value)

@overload
def add_player_class(
    team: int,
    color: RGB,
    skin: int,
    pos: Vector,
    angle: float,
    weapon: int,
    ammo: int
):
    ...

@overload
def add_player_class(
    team: int,
    color: RGB,
    skin: int,
    pos: Vector,
    angle: float,
    weapon: int,
    ammo: int,
    weapon2: int,
    ammo2: int
):
    ...

def add_player_class(
    team: int,
    color: RGB,
    skin: int,
    pos: Vector,
    angle: float,
    weapon: int,
    ammo: int,
    weapon2: int = 0,
    ammo2: int = 0,
    weapon3: int = 0,
    ammo3: int = 0
):
    """
    Add a player class to the game world.

    Args:
        team (int): The team of the player class.
        color (RGB): The color of the player class.
        skin (int): The skin of the player class.
        pos (Vector): The position of the player class.
        angle (float): The angle of the player class.
        weapon (int): The weapon of the player class.
        ammo (int): The ammo of the player class.
        weapon2 (int): The second weapon of the player class.
        ammo2 (int): The second ammo of the player class.
        weapon3 (int): The third weapon of the player class.
        ammo3 (int): The third ammo of the player class.
    """

    funcs.add_player_class(team, color.to_alpha(), skin, pos.x, pos.y, pos.z, angle, weapon, ammo, weapon2, ammo2, weapon3, ammo3)

def set_spawn_player_position(
    pos: Vector,
):
    """
    Set the spawn position of the player.

    Args:
        pos (Vector): The new spawn position of the player.
    """

    funcs.set_spawn_player_position(pos.x, pos.y, pos.z)

def set_spawn_camera_position(
    pos: Vector,
) -> None:
    """
    Set the camera position of the player.

    Args:
        pos (Vector): The new camera position of the player.
    """

    funcs.set_spawn_camera_position(pos.x, pos.y, pos.z)

def set_spawn_camera_look_at(
    pos: Vector,
) -> None:
    """
    Set the camera look at position of the player.

    Args:
        pos (Vector): The new camera look at position of the player.
    """

    funcs.set_spawn_camera_look_at(pos.x, pos.y, pos.z)

def set_spawn_camera(
    pos: Vector,
    look_yaw: float,
    look_pitch: float,
    range: float = 0.5
) -> None:
    """
    Set the camera position and look at position of the player.

    Args:
        pos (Vector): The new camera position of the player.
        look_yaw (float): The new camera look at yaw of the player. Should be in degrees.
        look_pitch (float): The new camera look at pitch of the player. Should be in degrees.
        range (float): The distance between the camera and the look at position. Default is 0.5.
    """

    look = Vector(0, 0, 0)

    # Calculate the look at position
    look.x = pos.x + math.cos(math.radians(look_yaw)) * math.sin(math.radians(look_pitch)) * range
    look.y = pos.y + math.sin(math.radians(look_yaw)) * math.sin(math.radians(look_pitch)) * range
    look.z = pos.z + math.cos(math.radians(look_pitch)) * range

    # Set the camera position and look at position
    set_spawn_camera_position(pos)
    set_spawn_camera_look_at(look)

def set_fall_timer(value: int) -> None:
    """
    Set the fail timer for the game.

    Args:
        value (int): The new fail timer value.
    """

    funcs.set_fall_timer(value)

def get_fall_timer() -> int:
    """
    Get the fail timer for the game.

    Returns:
        int: The fail timer value.
    """

    return funcs.get_fall_timer()

def get_wasted_settings() -> WastedSettings:
    """
    Get the wasted settings for the game.

    Returns:
        WastedSettings: The wasted settings.
    """

    settings = funcs.get_wasted_settings()

    return WastedSettings(
        settings["death_timer"],
        settings["fade_timer"],
        settings["fade_in_speed"],
        settings["fade_out_speed"],
        RGB.from_int(settings["fade_colour"]),
        settings["corpse_fade_start"],
        settings["corpse_fade_time"]
    )

def set_wasted_settings(settings: WastedSettings) -> None:
    """
    Set the wasted settings for the game.

    Args:
        settings (WastedSettings): The new wasted settings.
    """

    funcs.set_wasted_settings(
        settings.death_timer,
        settings.fade_timer,
        settings.fade_in_speed,
        settings.fade_out_speed,
        settings.fade_colour.to_int(),
        settings.corpse_fade_start,
        settings.corpse_fade_time
    )