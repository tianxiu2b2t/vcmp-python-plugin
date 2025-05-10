from ..functions.keybind import get_bindkey
from ..__export import vcmpBodyPart, vcmpDisconnectReason, vcmpPlayerState, vcmpPlayerUpdate
from .abc import Event
from ..instance import Player, get_player_from_id, get_vehicle_from_id
from ..streams import ReadStream

class PlayerEvent(Event):
    id: int
    player: Player

    def __init__(
        self,
        id: int
    ):
        self.id = id

        player = get_player_from_id(id)
        assert player is not None, f"Player with id {id} does not exist"

        self.player = player
        

class IncomingConnectionEvent(Event):
    __fields__ = (
        "name",
        "password",
        "address",
    )
    player_name: str
    password: str
    address: str

class ClientScriptDataEvent(PlayerEvent):
    def __init__(self, id: int, data: bytes, size: int):
        super().__init__(id)
        self.data = data
        self.stream = ReadStream(data)
        self.size = size

class PlayerConnectEvent(PlayerEvent):
    ...

class PlayerDisconnectEvent(PlayerEvent):
    reason: vcmpDisconnectReason

    def __init__(self, id: int, reason: vcmpDisconnectReason):
        super().__init__(id)
        self.reason = vcmpDisconnectReason(reason)

class PlayerRequestClassEvent(PlayerEvent):
    classid: int

    def __init__(self, id: int, classid: int):
        super().__init__(id)
        self.classid = classid

class PlayerRequestSpawnEvent(PlayerEvent):
    ...

class PlayerSpawnEvent(PlayerEvent):
    ...

class PlayerDeathEvent(PlayerEvent):
    def __init__(self, id: int, killerid: int, reason: int, bodypart: vcmpBodyPart):
        super().__init__(id)

        self.killerid = killerid
        self.reason = reason
        self.bodypart = bodypart

        self.killer = get_player_from_id(killerid)

class PlayerUpdateEvent(PlayerEvent):
    update: vcmpPlayerUpdate

    def __init__(self, id: int, update: vcmpPlayerUpdate):
        super().__init__(id)
        self.update = update

class PlayerRequestEnterVehicleEvent(PlayerEvent):
    def __init__(self, id: int, vehicleid: int, slotindex: int):
        super().__init__(id)

        self.vehicleid = vehicleid
        self.slotindex = slotindex

        self.vehicle = get_vehicle_from_id(vehicleid)


class PlayerEnterVehicleEvent(PlayerEvent):
    def __init__(self, id: int, vehicleid: int, slotindex: int):
        super().__init__(id)

        self.vehicleid = vehicleid
        self.slotindex = slotindex

        self.vehicle = get_vehicle_from_id(vehicleid)


class PlayerExitVehicleEvent(PlayerEvent):
    def __init__(self, id: int, vehicleid: int):
        super().__init__(id)

        self.vehicleid = vehicleid

        self.vehicle = get_vehicle_from_id(vehicleid)


class PlayerNameChangeEvent(PlayerEvent):
    def __init__(self, id: int, old_name: str, new_name: str):
        super().__init__(id)

        self.old_name = old_name
        self.new_name = new_name

class PlayerStateChangeEvent(PlayerEvent):
    def __init__(self, id: int, old_state: vcmpPlayerState, new_state: vcmpPlayerState):
        super().__init__(id)

        self.old_state = old_state
        self.new_state = new_state

class PlayerActionChangeEvent(PlayerEvent):
    def __init__(self, id: int, old_action: int, new_action: int):
        super().__init__(id)

        self.old_action = old_action
        self.new_action = new_action

class PlayerOnFireChangeEvent(PlayerEvent):
    def __init__(self, id: int, onfire: bool):
        super().__init__(id)

        self.onfire = onfire

class PlayerCrouchChangeEvent(PlayerEvent):
    def __init__(self, id: int, crouching: bool):
        super().__init__(id)

        self.crouching = crouching


class PlayerGameKeysChangeEvent(PlayerEvent):
    def __init__(self, id: int, old_keys: int, new_keys: int):
        super().__init__(id)

        self.old_keys = old_keys
        self.new_keys = new_keys

class PlayerBeginTypingEvent(PlayerEvent):
    ...

class PlayerEndTypingEvent(PlayerEvent):
    ...

class PlayerAwayChangeEvent(PlayerEvent):
    def __init__(self, id: int, away: bool):
        super().__init__(id)

        self.away = away

class PlayerMessageEvent(PlayerEvent):
    def __init__(self, id: int, message: str):
        super().__init__(id)

        self.message = message

class PlayerCommandEvent(PlayerEvent):
    def __init__(self, id: int, message: str):
        super().__init__(id)

        self.message = message

        self.cmd, _, self.text = message.partition(' ')
        self.args = tuple(
            arg for arg in self.text.split(' ') if arg.strip()
        )

class PlayerPrivateMessageEvent(PlayerEvent):
    def __init__(self, id: int, senderid: int, message: str):
        super().__init__(id)

        self.senderid = senderid
        self.message = message

        sender = get_player_from_id(senderid)
        assert sender is not None
        
        self.sender = sender

class PlayerKeyBindDownEvent(PlayerEvent):
    def __init__(self, id: int, keyid: int):
        super().__init__(id)

        self.keyid = keyid

        self.key = get_bindkey(keyid)


class PlayerKeyBindUpEvent(PlayerEvent):
    def __init__(self, id: int, keyid: int):
        super().__init__(id)

        self.keyid = keyid

        self.key = get_bindkey(keyid)

class PlayerSpectateEvent(PlayerEvent):
    def __init__(self, id: int, targetid: int):
        super().__init__(id)

        self.targetid = targetid
        self.target = get_player_from_id(targetid)

class PlayerCrashReportEvent(PlayerEvent):
    def __init__(self, id: int, report: str):
        super().__init__(id)

        self.report = report

class PlayerModuleList(PlayerEvent):
    def __init__(self, id: int, modules: str):
        super().__init__(id)

        self.modules = modules
