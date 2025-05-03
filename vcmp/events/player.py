from vcmp.__export import vcmpBodyPart, vcmpDisconnectReason, vcmpPlayerState, vcmpPlayerUpdate
from .abc import Event

class PlayerEvent(Event):
    ...

class IncomingConnectionEvent(PlayerEvent):
    __fields__ = (
        "name",
        "password",
        "address",
    )
    player_name: str
    password: str
    address: str

class ClientScriptDataEvent(PlayerEvent):
    __fields__ = (
        "id"
    )
    id: int

class PlayerConnectEvent(PlayerEvent):
    __fields__ = (
        "id",
    )
    id: int

class PlayerDisconnectEvent(PlayerEvent):
    __fields__ = (
        "id",
        "reason"
    )
    id: int
    reason: vcmpDisconnectReason

class PlayerRequestClassEvent(PlayerEvent):
    __fields__ = (
        "id",
        "clazz"
    )
    id: int
    clazz: int

class PlayerRequestSpawnEvent(PlayerEvent):
    __fields__ = (
        "id",
    )
    id: int

class PlayerSpawnEvent(PlayerEvent):
    __fields__ = (
        "id",
    )
    id: int

class PlayerDeathEvent(PlayerEvent):
    __fields__ = (
        "id",
        "killerid",
        "reason",
        "bodypart",
    )
    id: int
    killerid: int
    reason: int
    bodypart: vcmpBodyPart

class PlayerUpdateEvent(PlayerEvent):
    __fields__ = (
        "id",
        "update",
    )
    id: int
    update: vcmpPlayerUpdate
class PlayerRequestEnterVehicleEvent(PlayerEvent):
    __fields__ = (
        "id",
        "vehicleid",
        "slotindex"
    )
    id: int
    vehicleid: int
    slotindex: int

class PlayerEnterVehicleEvent(PlayerEvent):
    __fields__ = (
        "id",
        "vehicleid",
        "slotindex",
    )
    id: int
    vehicleid: int
    slotindex: int

class PlayerExitVehicleEvent(PlayerEvent):
    __fields__ = (
        "id",
        "vehicleid",
    )
    id: int
    vehicleid: int

class PlayerNameChangeEvent(PlayerEvent):
    __fields__ = (
        "id",
        "oldname",
        "newname",
    )
    id: int
    oldname: str
    newname: str

class PlayerStateChangeEvent(PlayerEvent):
    __fields__ = (
        "id",
        "oldstate",
        "newstate",
    )
    id: int
    oldstate: vcmpPlayerState
    newstate: vcmpPlayerState

class PlayerActionChangeEvent(PlayerEvent):
    __fields__ = (
        "id",
        "oldaction",
        "newaction",
    )
    id: int
    oldaction: int
    newaction: int

class PlayerOnFireChangeEvent(PlayerEvent):
    __fields__ = (
        "id",
        "isonfire",
    )
    id: int
    isonfire: int

class PlayerCrouchChangeEvent(PlayerEvent):
    __fields__ = (
        "id",
        "iscrouching",
    )
    id: int
    iscrouching: int

class PlayerGameKeysChangeEvent(PlayerEvent):
    __fields__ = (
        "id",
        "oldkeys",
        "newkeys",
    )
    id: int
    oldkeys: int
    newkeys: int

class PlayerBeginTypingEvent(PlayerEvent):
    __fields__ = (
        "id",
    )
    id: int

class PlayerEndTypingEvent(PlayerEvent):
    __fields__ = (
        "id",
    )
    id: int

class PlayerAwayChangeEvent(PlayerEvent):
    __fields__ = (
        "id",
        "isaway",
    )
    id: int
    isaway: int

class PlayerMessageEvent(PlayerEvent):
    __fields__ = (
        "id",
        "message",
    )
    id: int
    message: str

class PlayerCommandEvent(PlayerEvent):
    __fields__ = (
        "id",
        "message",
    )
    id: int
    message: str

class PlayerPrivateMessageEvent(PlayerEvent):
    __fields__ = (
        "id",
        "senderid",
        "message",
    )
    id: int
    senderid: int
    message: str

class PlayerKeyBindDownEvent(PlayerEvent):
    __fields__ = (
        "id",
        "bindid",
    )
    id: int
    bindid: int

class PlayerKeyBindUpEvent(PlayerEvent):
    __fields__ = (
        "id",
        "bindid",
    )
    id: int
    bindid: int

class PlayerSpectateEvent(PlayerEvent):
    __fields__ = (
        "id",
        "targetid",
    )
    id: int
    targetid: int

class PlayerCrashReportEvent(PlayerEvent):
    __fields__ = (
        "id",
        "report",
    )
    id: int
    report: str

class PlayerModuleList(PlayerEvent):
    __fields__ = (
        "id",
        "list",
    )
    id: int
    list: str
