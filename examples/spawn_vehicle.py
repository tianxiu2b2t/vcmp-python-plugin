from vcmp.callback import callbacks
from vcmp.events.player import PlayerCommandEvent
from vcmp.functions.vehicle import create_vehicle
from vcmp.types import RGB


@callbacks.on_player_command()
def _(event: PlayerCommandEvent):
    if event.command in ("car", "veh", "cveh"):
        if not event.text.isdigit():
            event.player.send_raw_message(
                RGB.from_rgb(0xFF5555), "Usage: /car <vehicleid>"
            )
            return
        veh = create_vehicle(
            int(event.text),
            event.player.world,
            event.player.position,
            event.player.angle,
        )
        event.player.vehicle = veh
        event.player.send_raw_message(
            RGB.from_rgb(0x55FF55), f"Spawned vehicle {event.text}"
        )
