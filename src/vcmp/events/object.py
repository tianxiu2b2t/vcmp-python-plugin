from vcmp.events.abc import Event
from vcmp.instance import Object, get_player_from_id

class ObjectEvent(Event):
    def __init__(
        self,
        object_id: int,
        id: int,
        *args,
        **kwargs
    ):
        super().__init__(*args, **kwargs)

        self.object_id = object_id
        self.id = id

        object = Object(object_id)
        assert object is not None, f"Object with id {object_id} does not exist"
        self.object = object
        
        player = get_player_from_id(id)
        assert player is not None, f"Player with id {id} does not exist"
        self.player = player

class ObjectShotEvent(ObjectEvent):
    def __init__(
        self,
        object_id: int,
        id: int,
        weaponid: int,
        *args,
        **kwargs
    ):
        super().__init__(object_id, id, *args, **kwargs)

        self.weaponid = weaponid

class ObjectTouchedEvent(ObjectEvent):
    ...