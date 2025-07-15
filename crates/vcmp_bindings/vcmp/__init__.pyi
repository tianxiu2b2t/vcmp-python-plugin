from typing import NewType, TYPE_CHECKING

if TYPE_CHECKING:
    from . import util
    from . import streams

__all__ = ["util", "streams"]

u8 = NewType("u8", int)
u16 = NewType("u16", int)
u32 = NewType("u32", int)
u64 = NewType("u64", int)
u128 = NewType("u128", int)
i8 = NewType("i8", int)
i16 = NewType("i16", int)
i32 = NewType("i32", int)
i64 = NewType("i64", int)
i128 = NewType("i128", int)
f32 = NewType("f32", float)
f64 = NewType("f64", float)
