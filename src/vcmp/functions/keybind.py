from typing import NamedTuple, Optional, overload, final
from vcmp.__export import funcs
from vcmp.types import KeyCode

class KeyBind(
    NamedTuple
):
    slot: int
    can_release: bool
    key1: KeyCode
    key2: Optional[KeyCode]
    key3: Optional[KeyCode]

    def __hash__(self) -> int:
        return self.slot

@overload
def bindkey(
    can_release: bool,
    key: KeyCode,
) -> KeyBind:
    ...

@overload
def bindkey(
    can_release: bool,
    key: KeyCode,
    key2: KeyCode,
) -> KeyBind:
    ...

def bindkey(
    can_release: bool,
    key: KeyCode,
    key2: Optional[KeyCode] = None,
    key3: Optional[KeyCode] = None,
) -> KeyBind:
    """Bind a key to trigger event call."""
    keys = [
        key,
        key2 if key2 is not None else 0,
        key3 if key3 is not None else 0,
    ]
    slot = funcs.get_key_bind_unused_slot()
    funcs.register_key_bind(slot, can_release, *keys)

    data = funcs.get_key_bind_data(slot)
    return KeyBind(
        slot,
        data[0],
        data[1],
        data[2] if data[2] != -1 else None,
        data[3] if data[3] != -1 else None,
    )

def get_bindkey(
    slot: int
):
    """Get binded key from slot."""
    try:
        data = funcs.get_key_bind_data(slot)
        return KeyBind(
            slot,
            data[0],
            data[1],
            data[2] if data[2] != -1 else None,
            data[3] if data[3] != -1 else None,
        )
    except:
        return None
    
def remove_bindkey(
    slot: int
):
    """
        Remove binded key from slot.
    """
    try:
        funcs.remove_key_bind(slot)
        return True
    except:
        return False
    
def remove_all_bindkeys():
    """Remove all binded keys."""
    funcs.remove_all_key_binds()
    