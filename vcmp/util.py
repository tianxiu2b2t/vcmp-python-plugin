from dataclasses import dataclass
import random

POLY = tuple[float, float]

@dataclass
class AreaPoints:
    x: float
    y: float

def get_vehicle_random_color(
    color: int
):
    if color < 0:
        return random.randint(0, 94)
    return color

def in_poly(
    x: float,
    y: float,
    *polies: POLY
):
    """
    Determines if a point is inside a polygon using the Jordan Curve Theorem.

    Args:
        x (float): The x-coordinate of the point.
        y (float): The y-coordinate of the point.
        ui_points (unsigned int): The number of points in the polygon.
        f_points (list of AreaPoints): The points creating the polygon.

    Returns:
        bool: True if the point is inside the polygon, False otherwise.
    """

    # Initialize variables to track crossings
    crossings = 0

    # Iterate through each line segment of the polygon
    ui_points = len(polies)
    f_points = [
        AreaPoints(
            polies[i][0],
            polies[i][1]
        ) for i in range(ui_points)
    ]
    for i in range(ui_points):
        # Ensure the line segment is checked from left to right
        if f_points[i].x < f_points[(i + 1) % ui_points].x:
            x1 = f_points[i].x
            x2 = f_points[(i + 1) % ui_points].x
        else:
            x1 = f_points[(i + 1) % ui_points].x
            x2 = f_points[i].x

        # Check if the ray can potentially cross the line segment
        if x > x1 and x <= x2 and (y < f_points[i].y or y <= f_points[(i + 1) % ui_points].y):
            # Calculate the equation of the line segment
            dx = f_points[(i + 1) % ui_points].x - f_points[i].x
            dy = f_points[(i + 1) % ui_points].y - f_points[i].y

            # Avoid division by zero for vertical lines
            if abs(dx) < 0.000001:
                k = float('inf')
            else:
                k = dy / dx

            m = f_points[i].y - k * f_points[i].x

            # Determine if the ray crosses the line segment
            y2 = k * x + m
            if y <= y2:
                crossings += 1

    # Return True if the number of crossings is odd (point is inside)
    return crossings % 2 == 1