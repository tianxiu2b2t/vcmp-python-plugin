use crate::PlayerId;

#[derive(Debug, Clone)]
pub struct PickupPickAttemptEvent {
    pub pickup_id: i32,
    pub player_id: PlayerId,
}

impl From<(i32, i32)> for PickupPickAttemptEvent {
    fn from(value: (i32, i32)) -> Self {
        Self {
            pickup_id: value.0,
            player_id: value.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PickupPickedEvent {
    pub pickup_id: i32,
    pub player_id: PlayerId,
}

impl From<(i32, i32)> for PickupPickedEvent {
    fn from(value: (i32, i32)) -> Self {
        Self {
            pickup_id: value.0,
            player_id: value.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PickupRespawnEvent {
    pub pickup_id: i32,
}

impl From<i32> for PickupRespawnEvent {
    fn from(value: i32) -> Self {
        Self { pickup_id: value }
    }
}
