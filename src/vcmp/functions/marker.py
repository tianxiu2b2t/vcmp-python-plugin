from typing import Optional

from vcmp.types import Vector, RGB

class Marker:
    @property
    def id(self) -> int: ...
    
    def __hash__(self) -> int: ...
    def __eq__(self, other: 'Marker') -> bool: ...
    
    @property
    def color(self) -> RGB: ...
    
    @property
    def is_alive(self) -> bool: ...
    
    @property
    def model(self) -> int: ...
    
    @property
    def position(self) -> Vector: ...
    
    @property
    def scale(self) -> int: ...
    
    @property
    def world(self) -> int: ...
    
    def delete(self) -> None: ...

def create_marker(
    model: int,
    world: int,
    position: Vector,
    scale: int,
    color: RGB
) -> Marker: ...