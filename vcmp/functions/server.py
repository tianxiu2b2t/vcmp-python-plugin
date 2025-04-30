from dataclasses import dataclass
from ..__abc import funcs

@dataclass
class ServerSettings:
    max_players: int
    port: int
    servername: str
    flags: int

def get_server_settings() -> ServerSettings: 
    return ServerSettings(**funcs.get_server_settings())
    
def get_server_version() -> int: 
    return funcs.get_server_version()