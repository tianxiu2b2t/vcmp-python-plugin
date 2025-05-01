from dataclasses import dataclass

from ..__export import INT32_MAX, funcs, vcmpServerOption

@dataclass
class WorldBounds:
    min_x: float
    min_y: float
    max_x: float
    max_y: float

def set_world_bounds(
    bounds: WorldBounds,
):
    funcs.set_world_bounds(
        min_x=bounds.min_x,
        min_y=bounds.min_y,
        max_x=bounds.max_x,
        max_y=bounds.max_y,
    )

def get_world_bounds():
    val = funcs.get_world_bounds()
    return WorldBounds(
        min_x=val["min_x"],
        min_y=val["min_y"],
        max_x=val["max_x"],
        max_y=val["max_y"]
    )

def set_time_rate(rate: int):
    funcs.set_time_rate(rate)

def get_time_rate():
    return funcs.get_time_rate()

def get_time():
    return funcs.get_hour() * 60 + funcs.get_minute()

def get_hour():
    return funcs.get_hour()

def get_minute():
    return funcs.get_minute()

def set_hour(hour: int):
    funcs.set_hour(hour)

def set_minute(minute: int):
    funcs.set_minute(minute)

def set_time(t: int):
    t = abs(t) % 1440
    
    hour = t // 60
    minute = t % 60

    set_hour(hour)
    set_minute(minute)

def set_weather(weather: int):
    funcs.set_weather(weather)

def get_weather():
    return funcs.get_weather()

def set_gravity(gravity: float):
    funcs.set_gravity(gravity)

def get_gravity():
    return funcs.get_gravity()

def set_game_speed(speed: float):
    funcs.set_game_speed(speed)

def get_game_speed():
    return funcs.get_game_speed()

def set_water_level(level: float):
    funcs.set_water_level(level)

def get_water_level():
    return funcs.get_water_level()

def set_maximum_flight_altitude(alitude: float):
    funcs.set_maximum_flight_altitude(alitude)

def get_maximum_flight_altitude():
    return funcs.get_maximum_flight_altitude()

def set_kill_command_delay(delay: int):
    funcs.set_kill_command_delay(delay)

def get_kill_command_delay():
    return funcs.get_kill_command_delay()

def disable_kill_command():
    set_kill_command_delay(INT32_MAX)

def set_vehicles_forced_respawn_height(height: float):
    funcs.set_vehicles_forced_respawn_height(height)

def get_vehicles_forced_respawn_height():
    return funcs.get_vehicles_forced_respawn_height()

def create_explosion(
    world_id: int,
    type: int,
    x: float,
    y: float,
    z: float,
    responsible_player_id: int,
    at_ground_level: int
):
    funcs.create_explosion(world_id, type, x, y, z, responsible_player_id, at_ground_level)

def play_sound(
    world_id: int,
    sound_id: int,
    x: float,
    y: float,
    z: float
):
    funcs.play_sound(
        world_id,
        sound_id,
        x,
        y,
        z
    )

def hide_map_object(
    model_id: int,
    x: int,
    y: int,
    z: int
):
    funcs.hide_map_object(model_id, x, y, z)

def show_map_object(
    model_id: int,
    x: int,
    y: int,
    z: int
):
    funcs.show_map_object(model_id, x, y, z)

def show_all_map_objects():
    funcs.show_all_map_objects()

"""
    Game environment options
"""


def set_taxi_boost_jump(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionTaxiBoostJump, value)

def get_taxi_boost_jump() -> bool:
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionTaxiBoostJump)

def set_drive_on_water(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionDriveOnWater, value)

def get_drive_on_water() -> bool:
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionDriveOnWater)

def set_fast_switch(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionFastSwitch, value)

def get_fast_switch() -> bool:
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionFastSwitch)

def set_friendly_fire(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionFriendlyFire, value)

def get_friendly_fire() -> bool:
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionFriendlyFire)

def set_disable_drive_by(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionDisableDriveBy, value)

