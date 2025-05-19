from vcmp.events.abc import Event
from vcmp.instance import CheckPoint, get_player_from_id

class CheckpointEvent(Event):
    def     __init__(
        self,
        checkpoint_id: int,
        id: int,
        *args,
        **kwargs
    ):
        super().__init__(*args, **kwargs)

        self.checkpoint_id = checkpoint_id
        self.id = id

        checkpoint = CheckPoint(checkpoint_id)
        assert checkpoint is not None, f"Checkpoint with id {checkpoint_id} does not exist"

        self.checkpoint = checkpoint

        player = get_player_from_id(id)
        assert player is not None, f"Player with id {id} does not exist"
        self.player = player

class CheckpointEnteredEvent(CheckpointEvent):
    ...

class CheckpointExitedEvent(CheckpointEvent):
    ...