class ServerSettings:
    server_name: str
    port: int
    max_players: int
    flags: int

class Vector:
    x: float
    y: float
    z: float

    def __init__(self, x: float, y: float, z: float):
        ...

class Quaternion:
    x: float
    y: float
    z: float
    w: float

    def __init__(self, x: float, y: float, z: float, w: float):
        ...

class RGB:
    r: int
    g: int
    b: int
    a: int

    def __init__(self, r: int, g: int, b: int, a: int): ...
    def as_rgba(self) -> int: ...
    def as_argb(self) -> int: ...
    def as_rgb(self) -> int: ...
    def __repr__(self) -> str: ...

    @staticmethod
    def from_rgb(value: int, a: int) -> 'RGB': ...
    @staticmethod
    def from_rgba(value: int) -> 'RGB': ...
    @staticmethod
    def from_argb(value: int) -> 'RGB': ...