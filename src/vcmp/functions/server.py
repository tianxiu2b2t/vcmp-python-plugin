from vcmp.types import ServerSettings
from vcmp.__export import funcs


def set_servername(servername: str):
    """
        Set the server name.

        :param servername: The name of the server.
    """

    funcs.set_server_name(servername)

def get_servername() -> str:
    """
        Get the server name.

        :return: The name of the server.
    """

    return funcs.get_server_name()

def set_gamemode(gamemode: str):
    """
        Set the game mode.

        :param gamemode: The game mode.
    """

    funcs.set_game_mode_text(gamemode)

def get_gamemode() -> str:
    """
        Get the game mode.

        :return: The game mode.
    """

    return funcs.get_game_mode_text()

def set_password(password: str):
    """
        Set the server password.

        :param password: The password.
    """

    funcs.set_server_password(password)

def get_password() -> str:
    """
        Get the server password.

        :return: The password.
    """

    return funcs.get_server_password()

def set_maxplayers(maxplayers: int):
    """
        Set the maximum number of players.

        Only set range from 1 to 100. 
        If you set more than 100, it will be set to 100.
        If you set less than 1, it will be set to 1.
        
        :param maxplayers: The maximum number of players.

    """

    funcs.set_max_players(
        max(1, 
            min(100, maxplayers)
        )
    )

def get_maxplayers() -> int:
    """
        Get the maximum number of players.

        :return: The maximum number of players.
    """

    return funcs.get_max_players()

def get_server_settings() -> ServerSettings:
    """
        Get the server settings.

        :return: The server settings.
    """
    settings = funcs.get_server_settings()

    return ServerSettings(
        port=settings["port"],
        maxplayers=settings["max_players"],
        servername=settings["servername"],
        flags=settings["flags"],
        locked=len(get_password()) != 0,
        gamemode=get_gamemode(),
        password=get_password(),
        version=funcs.get_server_version()
    )

def get_server_version() -> int:
    """
        Get the server version.
        
        67710 of 0.4.7.1

        :return: The server version.
    """

    return funcs.get_server_version()

def shutdown_server():
    """
        Shutdown the server.
    """

    funcs.shutdown_server()

