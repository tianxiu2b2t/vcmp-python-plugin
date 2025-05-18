from dataclasses import dataclass


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

    def distance(self, other: "Vector") -> float:
        """
        Calculate the distance between two vectors (x, y).
        """
        return ((self.x - other.x) ** 2 + (self.y - other.y) ** 2 ** 2) ** 0.5
    
    def distance_with_z(self, other: "Vector") -> float:
        """
        Calculate the distance between two vectors (x, y, z).
        """
        return ((self.x - other.x) ** 2 + (self.y - other.y) ** 2 + (self.z - other.z) ** 2) ** 0.5
    
    
@dataclass
class Quaternion:
    x: float
    y: float
    z: float
    w: float

    def __add__(self, other: "Quaternion") -> "Quaternion":
        return Quaternion(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)
    
    def __sub__(self, other: "Quaternion") -> "Quaternion":
        return Quaternion(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w)
    
    def __mul__(self, other: float) -> "Quaternion":
        return Quaternion(self.x * other, self.y * other, self.z * other, self.w * other)
    
    def __truediv__(self, other: float) -> "Quaternion":
        return Quaternion(self.x / other, self.y / other, self.z / other, self.w / other)
    
    def __neg__(self) -> "Quaternion":
        return Quaternion(-self.x, -self.y, -self.z, -self.w)
    
    def __repr__(self) -> str:
        return f"Quaternion({self.x}, {self.y}, {self.z}, {self.w})"
