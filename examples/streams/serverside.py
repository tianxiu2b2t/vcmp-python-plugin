from vcmp.functions.player import Player
from vcmp.streams import WriteStream
from vcmp.callback import callbacks
from vcmp.events.player import ClientScriptDataEvent


def send_data(player: Player):
    stream = WriteStream()
    stream.write_string("Hello World!")  # in squirrel is PyWriteString
    stream.write_long(123)  # in squirrel is PyWriteLong
    stream.write_float(3.14)
    player.send_data(stream)


@callbacks.on_client_script_data()
def on_client_script_data(event: ClientScriptDataEvent):
    print("Received data from client")
    print(event.stream.get_raw_buffer())
