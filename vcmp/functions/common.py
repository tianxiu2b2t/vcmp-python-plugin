from ..__abc import funcs

def get_player_ids() -> list[int]:
    players = []
    for i in range(100):
        try:
            funcs.get_player_name(i)
            players.append(i)
        except:
            ...
    return players