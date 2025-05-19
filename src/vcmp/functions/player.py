from vcmp.__export import funcs

def is_player_connected(
    id: int
) -> bool:
    """Checks if a player is connected to the server.

    Args:
        id (int): The ID of the player.

    Returns:
        bool: True if the player is connected, False otherwise.
    """
    return funcs.is_player_connected(id)