import io
from vcmp.logger import logger
import struct


class Stream:
    def __init__(
        self
    ):
        self._buffer = io.BytesIO()

class WriteStream(Stream):
    def write(self, data: bytes | bytearray | int):
        if isinstance(data, int):
            data = data.to_bytes(1, "big")
        elif isinstance(data, bytearray):
            data = bytes(data)
        elif not isinstance(data, bytes):
            raise TypeError("Data must be bytes, bytearray or int")
        self._buffer.write(data)

    def write_byte(self, value: int):
        if not (0 <= value <= 255):
            raise ValueError("Byte value must be between 0 and 255")
        self.write(bytearray([value]))

    def write_int(self, value: int):
        self.write(struct.pack("!I", value))

    def write_long(self, value: int):
        datum = (value << 1) ^ (value >> 63)
        while (datum & ~0x7F) != 0:
            self.write(bytearray([(datum & 0x7F) | 0x80]))
            datum >>= 7
        self.write(bytearray([datum]))

    def write_sq_string(self, value: str, encoding = "gbk"):
        data = value.encode(encoding)
        if len(data) > 4096:
            data = data[:4096]
            logger.warning(f"String is too long, truncated to 4096 bytes")
        self.write(len(data).to_bytes(2, "big"))
        self.write(data)

    def write_string(self, value: str, encoding = "gbk"):
        data = value.encode(encoding)
        self.write_long(len(data))
        self.write(data)

    def write_boolean(self, value: bool):
        self.write(bytearray([1 if value else 0]))

    def write_float(self, value: float):
        self.write(struct.pack("f", value))
    
class ReadStream(Stream):
    def __init__(self, data: bytes):
        super().__init__()
        self._buffer = io.BytesIO(data)

    def read(self, length: int) -> bytes:
        return self._buffer.read(length)
    
    def read_byte(self) -> int:
        return ord(self.read(1))
    
    def read_bytes(self, length: int) -> bytes:
        return self.read(length)
    
    def read_int(self) -> int:
        return struct.unpack("!I", self.read(4))[0]

    def read_long(self) -> int:
        b = ord(self.read(1))
        n = b & 0x7F
        shift = 7
        while (b & 0x80) != 0:
            b = ord(self.read(1))
            n |= (b & 0x7F) << shift
            shift += 7
        datum = (n >> 1) ^ -(n & 1)
        return datum
    
    def read_sq_string(self, encoding = "gbk") -> str:
        length = int.from_bytes(self.read(2))
        return self.read(length).decode(encoding)

    def read_string(self, encoding = "gbk") -> str:
        length = self.read_long()
        return self.read(length).decode(encoding)
        
    def read_boolean(self) -> bool:
        return self.read(1)[0] != 0

    def read_float(self) -> float:
        return struct.unpack("f", self.read(4))[0]
    