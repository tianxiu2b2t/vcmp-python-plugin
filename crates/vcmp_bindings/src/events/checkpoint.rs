use crate::PlayerId;

#[derive(Debug, Clone)]
pub struct CheckpointEnteredEvent {
    pub checkpoint_id: i32,
    pub player_id: PlayerId,
}

impl From<(i32, i32)> for CheckpointEnteredEvent {
    fn from(value: (i32, i32)) -> Self {
        Self {
            checkpoint_id: value.0,
            player_id: value.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CheckpointExitedEvent {
    pub checkpoint_id: i32,
    pub player_id: PlayerId,
}

impl From<(i32, i32)> for CheckpointExitedEvent {
    fn from(value: (i32, i32)) -> Self {
        Self {
            checkpoint_id: value.0,
            player_id: value.1,
        }
    }
}
