from dataclasses import dataclass
from typing import NamedTuple


@dataclass
class Vector:
    x: float
    y: float
    z: float

    def __add__(self, other: "Vector") -> "Vector":
        return Vector(self.x + other.x, self.y + other.y, self.z + other.z)

    def __sub__(self, other: "Vector") -> "Vector":
        return Vector(self.x - other.x, self.y - other.y, self.z - other.z)

    def __mul__(self, other: float) -> "Vector":
        return Vector(self.x * other, self.y * other, self.z * other)

    def __truediv__(self, other: float) -> "Vector":
        return Vector(self.x / other, self.y / other, self.z / other)

    def __neg__(self) -> "Vector":
        return Vector(-self.x, -self.y, -self.z)
    
    def __repr__(self) -> str:
        return f"Vector({self.x}, {self.y}, {self.z})"

@dataclass
class Quaterion:
    x: float
    y: float
    z: float
    w: float