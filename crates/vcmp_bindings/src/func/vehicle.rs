use crate::func::VcmpFunctions;
use crate::options::VcmpEntityPool;
use crate::utils::{Quaternion, Vector};
use crate::{VcmpError, VcmpResult};

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
    fn set_vehicle_world(&self, vehicle_id: i32, world_id: i32) -> VcmpResult<()>;
    fn respawn_vehicle(&self, vehicle_id: i32) -> VcmpResult<()>;
    fn set_vehicle_immunity(&self, vehicle_id: i32, immunity: u32) -> VcmpResult<()>;
    fn explode_vehicle(&self, vehicle_id: i32) -> VcmpResult<()>;
    fn set_vehicle_position(
        &self,
        vehicle_id: i32,
        position: Vector,
        remove_occupants: Option<bool>,
    ) -> VcmpResult<()>;
    fn set_vehicle_rotation(&self, vehicle_id: i32, rotation: Quaternion) -> VcmpResult<()>;
    fn set_vehicle_rotation_euler(&self, vehicle_id: i32, rotation: Vector) -> VcmpResult<()>;
    fn set_vehicle_speed(&self, vehicle_id: i32, speed: Vector) -> VcmpResult<()>;
    fn set_vehicle_rel_speed(&self, vehicle_id: i32, speed: Vector) -> VcmpResult<()>;
    fn set_vehicle_turn_speed(&self, vehicle_id: i32, speed: Vector) -> VcmpResult<()>;
    fn set_vehicle_rel_turn_speed(&self, vehicle_id: i32, speed: Vector) -> VcmpResult<()>;
    fn set_vehicle_add_speed(&self, vehicle_id: i32, speed: Vector) -> VcmpResult<()>;
    fn set_vehicle_add_rel_speed(&self, vehicle_id: i32, speed: Vector) -> VcmpResult<()>;
    fn set_vehicle_add_turn_speed(&self, vehicle_id: i32, speed: Vector) -> VcmpResult<()>;
    fn set_vehicle_add_rel_turn_speed(&self, vehicle_id: i32, speed: Vector) -> VcmpResult<()>;
    fn set_vehicle_spawn_position(&self, vehicle_id: i32, position: Vector) -> VcmpResult<()>;
    fn set_vehicle_spawn_rotation(&self, vehicle_id: i32, rotation: Quaternion) -> VcmpResult<()>;
    fn set_vehicle_spawn_rotation_euler(&self, vehicle_id: i32, rotation: Vector)
    -> VcmpResult<()>;
    fn set_vehicle_idle_respawn_timer(&self, vehicle_id: i32, timer: u32) -> VcmpResult<()>;
    fn set_vehicle_health(&self, vehicle_id: i32, health: f32) -> VcmpResult<()>;
    fn set_vehicle_color(
        &self,
        vehicle_id: i32,
        primary_color: i32,
        secondary_color: i32,
    ) -> VcmpResult<()>;
    fn set_vehicle_part_status(&self, vehicle_id: i32, part_id: i32, status: i32)
    -> VcmpResult<()>;
    fn set_vehicle_tyre_status(&self, vehicle_id: i32, tyre_id: i32, status: i32)
    -> VcmpResult<()>;
    fn set_vehicle_damage_data(&self, vehicle_id: i32, data: u32) -> VcmpResult<()>;
    fn set_vehicle_radio(&self, vehicle_id: i32, radio_id: i32) -> VcmpResult<()>;
}

