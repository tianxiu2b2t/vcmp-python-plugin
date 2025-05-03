import random


def get_vehicle_random_color(
    color: int
):
    if color < 0:
        return random.randint(0, 94)
    return color