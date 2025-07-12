from typing import Tuple, Optional

from .abc import Event
from vcmp.functions.vehicle import Vehicle
from vcmp.types import Vector

class VehicleEvent(Event):
    """车辆事件的基类"""
    def __init__(self) -> None: ...
    def __repr__(self) -> str: ...

class VehicleUpdateEvent(VehicleEvent):
    """车辆更新事件"""
    @property
    def vehicle(self) -> Vehicle:
        """获取事件关联的车辆"""
        ...
    
    @property
    def update_type(self) -> int:
        """获取更新类型（整数标识）"""
        ...
    
    def __repr__(self) -> str: ...

class VehicleExplodeEvent(VehicleEvent):
    """车辆爆炸事件"""
    @property
    def vehicle(self) -> Vehicle:
        """获取事件关联的车辆"""
        ...
    
    def __repr__(self) -> str: ...

class VehicleRespawnEvent(VehicleEvent):
    """车辆重生事件"""
    @property
    def vehicle(self) -> Vehicle:
        """获取事件关联的车辆"""
        ...
    
    def __repr__(self) -> str: ...

class VehicleMoveEvent(VehicleEvent):
    """车辆移动事件（包含位置变化信息）"""
    @property
    def vehicle(self) -> Vehicle:
        """获取事件关联的车辆"""
        ...
    
    @property
    def old_position(self) -> Vector:
        """获取移动前的位置"""
        ...
    
    @property
    def new_position(self) -> Vector:
        """获取移动后的目标位置"""
        ...
    
    @property
    def current_position(self) -> Vector:
        """获取当前实际位置（可修改）"""
        ...
    
    @current_position.setter
    def current_position(self, position: Vector) -> None:
        """设置当前实际位置"""
        ...
    
    def __repr__(self) -> str: ...

class VehicleHealthChangeEvent(VehicleEvent):
    """车辆生命值变化事件"""
    @property
    def vehicle(self) -> Vehicle:
        """获取事件关联的车辆"""
        ...
    
    @property
    def old_health(self) -> float:
        """获取变化前的生命值"""
        ...
    
    @property
    def new_health(self) -> float:
        """获取变化后的目标生命值"""
        ...
    
    @property
    def current_health(self) -> float:
        """获取当前实际生命值（可修改）"""
        ...
    
    @current_health.setter
    def current_health(self, health: float) -> None:
        """设置当前实际生命值"""
        ...
    
    def __repr__(self) -> str: ...