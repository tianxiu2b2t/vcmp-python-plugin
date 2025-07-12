from vcmp.types import ServerSettings

def set_servername(servername: str):
    """
        Set the server name.

        :param servername: The name of the server.
    """

    ...

def get_servername() -> str:
    """
        Get the server name.

        :return: The name of the server.
    """

    ...

def set_gamemode(gamemode: str):
    """
        Set the game mode.

        :param gamemode: The game mode.
    """

    ...

def get_gamemode() -> str:
    """
        Get the game mode.

        :return: The game mode.
    """

    ...

def set_password(password: str):
    """
        Set the server password.

        :param password: The password.
    """

    ...

def get_password() -> str:
    """
        Get the server password.

        :return: The password.
    """

    ...

def set_maxplayers(maxplayers: int):
    """
        Set the maximum number of players.

        Only set range from 1 to 100. 
        If you set more than 100, it will be set to 100.
        If you set less than 1, it will be set to 1.
        
        :param maxplayers: The maximum number of players.

    """

    ...

def get_maxplayers() -> int:
    """
        Get the maximum number of players.

        :return: The maximum number of players.
    """

    ...

def get_server_settings() -> ServerSettings:
    """
        Get the server settings.

        :return: The server settings.
    """
    ...

def get_server_version() -> int:
    """
        Get the server version.
        
        67710 of 0.4.7.1

        :return: The server version.
    """

    ...

def shutdown_server():
    """
        Shutdown the server.
    """

    ...