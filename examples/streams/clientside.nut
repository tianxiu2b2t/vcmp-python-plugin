include("stream.nut")

function Script::ScriptLoad() {
    local data = ::Stream();
    data.PyWriteString("Hello World") // py: write_string
    data.PyWriteLong(123) // py: write_long
    data.WriteFloat(3.14) // raw
    Server.SendData(data)
}

function Server::ServerData(data) {
    local str = data.PyReadString() // py: read_string
    local num = data.PyReadLong() // py: read_long
    local flt = data.ReadFloat() // raw
    ::Console.Print("Received: " + str + ", " + num + ", " + flt)
}