def get_disable_drive_by() -> bool:
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionDisableDriveBy)

def set_perfect_handling(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionPerfectHandling, value)

def get_perfect_handling() -> bool:
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionPerfectHandling)

def set_flying_cars(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionFlyingCars, value)

def get_flying_cars() -> bool:
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionFlyingCars)

def set_jump_switch(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionJumpSwitch, value)

def get_jump_switch() -> bool:
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionJumpSwitch)

def set_show_markers(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionShowMarkers, value)

def get_show_markers() -> bool:
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionShowMarkers)

def set_only_show_team_markers(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionOnlyShowTeamMarkers, value)

def get_only_show_team_markers() -> bool:
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionOnlyShowTeamMarkers)

def set_stunt_bike(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionStuntBike, value)

def get_stunt_bike() -> bool:
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionStuntBike)

def set_shoot_in_air(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionShootInAir, value)

def get_shoot_in_air() -> bool:
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionShootInAir)

def set_show_name_tags(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionShowNameTags, value)

def get_show_name_tags() -> bool:
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionShowNameTags)

def set_join_messages(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionJoinMessages, value)

def get_join_messages() -> bool:
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionJoinMessages)

def set_death_messages(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionDeathMessages, value)

def get_death_messages() -> bool:
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionDeathMessages)

def set_chat_tags_enabled(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionChatTagsEnabled, value)

def get_chat_tags_enabled() -> bool:
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionChatTagsEnabled)

def set_use_classes(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionUseClasses, value)

def get_use_classes() -> bool:
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionUseClasses)

def set_wall_glitch(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionWallGlitch, value)

def get_wall_glitch() -> bool:
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionWallGlitch)

def set_disable_backface_culling(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionDisableBackfaceCulling, value)

def get_disable_backface_culling() -> bool:
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionDisableBackfaceCulling)

def set_disable_heli_blade_damage(value: bool) -> None:
    funcs.set_server_option(vcmpServerOption.vcmpServerOptionDisableHeliBladeDamage, value)

def get_disable_heli_blade_damage() -> bool:
    return funcs.get_server_option(vcmpServerOption.vcmpServerOptionDisableHeliBladeDamage)


"""
    Weapon Settings
"""

def set_weapon_data_value(
    weapon_id: int,
    field: int,
    value: float
):
    funcs.set_weapon_data_value(weapon_id, field, value)

def get_weapon_data_value(
    weapon_id: int,
    field: int
) -> float:
    return funcs.get_weapon_data_value(weapon_id, field)

def reset_weapon_data_value(
    weapon_id: int,
    field: int
):
    funcs.reset_weapon_data_value(weapon_id, field)

def is_weapon_data_value_modified(
    weapon_id: int,
    field: int
) -> bool:
    return funcs.is_weapon_data_value_modified(weapon_id, field)

def reset_weapon_data(
    weapon_id: int
):
    funcs.reset_weapon_data(weapon_id)

def reset_all_weapon_data():
    funcs.reset_all_weapon_data()


"""
    Radio
"""

def add_radio_stream(
    radio_id: int,
    radio_name: str,
    radio_url: str,
    can_select: bool
):
    funcs.add_radio_stream(radio_id, radio_name, radio_url, can_select)

def remove_radio_stream(
    radio_id: int
):
    funcs.remove_radio_stream(radio_id)


"""
    Spawning and Classes
"""

def add_player_class(
    team_id: int,
    colour: int,
    skin: int,
    x: float,
    y: float,
    z: float,
    angle: float,
    weapon1: int,
    weapon1_ammo: int,
    weapon2: int,
    weapon2_ammo: int,
    weapon3: int,
    weapon3_ammo: int,
):
    funcs.add_player_class(team_id, colour, skin, x, y, z, angle, weapon1, weapon1_ammo, weapon2, weapon2_ammo, weapon3, weapon3_ammo)

