from .abc import Event

from vcmp.functions.player import Player
from vcmp.functions.vehicle import Vehicle
from vcmp.functions.keybind import KeyBind
from vcmp.streams import ReadStream
from vcmp.types import Vector

class PlayerEvent(Event):
    ...

class IncomingConnectionEvent(PlayerEvent):
    """玩家 incoming 连接事件"""
    
    @property
    def ip(self) -> str:
        """获取玩家 IP 地址"""
        ...
    
    @property
    def player_name(self) -> str:
        """获取玩家名称"""
        ...
    
    @property
    def password(self) -> str:
        """获取玩家密码"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class ClientScriptDataEvent(PlayerEvent):
    """客户端脚本数据事件"""
    
    @property
    def player(self) -> Player:
        """获取相关玩家对象"""
        ...
    
    @property
    def stream(self) -> ReadStream:
        """获取数据流对象"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerConnectEvent(PlayerEvent):
    """玩家连接事件"""
    
    @property
    def player(self) -> Player:
        """获取连接的玩家对象"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerDisconnectEvent(PlayerEvent):
    """玩家断开连接事件"""
    
    @property
    def player(self) -> Player:
        """获取断开连接的玩家对象"""
        ...
    
    @property
    def reason(self) -> int:
        """获取断开连接的原因"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerRequestClassEvent(PlayerEvent):
    """玩家请求角色事件"""
    
    @property
    def player(self) -> Player:
        """获取请求角色的玩家对象"""
        ...
    
    @property
    def class_id(self) -> int:
        """获取请求的角色 ID"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerSpawnEvent(PlayerEvent):
    """玩家 spawn 事件"""
    
    @property
    def player(self) -> Player:
        """获取 spawn 的玩家对象"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerRequestSpawnEvent(PlayerEvent):
    """玩家请求 spawn 事件"""
    
    @property
    def player(self) -> Player:
        """获取请求 spawn 的玩家对象"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerDeathEvent(PlayerEvent):
    """玩家死亡事件"""
    
    @property
    def player(self) -> Player:
        """获取死亡的玩家对象"""
        ...
    
    @property
    def killer(self) -> Player | None:
        """获取杀手玩家对象（可能为 None）"""
        ...
    
    @property
    def reason(self) -> int:
        """获取死亡原因"""
        ...
    
    @property
    def body(self) -> int:
        """获取死亡部位信息"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerUpdateEvent(PlayerEvent):
    """玩家更新事件"""
    
    @property
    def player(self) -> Player:
        """获取更新的玩家对象"""
        ...
    
    @property
    def update(self) -> int:
        """获取更新类型"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerRequestEnterVehicleEvent(PlayerEvent):
    """玩家请求进入载具事件"""
    
    @property
    def player(self) -> Player:
        """获取请求进入载具的玩家对象"""
        ...
    
    @property
    def vehicle(self) -> Vehicle:
        """获取目标载具对象"""
        ...
    
    @property
    def slot_index(self) -> int:
        """获取座位索引"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerEnterVehicleEvent(PlayerEvent):
    """玩家进入载具事件"""
    
    @property
    def player(self) -> Player:
        """获取进入载具的玩家对象"""
        ...
    
    @property
    def vehicle(self) -> Vehicle:
        """获取目标载具对象"""
        ...
    
    @property
    def slot_index(self) -> int:
        """获取座位索引"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerExitVehicleEvent(PlayerEvent):
    """玩家离开载具事件"""
    
    @property
    def player(self) -> Player:
        """获取离开载具的玩家对象"""
        ...
    
    @property
    def vehicle(self) -> Vehicle:
        """获取目标载具对象"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerNameChangeEvent(PlayerEvent):
    """玩家名称变更事件"""
    
    @property
    def player(self) -> Player:
        """获取名称变更的玩家对象"""
        ...
    
    @property
    def old_name(self) -> str:
        """获取旧名称"""
        ...
    
    @property
    def new_name(self) -> str:
        """获取新名称"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerStateChangeEvent(PlayerEvent):
    """玩家状态变更事件"""
    
    @property
    def player(self) -> Player:
        """获取状态变更的玩家对象"""
        ...
    
    @property
    def old_state(self) -> int:
        """获取旧状态"""
        ...
    
    @property
    def new_state(self) -> int:
        """获取新状态"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerActionChangeEvent(PlayerEvent):
    """玩家动作变更事件"""
    
    @property
    def player(self) -> Player:
        """获取动作变更的玩家对象"""
        ...
    
    @property
    def old_action(self) -> int:
        """获取旧动作"""
        ...
    
    @property
    def new_action(self) -> int:
        """获取新动作"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerOnFireChangeEvent(PlayerEvent):
    """玩家着火状态变更事件"""
    
    @property
    def player(self) -> Player:
        """获取着火状态变更的玩家对象"""
        ...
    
    @property
    def is_on_fire(self) -> bool:
        """获取是否着火"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerCrouchChangeEvent(PlayerEvent):
    """玩家蹲下状态变更事件"""
    
    @property
    def player(self) -> Player:
        """获取蹲下状态变更的玩家对象"""
        ...
    
    @property
    def is_crouching(self) -> bool:
        """获取是否蹲下"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerGameKeysChangeEvent(PlayerEvent):
    """玩家游戏按键变更事件"""
    
    @property
    def player(self) -> Player:
        """获取按键变更的玩家对象"""
        ...
    
    @property
    def old_keys(self) -> int:
        """获取旧按键状态"""
        ...
    
    @property
    def new_keys(self) -> int:
        """获取新按键状态"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerBeginTypingEvent(PlayerEvent):
    """玩家开始输入事件"""
    
    @property
    def player(self) -> Player:
        """获取开始输入的玩家对象"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerEndTypingEvent(PlayerEvent):
    """玩家结束输入事件"""
    
    @property
    def player(self) -> Player:
        """获取结束输入的玩家对象"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerAwayChangeEvent(PlayerEvent):
    """玩家离开状态变更事件"""
    
    @property
    def player(self) -> Player:
        """获取离开状态变更的玩家对象"""
        ...
    
    @property
    def is_away(self) -> bool:
        """获取是否离开"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerMessageEvent(PlayerEvent):
    """玩家发送消息事件"""
    
    @property
    def player(self) -> Player:
        """获取发送消息的玩家对象"""
        ...
    
    @property
    def message(self) -> str:
        """获取消息内容"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerCommandEvent(PlayerEvent):
    """玩家发送命令事件"""
    
    @property
    def player(self) -> Player:
        """获取发送命令的玩家对象"""
        ...
    
    @property
    def command(self) -> str:
        """获取命令名称"""
        ...
    
    @property
    def text(self) -> str:
        """获取命令文本"""
        ...
    
    @property
    def args(self) -> list[str]:
        """获取命令参数列表"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerPrivateMessageEvent(PlayerEvent):
    """玩家发送私聊事件"""
    
    @property
    def player(self) -> Player:
        """获取发送私聊的玩家对象"""
        ...
    
    @property
    def target(self) -> Player:
        """获取私聊目标玩家对象"""
        ...
    
    @property
    def message(self) -> str:
        """获取私聊内容"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerKeyBindDownEvent(PlayerEvent):
    """玩家按键按下事件"""
    
    @property
    def player(self) -> Player:
        """获取按键的玩家对象"""
        ...
    
    @property
    def key(self) -> KeyBind:
        """获取按下的按键"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerKeyBindUpEvent(PlayerEvent):
    """玩家按键释放事件"""
    
    @property
    def player(self) -> Player:
        """获取释放按键的玩家对象"""
        ...
    
    @property
    def key(self) -> KeyBind:
        """获取释放的按键"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerSpectateEvent(PlayerEvent):
    """玩家 spectator 事件"""
    
    @property
    def player(self) -> Player:
        """获取 spectator 的玩家对象"""
        ...
    
    @property
    def target(self) -> Player | None:
        """获取 spectator 目标玩家对象（可能为 None）"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerCrashReportEvent(PlayerEvent):
    """玩家崩溃报告事件"""
    
    @property
    def player(self) -> Player:
        """获取崩溃的玩家对象"""
        ...
    
    @property
    def report(self) -> str:
        """获取崩溃报告内容"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerModuleListEvent(PlayerEvent):
    """玩家模块列表事件"""
    
    @property
    def player(self) -> Player:
        """获取模块列表对应的玩家对象"""
        ...
    
    @property
    def modules(self) -> str:
        """获取模块列表内容"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerHealthChangeEvent(PlayerEvent):
    """玩家生命值变更事件"""
    
    @property
    def player(self) -> Player:
        """获取生命值变更的玩家对象"""
        ...
    
    @property
    def old_health(self) -> float:
        """获取旧生命值"""
        ...
    
    @property
    def new_health(self) -> float:
        """获取新生命值"""
        ...
    
    @property
    def current_health(self) -> float:
        """获取当前生命值"""
        ...
    
    @current_health.setter
    def current_health(self, value: float) -> None:
        """设置当前生命值"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerArmourChangeEvent(PlayerEvent):
    """玩家护甲值变更事件"""
    
    @property
    def player(self) -> Player:
        """获取护甲值变更的玩家对象"""
        ...
    
    @property
    def old_armour(self) -> float:
        """获取旧护甲值"""
        ...
    
    @property
    def new_armour(self) -> float:
        """获取新护甲值"""
        ...
    
    @property
    def current_armour(self) -> float:
        """获取当前护甲值"""
        ...
    
    @current_armour.setter
    def current_armour(self, value: float) -> None:
        """设置当前护甲值"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerWeaponChangeEvent(PlayerEvent):
    """玩家武器变更事件"""
    
    @property
    def player(self) -> Player:
        """获取武器变更的玩家对象"""
        ...
    
    @property
    def old_weapon(self) -> int:
        """获取旧武器"""
        ...
    
    @property
    def new_weapon(self) -> int:
        """获取新武器"""
        ...
    
    @property
    def current_weapon(self) -> int:
        """获取当前武器"""
        ...
    
    @current_weapon.setter
    def current_weapon(self, value: int) -> None:
        """设置当前武器"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerAmmoChangeEvent(PlayerEvent):
    """玩家弹药量变更事件"""
    
    @property
    def player(self) -> Player:
        """获取弹药量变更的玩家对象"""
        ...
    
    @property
    def old_ammo(self) -> int:
        """获取旧弹药量"""
        ...
    
    @property
    def new_ammo(self) -> int:
        """获取新弹药量"""
        ...
    
    @property
    def current_ammo(self) -> int:
        """获取当前弹药量"""
        ...
    
    @current_ammo.setter
    def current_ammo(self, value: int) -> None:
        """设置当前弹药量"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...

class PlayerMoveEvent(PlayerEvent):
    """玩家移动事件"""
    
    @property
    def player(self) -> Player:
        """获取移动的玩家对象"""
        ...
    
    @property
    def old_position(self) -> Vector:
        """获取旧位置"""
        ...
    
    @property
    def new_position(self) -> Vector:
        """获取新位置"""
        ...
    
    @property
    def current_position(self) -> Vector:
        """获取当前位置"""
        ...
    
    @current_position.setter
    def current_position(self, value: Vector) -> None:
        """设置当前位置"""
        ...
    
    def __repr__(self) -> str:
        """返回事件的字符串表示"""
        ...