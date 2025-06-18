use crate::VehicleId;

#[derive(Debug, Clone)]
pub struct VehicleUpdateEvent {
    pub vehicle_id: VehicleId,
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

#[derive(Debug, Clone)]
pub struct VehicleExplodeEvent {
    pub vehicle_id: VehicleId,
}

impl From<i32> for VehicleExplodeEvent {
    fn from(value: i32) -> Self {
        Self { vehicle_id: value }
    }
}

#[derive(Debug, Clone)]
pub struct VehicleRespawnEvent {
    pub vehicle_id: VehicleId,
}

impl From<i32> for VehicleRespawnEvent {
    fn from(value: i32) -> Self {
        Self { vehicle_id: value }
    }
}
