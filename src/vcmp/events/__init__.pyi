from . import abc
from . import server
from . import player

class EventBuilder:
    @staticmethod
    def server_initialise() -> 'EventBuilder':
        ...