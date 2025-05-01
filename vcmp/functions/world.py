from dataclasses import dataclass

from ..__abc import funcs

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