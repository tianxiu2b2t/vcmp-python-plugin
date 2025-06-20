function Stream::PyReadLong() {
    local b = this.ReadByte()
    local n = b & 0x7F
    local shift = 7
    while ((b & 0x80) != 0) {
        b = this.ReadByte()
        n = n | (b & 0x7F) << shift;
        shift += 7
    }
    return (n >> 1) ^ -(n & 1)
}
function Stream::PyReadString() {
    local length = this.PyReadLong()
    local result = ""
    for (local i = 0; i < length; i++) {
        result += this.ReadByte().tochar()
    }
    return result
}
function Stream::PyReadBoolean() {
    return this.ReadByte() != 0
}
function Stream::PyReadFloat() {
    return this.ReadFloat()
}
function Stream::PyWriteLong(n) {
    local d = (value << 1) ^ (value >> 63);
    while ((d & ~0x7F) != 0) {
        this.WriteByte((d & 0x7F) | 0x80)
        d = d >> 7
    }
    this.WriteByte(d)
}
function Stream::PyWriteString(s) {
    this.PyWriteLong(s.len())
    foreach (char in s) {
        this.WriteByte(char.toint())
    }
}
function Stream::PyWriteBoolean(b) {
    this.WriteByte(b ? 1 : 0)
}
