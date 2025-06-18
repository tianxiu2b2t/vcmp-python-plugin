pub struct VehicleUpdateEvent {
    pub vehicle_id: i32,
    pub update_type: i32,
}

impl From<(i32, i32)> for VehicleUpdateEvent {
    fn from(value: (i32, i32)) -> Self {
        Self {
            vehicle_id: value.0,
            update_type: value.1,
        }
    }
}

pub struct VehicleExplodeEvent {
    pub vehicle_id: i32,
}

impl From<i32> for VehicleExplodeEvent {
    fn from(value: i32) -> Self {
        Self { vehicle_id: value }
    }
}

pub struct VehicleRespawnEvent {
    pub vehicle_id: i32,
}

impl From<i32> for VehicleRespawnEvent {
    fn from(value: i32) -> Self {
        Self { vehicle_id: value }
    }
}