impl SetVehicle for VcmpFunctions {
    fn set_vehicle_world(&self, vehicle_id: i32, world_id: i32) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleWorld)(vehicle_id, world_id);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn respawn_vehicle(&self, vehicle_id: i32) -> VcmpResult<()> {
        let code = (self.inner.RespawnVehicle)(vehicle_id);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_immunity(&self, vehicle_id: i32, immunity: u32) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleImmunityFlags)(vehicle_id, immunity);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn explode_vehicle(&self, vehicle_id: i32) -> VcmpResult<()> {
        let code = (self.inner.ExplodeVehicle)(vehicle_id);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_position(
        &self,
        vehicle_id: i32,
        position: Vector,
        remove_occupants: Option<bool>,
    ) -> VcmpResult<()> {
        let code = (self.inner.SetVehiclePosition)(
            vehicle_id,
            position.x,
            position.y,
            position.z,
            remove_occupants.unwrap_or(false) as u8,
        );
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_rotation(&self, vehicle_id: i32, rotation: Quaternion) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleRotation)(
            vehicle_id, rotation.x, rotation.y, rotation.z, rotation.w,
        );
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_rotation_euler(&self, vehicle_id: i32, rotation: Vector) -> VcmpResult<()> {
        let code =
            (self.inner.SetVehicleRotationEuler)(vehicle_id, rotation.x, rotation.y, rotation.z);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_speed(&self, vehicle_id: i32, speed: Vector) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleSpeed)(vehicle_id, speed.x, speed.y, speed.z, 0, 0);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_rel_speed(&self, vehicle_id: i32, speed: Vector) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleSpeed)(vehicle_id, speed.x, speed.y, speed.z, 0, 1);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_turn_speed(&self, vehicle_id: i32, speed: Vector) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleTurnSpeed)(vehicle_id, speed.x, speed.y, speed.z, 0, 0);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_rel_turn_speed(&self, vehicle_id: i32, speed: Vector) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleTurnSpeed)(vehicle_id, speed.x, speed.y, speed.z, 0, 1);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_add_speed(&self, vehicle_id: i32, speed: Vector) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleSpeed)(vehicle_id, speed.x, speed.y, speed.z, 1, 0);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_add_rel_speed(&self, vehicle_id: i32, speed: Vector) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleSpeed)(vehicle_id, speed.x, speed.y, speed.z, 1, 1);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_add_turn_speed(&self, vehicle_id: i32, speed: Vector) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleTurnSpeed)(vehicle_id, speed.x, speed.y, speed.z, 1, 0);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_add_rel_turn_speed(&self, vehicle_id: i32, speed: Vector) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleTurnSpeed)(vehicle_id, speed.x, speed.y, speed.z, 1, 1);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_spawn_position(&self, vehicle_id: i32, position: Vector) -> VcmpResult<()> {
        let code =
            (self.inner.SetVehicleSpawnPosition)(vehicle_id, position.x, position.y, position.z);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_spawn_rotation(&self, vehicle_id: i32, rotation: Quaternion) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleSpawnRotation)(
            vehicle_id, rotation.x, rotation.y, rotation.z, rotation.w,
        );
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_spawn_rotation_euler(
        &self,
        vehicle_id: i32,
        rotation: Vector,
    ) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleSpawnRotationEuler)(
            vehicle_id, rotation.x, rotation.y, rotation.z,
        );
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_idle_respawn_timer(&self, vehicle_id: i32, timer: u32) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleIdleRespawnTimer)(vehicle_id, timer);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_health(&self, vehicle_id: i32, health: f32) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleHealth)(vehicle_id, health);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_color(
        &self,
        vehicle_id: i32,
        primary_color: i32,
        secondary_color: i32,
    ) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleColour)(vehicle_id, primary_color, secondary_color);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_part_status(
        &self,
        vehicle_id: i32,
        part_id: i32,
        status: i32,
    ) -> VcmpResult<()> {
        let code = (self.inner.SetVehiclePartStatus)(vehicle_id, part_id, status);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_tyre_status(
        &self,
        vehicle_id: i32,
        tyre_id: i32,
        status: i32,
    ) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleTyreStatus)(vehicle_id, tyre_id, status);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_damage_data(&self, vehicle_id: i32, data: u32) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleDamageData)(vehicle_id, data);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_radio(&self, vehicle_id: i32, radio_id: i32) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleRadio)(vehicle_id, radio_id);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
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
