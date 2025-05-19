from vcmp.functions.keybind import get_bindkey
from vcmp.types import vcmpBodyPart, vcmpDisconnectReason, vcmpPlayerState, vcmpPlayerUpdate
from vcmp.events.abc import Event
from vcmp.instance import Player, get_player_from_id, get_vehicle_from_id
from vcmp.streams import ReadStream

class PlayerEvent(Event):
    id: int
    player: Player

    def __init__(
        self,
        id: int,
        *args,
        **kwargs
    ):
        super().__init__(*args, **kwargs)
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
    def __init__(self, id: int, data: bytes, size: int, *args, **kwargs):
        super().__init__(id, *args, **kwargs)
        self.data = data
        self.stream = ReadStream(data)
        self.size = size

class PlayerConnectEvent(PlayerEvent):
    ...

class PlayerDisconnectEvent(PlayerEvent):
    def __init__(self, id: int, reason: vcmpDisconnectReason, *args, **kwargs):
        super().__init__(id, *args, **kwargs)
        self.reason = vcmpDisconnectReason(reason)

class PlayerRequestClassEvent(PlayerEvent):
    classid: int

    def __init__(self, id: int, classid: int, *args, **kwargs):
        super().__init__(id, *args, **kwargs)
        self.classid = classid

class PlayerRequestSpawnEvent(PlayerEvent):
    ...

class PlayerSpawnEvent(PlayerEvent):
    ...

class PlayerDeathEvent(PlayerEvent):
    def __init__(self, id: int, killerid: int, reason: int, bodypart: vcmpBodyPart, *args, **kwargs):
        super().__init__(id, *args, **kwargs)

        self.killerid = killerid
        self.reason = reason
        self.bodypart = bodypart

        self.killer = get_player_from_id(killerid)

class PlayerUpdateEvent(PlayerEvent):
    update: vcmpPlayerUpdate

    def __init__(self, id: int, update: vcmpPlayerUpdate, *args, **kwargs):
        super().__init__(id, *args, **kwargs)
        self.update = update

class PlayerVehicleEvent(PlayerEvent):

    def __init__(self, id: int, vehicleid: int, *args, **kwargs):
        super().__init__(id, *args, **kwargs)
        self.vehicleid = vehicleid

        vehicle = get_vehicle_from_id(vehicleid)
        assert vehicle is not None, f"Vehicle with id {vehicleid} does not exist"

        self.vehicle = vehicle

class PlayerRequestEnterVehicleEvent(PlayerVehicleEvent):
    def __init__(self, id: int, vehicleid: int, slotindex: int, *args, **kwargs):
        super().__init__(id, vehicleid, *args, **kwargs)

        self.slotindex = slotindex


class PlayerEnterVehicleEvent(PlayerEvent):
    def __init__(self, id: int, vehicleid: int, slotindex: int, *args, **kwargs):
        super().__init__(id, vehicleid, *args, **kwargs)

        self.slotindex = slotindex


class PlayerExitVehicleEvent(PlayerVehicleEvent):
    ...


class PlayerNameChangeEvent(PlayerEvent):
    def __init__(self, id: int, old_name: str, new_name: str, *args, **kwargs):
        super().__init__(id, *args, **kwargs)

        self.old_name = old_name
        self.new_name = new_name

class PlayerStateChangeEvent(PlayerEvent):
    def __init__(self, id: int, old_state: vcmpPlayerState, new_state: vcmpPlayerState, *args, **kwargs):
        super().__init__(id, *args, **kwargs)

        self.old_state = old_state
        self.new_state = new_state

class PlayerActionChangeEvent(PlayerEvent):
    def __init__(self, id: int, old_action: int, new_action: int, *args, **kwargs):
        super().__init__(id, *args, **kwargs)

        self.old_action = old_action
        self.new_action = new_action

class PlayerOnFireChangeEvent(PlayerEvent):
    def __init__(self, id: int, onfire: bool, *args, **kwargs):
        super().__init__(id, *args, **kwargs)

        self.onfire = onfire

class PlayerCrouchChangeEvent(PlayerEvent):
    def __init__(self, id: int, crouching: bool, *args, **kwargs):
        super().__init__(id, *args, **kwargs)

        self.crouching = crouching


class PlayerGameKeysChangeEvent(PlayerEvent):
    def __init__(self, id: int, old_keys: int, new_keys: int, *args, **kwargs):
        super().__init__(id, *args, **kwargs)

        self.old_keys = old_keys
        self.new_keys = new_keys

class PlayerBeginTypingEvent(PlayerEvent):
    ...

class PlayerEndTypingEvent(PlayerEvent):
    ...

class PlayerAwayChangeEvent(PlayerEvent):
    def __init__(self, id: int, away: bool, *args, **kwargs):
        super().__init__(id, *args, **kwargs)

        self.away = away

class PlayerMessageEvent(PlayerEvent):
    def __init__(self, id: int, message: str, *args, **kwargs):
        super().__init__(id, *args, **kwargs)

        self.message = message

class PlayerCommandEvent(PlayerEvent):
    def __init__(self, id: int, message: str, *args, **kwargs):
        super().__init__(id, *args, **kwargs)

        self.message = message

        self.cmd, _, self.text = message.partition(' ')
        self.args = tuple(
            arg for arg in self.text.split(' ') if arg.strip()
        )

class PlayerPrivateMessageEvent(PlayerEvent):
    def __init__(self, id: int, senderid: int, message: str, *args, **kwargs):
        super().__init__(id, *args, **kwargs)

        self.senderid = senderid
        self.message = message

        sender = get_player_from_id(senderid)
        assert sender is not None
        
        self.sender = sender

class PlayerKeyBindEvent(PlayerEvent):
    def __init__(self, id: int, keyid: int, *args, **kwargs):
        super().__init__(id, *args, **kwargs)

        self.keyid = keyid

        key = get_bindkey(keyid)
        assert key is not None, f"Key with id {keyid} does not exist"
        self.key = key

class PlayerKeyBindDownEvent(PlayerKeyBindEvent):
    ...


class PlayerKeyBindUpEvent(PlayerKeyBindEvent):
    ...

class PlayerSpectateEvent(PlayerEvent):
    def __init__(self, id: int, targetid: int, *args, **kwargs):
        super().__init__(id, *args, **kwargs)

        self.targetid = targetid
        self.target = get_player_from_id(targetid)

class PlayerCrashReportEvent(PlayerEvent):
    def __init__(self, id: int, report: str, *args, **kwargs):
        super().__init__(id, *args, **kwargs)

        self.report = report

class PlayerModuleList(PlayerEvent):
    def __init__(self, id: int, modules: str, *args, **kwargs):
        super().__init__(id, *args, **kwargs)

        self.modules = modules
