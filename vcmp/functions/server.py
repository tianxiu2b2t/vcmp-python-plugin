from dataclasses import dataclass

from ..__export import funcs, vcmpServerOption

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

def set_servername(servername: str) -> None:
    funcs.set_server_name(servername) # type: ignore : vcmp server default use gbk encoding

def set_max_players(max_players: int) -> None:
    funcs.set_max_players(max(1, min(100, max_players)))

def set_gamemode(gamemode: str) -> None:
    funcs.set_game_mode_text(gamemode)

def get_servername() -> str:
    return funcs.get_server_name()

def get_max_players() -> int:
    return funcs.get_max_players()

def get_gamemode() -> str:
    return funcs.get_game_mode_text()

def set_password(password: str) -> None:
    funcs.set_server_password(password)

def get_password() -> str:
    return funcs.get_server_password()

def shutdown_server() -> None:
    funcs.shutdown_server()

def shutdown():
    shutdown_server()

