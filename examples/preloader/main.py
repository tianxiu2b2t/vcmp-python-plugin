from vcmp.callback import callbacks
from vcmp.events.player import PlayerCommandEvent
from vcmp.types import EventType
from vcmp import reload

@callbacks.on_player_command(tag="pre-reload")
def _(event: PlayerCommandEvent):
    if event.command.lower() == "reload":
        reload()


if __name__ == "__main__":
    import core  # type: ignore

    core.main()  # type: ignore

    # remove
    for func in callbacks.get_register_callbacks(EventType.PlayerCommand, tag="pre-reload"):
        callbacks.remove_callback(func)
