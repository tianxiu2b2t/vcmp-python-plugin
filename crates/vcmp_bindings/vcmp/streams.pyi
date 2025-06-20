class WriteStream:
    """ A class for writing data to a stream. """
    
    def __init__(self): ...
    def __repr__(self): ...

    def write_bytes(self, data: bytes | bytearray) -> None:
        """ Write bytes to the stream. """

    def write_byte(self, value: int) -> None:
        """ Write a byte to the stream. 0 ~ 255 """
    
    def write_int(self, value: int) -> None:
        """ Write an integer to the stream (4 bits). """
        ...

    def write_long(self, value: int) -> None:
        """ Write a long integer to the stream (var bits). """
        ...

    def write_sq_string(self, value: str) -> None:
        """
        Write a string to the stream. 
        The string length must be less than 4095 characters.
        The string encoded in GBK.
        
        Why GBK?
        Because the server is written in C++ and uses ANSI encoding.
        Why 4095 length?
        https://bitbucket.org/stormeus/0.4-squirrel/src/master/CStream.cpp#lines-183
        """
        ...

    def write_string(self, value: str) -> None:
        """ 
        Write a string to the stream.
        The string encoded in GBK.
        The string length is not limited. (Fix it for s**t reasons)
        """
        ...

    def write_bool(self, value: bool) -> None:
        self.write_boolean(value)
        
    def write_boolean(self, value: bool) -> None:
        ...

    def write_float(self, value: float) -> None:
        """
        这里没有f64
        """
        ...
    
    def write_f32(self, value: float) -> None:
        """
        这里没有f64
        """
        self.write_float(value)


    def get_raw_buffer(self) -> bytes:
        """
        Get the raw buffer of the stream.
        """
        ...

class ReadStream:
    """ A class for reading data from a stream. """
    def __init__(self, data: bytes): ...
    def __repr__(self): ...

    def read(self, length: int) -> bytes:
        """ Read bytes from the stream. """
        ...

    def read_byte(self) -> int:
        """ Read a byte from the stream. 0 ~ 255 """
        ...

    def read_bytes(self, length: int) -> bytes:
        """ Read bytes from the stream. """
        ...

    def read_int(self) -> int:
        """ Read an integer from the stream (4 bits). """
        ...

    def read_long(self) -> int:
        """ Read a long integer from the stream (var bits). """
        ...

    def read_sq_string(self) -> str:
        """ Read a string from the stream. """
        ...

    def read_string(self) -> str:
        """ Read a string from the stream. """
        ...

    def read_boolean(self) -> bool:
        """ Read a boolean from the stream. """
        ...

    def read_float(self) -> float:
        """ Read a float from the stream. """
        ...

    def get_raw_buffer(self) -> bytes:
        """ Get the raw buffer of the stream. """
        ...
