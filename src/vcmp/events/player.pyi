from vcmp.functions.player import Player
from .abc import Event

class PlayerEvent(Event):
    ...

class IncomingConnection(PlayerEvent):
    ...

class PlayerConnectEvent(PlayerEvent):
    @property
    def player(self) -> Player:
        ...

class PlayerDisconnectEvent(PlayerEvent):
    @property
    def player(self) -> Player:
        ...
