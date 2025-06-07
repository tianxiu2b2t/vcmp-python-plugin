use crate::func::VcmpFunctions;
use crate::options::VcmpEntityPool;
use crate::utils::{Quaternion, Vector};

pub trait QueryVehicle {
    fn is_vehicle_alive(&self, vehicle_id: i32) -> bool;
    fn is_vehicle_streamed_for_player(&self, player_id: i32, vehicle_id: i32) -> bool;
    fn get_vehicle_world(&self, vehicle_id: i32) -> i32;
    fn get_vehicle_model(&self, vehicle_id: i32) -> i32;
    fn get_vehicle_occupant(&self, vehicle_id: i32, seat_id: i32) -> i32;
    fn get_vehicle_immunity(&self, vehicle_id: i32) -> u32;
    fn is_vehicle_wrecked(&self, vehicle_id: i32) -> bool;
    fn get_vehicle_position(&self, vehicle_id: i32) -> Vector;
    fn get_vehicle_rotation(&self, vehicle_id: i32) -> Quaternion;
    fn get_vehicle_rotation_euler(&self, vehicle_id: i32) -> Vector;
    fn get_vehicle_speed(&self, vehicle_id: i32) -> Vector;
    fn get_vehicle_rel_speed(&self, vehicle_id: i32) -> Vector;
    fn get_vehicle_turn_speed(&self, vehicle_id: i32) -> Vector;
    fn get_vehicle_rel_turn_speed(&self, vehicle_id: i32) -> Vector;
    fn get_vehicle_spawn_position(&self, vehicle_id: i32) -> Vector; // Vector3<f32>
    fn get_vehicle_spawn_rotation(&self, vehicle_id: i32) -> Quaternion;
    fn get_vehicle_spawn_rotation_euler(&self, vehicle_id: i32) -> Vector;
    fn get_vehicle_idle_respawn_timer(&self, vehicle_id: i32) -> u32;
    fn get_vehicle_health(&self, vehicle_id: i32) -> f32;
    fn get_vehicle_color(&self, vehicle_id: i32) -> (i32, i32);
    fn get_vehicle_part_status(&self, vehicle_id: i32, part_id: i32) -> bool;
    fn get_vehicle_tyre_status(&self, vehicle_id: i32, tyre_id: i32) -> bool;
    fn get_vehicle_damage_data(&self, vehicle_id: i32) -> u32;
    fn get_vehicle_radio(&self, vehicle_id: i32) -> i32;
    fn get_vehicle_turret_rotation(&self, vehicle_id: i32) -> (f32, f32);
}

pub trait SetVehicle {
    fn set_vehicle_world(&self, vehicle_id: i32, world_id: i32);
    fn respawn_vehicle(&self, vehicle_id: i32);
    fn set_vehicle_immunity(&self, vehicle_id: i32, immunity: i32);
    fn explode_vehicle(&self, vehicle_id: i32);
    fn set_vehicle_position(
        &self,
        vehicle_id: i32,
        position: Vector,
        remove_occupants: Option<bool>,
    );
    fn set_vehicle_rotation(&self, vehicle_id: i32, rotation: Quaternion);
    fn set_vehicle_rotation_euler(&self, vehicle_id: i32, rotation: Vector);

    fn set_vehicle_speed(&self, vehicle_id: i32, speed: Vector);
    fn set_vehicle_rel_speed(&self, vehicle_id: i32, speed: Vector);
    fn set_vehicle_turn_speed(&self, vehicle_id: i32, speed: Vector);
    fn set_vehicle_rel_turn_speed(&self, vehicle_id: i32, speed: Vector);

    fn set_vehicle_add_speed(&self, vehicle_id: i32, speed: Vector);
    fn set_vehicle_add_rel_speed(&self, vehicle_id: i32, speed: Vector);
    fn set_vehicle_add_turn_speed(&self, vehicle_id: i32, speed: Vector);
    fn set_vehicle_add_rel_turn_speed(&self, vehicle_id: i32, speed: Vector);

    fn set_vehicle_spawn_position(&self, vehicle_id: i32, position: Vector);
    fn set_vehicle_spawn_rotation(&self, vehicle_id: i32, rotation: Quaternion);
    fn set_vehicle_spawn_rotation_euler(&self, vehicle_id: i32, rotation: Vector);

    fn set_vehicle_idle_respawn_timer(&self, vehicle_id: i32, timer: i32);
    fn set_vehicle_health(&self, vehicle_id: i32, health: f32);
    fn set_vehicle_color(&self, vehicle_id: i32, primary_color: i32, secondary_color: i32);
    fn set_vehicle_part_status(&self, vehicle_id: i32, part_id: i32, status: i32);
    fn set_vehicle_tyre_status(&self, vehicle_id: i32, tyre_id: i32, status: i32);
    fn set_vehicle_damage_data(&self, vehicle_id: i32, data: f32);
    fn set_vehicle_radio(&self, vehicle_id: i32, radio_id: i32);
}

impl QueryVehicle for VcmpFunctions {
    fn is_vehicle_alive(&self, vehicle_id: i32) -> bool {
        (self.inner.CheckEntityExists)(VcmpEntityPool::Vehicle.into(), vehicle_id) != 0
    }
    fn is_vehicle_streamed_for_player(&self, player_id: i32, vehicle_id: i32) -> bool {
        (self.inner.IsVehicleStreamedForPlayer)(player_id, vehicle_id) != 0
    }

