#[derive(Debug, Clone)]
pub struct ObjectShotEvent {
    pub object_id: i32,
    pub player_id: i32,
    pub weapon_id: i32,
}

impl From<(i32, i32, i32)> for ObjectShotEvent {
    fn from(value: (i32, i32, i32)) -> Self {
        Self {
            object_id: value.0,
            player_id: value.1,
            weapon_id: value.2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObjectTouchedEvent {
    pub object_id: i32,
    pub player_id: i32,
}

impl From<(i32, i32)> for ObjectTouchedEvent {
    fn from(value: (i32, i32)) -> Self {
        Self {
            object_id: value.0,
            player_id: value.1,
        }
    }
}
