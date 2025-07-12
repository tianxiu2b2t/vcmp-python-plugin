from .abc import Event

from vcmp.functions.pickup import Pickup
from vcmp.functions.player import Player

class PickupEvent(Event):
    ...

class PickupPickAttemptEvent(PickupEvent):
    """拾取物拾取尝试事件"""
    
    @property
    def pickup(self) -> Pickup:
        """获取相关的拾取物对象"""
        ...
    
    @property
    def player(self) -> Player:
        """获取尝试拾取的玩家对象"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PickupPickedEvent(PickupEvent):
    """拾取物被成功拾取事件"""
    
    @property
    def pickup(self) -> Pickup:
        """获取被拾取的拾取物对象"""
        ...
    
    @property
    def player(self) -> Player:
        """获取拾取该物品的玩家对象"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PickupRespawnEvent(PickupEvent):
    """拾取物重生事件"""
    
    @property
    def pickup(self) -> Pickup:
        """获取重生的拾取物对象"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...