    fn get_vehicle_world(&self, vehicle_id: i32) -> i32 {
        (self.inner.GetVehicleWorld)(vehicle_id)
    }

    fn get_vehicle_model(&self, vehicle_id: i32) -> i32 {
        (self.inner.GetVehicleModel)(vehicle_id)
    }

    fn get_vehicle_occupant(&self, vehicle_id: i32, seat_id: i32) -> i32 {
        (self.inner.GetVehicleOccupant)(vehicle_id, seat_id)
    }

    fn get_vehicle_immunity(&self, vehicle_id: i32) -> u32 {
        (self.inner.GetVehicleImmunityFlags)(vehicle_id)
    }

    fn is_vehicle_wrecked(&self, vehicle_id: i32) -> bool {
        (self.inner.IsVehicleWrecked)(vehicle_id) != 0
    }

    fn get_vehicle_position(&self, vehicle_id: i32) -> Vector {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        (self.inner.GetVehiclePosition)(vehicle_id, &mut x, &mut y, &mut z);
        Vector::new(x, y, z)
    }

    fn get_vehicle_rotation(&self, vehicle_id: i32) -> Quaternion {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let mut w = 0.0;
        (self.inner.GetVehicleRotation)(vehicle_id, &mut x, &mut y, &mut z, &mut w);
        Quaternion::new(x, y, z, w)
    }

    fn get_vehicle_rotation_euler(&self, vehicle_id: i32) -> Vector {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        (self.inner.GetVehicleRotationEuler)(vehicle_id, &mut x, &mut y, &mut z);
        Vector::new(x, y, z)
    }

    fn get_vehicle_speed(&self, vehicle_id: i32) -> Vector {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        (self.inner.GetVehicleSpeed)(vehicle_id, &mut x, &mut y, &mut z, 0);
        Vector::new(x, y, z)
    }

    fn get_vehicle_rel_speed(&self, vehicle_id: i32) -> Vector {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        (self.inner.GetVehicleSpeed)(vehicle_id, &mut x, &mut y, &mut z, 0);
        Vector::new(x, y, z)
    }

    fn get_vehicle_turn_speed(&self, vehicle_id: i32) -> Vector {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        (self.inner.GetVehicleTurnSpeed)(vehicle_id, &mut x, &mut y, &mut z, 0);
        Vector::new(x, y, z)
    }

    fn get_vehicle_rel_turn_speed(&self, vehicle_id: i32) -> Vector {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        (self.inner.GetVehicleTurnSpeed)(vehicle_id, &mut x, &mut y, &mut z, 1);
        Vector::new(x, y, z)
    }

    fn get_vehicle_spawn_position(&self, vehicle_id: i32) -> Vector {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        (self.inner.GetVehicleSpawnPosition)(vehicle_id, &mut x, &mut y, &mut z);
        Vector::new(x, y, z)
    }

    fn get_vehicle_spawn_rotation(&self, vehicle_id: i32) -> Quaternion {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let mut w = 0.0;
        (self.inner.GetVehicleSpawnRotation)(vehicle_id, &mut x, &mut y, &mut z, &mut w);
        Quaternion::new(x, y, z, w)
    }

    fn get_vehicle_spawn_rotation_euler(&self, vehicle_id: i32) -> Vector {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        (self.inner.GetVehicleSpawnRotationEuler)(vehicle_id, &mut x, &mut y, &mut z);
        Vector::new(x, y, z)
    }

    fn get_vehicle_idle_respawn_timer(&self, vehicle_id: i32) -> u32 {
        (self.inner.GetVehicleIdleRespawnTimer)(vehicle_id)
    }

    fn get_vehicle_health(&self, vehicle_id: i32) -> f32 {
        (self.inner.GetVehicleHealth)(vehicle_id)
    }

    fn get_vehicle_color(&self, vehicle_id: i32) -> (i32, i32) {
        let mut primary = 0;
        let mut secondary = 0;
        (self.inner.GetVehicleColour)(vehicle_id, &mut primary, &mut secondary);
        (primary, secondary)
    }

    fn get_vehicle_part_status(&self, vehicle_id: i32, part_id: i32) -> bool {
        (self.inner.GetVehiclePartStatus)(vehicle_id, part_id) != 0
    }

    fn get_vehicle_tyre_status(&self, vehicle_id: i32, tyre_id: i32) -> bool {
        (self.inner.GetVehicleTyreStatus)(vehicle_id, tyre_id) != 0
    }

    fn get_vehicle_damage_data(&self, vehicle_id: i32) -> u32 {
        (self.inner.GetVehicleDamageData)(vehicle_id)
    }

    fn get_vehicle_radio(&self, vehicle_id: i32) -> i32 {
        (self.inner.GetVehicleRadio)(vehicle_id)
    }

    fn get_vehicle_turret_rotation(&self, vehicle_id: i32) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        (self.inner.GetVehicleTurretRotation)(vehicle_id, &mut x, &mut y);
        (x, y)
    }
}
// pyo3 property, setter, getter
