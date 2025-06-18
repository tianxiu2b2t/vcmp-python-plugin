use crate::func::VcmpFunctions;
use crate::options::{VcmpEntityPool, VcmpVehicleOption};
use crate::utils::{Quaternionf32, Vectorf32};
use crate::{PlayerId, VcmpError, VcmpResult, VehicleId};

pub trait VehicleMethods {
    fn create_vehicle(
        &self,
        model_index: i32,
        world: i32,
        position: Vectorf32,
        angle: f32,
        primary_colour: i32,
        secondary_colour: i32,
    ) -> i32;
    fn delete_vehicle(&self, vehicle_id: VehicleId) -> VcmpResult<()>;

    fn is_vehicle_3d_arrow_for_player(&self, vehicle_id: VehicleId, player_id: PlayerId) -> bool;

    fn set_vehicle_3d_arrow_for_player(
        &self,
        vehicle_id: VehicleId,
        player_id: PlayerId,
        show: bool,
    ) -> VcmpResult<()>;
}
impl VehicleMethods for VcmpFunctions {
    fn create_vehicle(
        &self,
        model_index: i32,
        world: i32,
        position: Vectorf32,
        angle: f32,
        primary_colour: i32,
        secondary_colour: i32,
    ) -> i32 {
        (self.inner.CreateVehicle)(
            model_index,
            world,
            position.x,
            position.y,
            position.z,
            angle,
            primary_colour,
            secondary_colour,
        )
    }
    fn delete_vehicle(&self, vehicle_id: VehicleId) -> VcmpResult<()> {
        let code = (self.inner.DeleteVehicle)(vehicle_id);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn is_vehicle_3d_arrow_for_player(&self, vehicle_id: VehicleId, player_id: PlayerId) -> bool {
        (self.inner.GetVehicle3DArrowForPlayer)(vehicle_id, player_id) != 0
    }

    fn set_vehicle_3d_arrow_for_player(
        &self,
        vehicle_id: VehicleId,
        player_id: PlayerId,
        show: bool,
    ) -> VcmpResult<()> {
        let code = (self.inner.SetVehicle3DArrowForPlayer)(vehicle_id, player_id, show as u8);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
}

pub trait QueryVehicleOptions {
    fn get_vehicle_option_doors_locked(&self, vehicle_id: VehicleId) -> bool;
    fn get_vehicle_option_alarm(&self, vehicle_id: VehicleId) -> bool;
    fn get_vehicle_option_lights(&self, vehicle_id: VehicleId) -> bool;
    fn get_vehicle_option_radio_locked(&self, vehicle_id: VehicleId) -> bool;
    fn get_vehicle_option_ghost(&self, vehicle_id: VehicleId) -> bool;
    fn get_vehicle_option_siren(&self, vehicle_id: VehicleId) -> bool;
    fn get_vehicle_option_single_use(&self, vehicle_id: VehicleId) -> bool;
    fn get_vehicle_option_engine_disabled(&self, vehicle_id: VehicleId) -> bool;
    fn get_vehicle_option_boot_open(&self, vehicle_id: VehicleId) -> bool;
    fn get_vehicle_option_bonnet_open(&self, vehicle_id: VehicleId) -> bool;
}
impl QueryVehicleOptions for VcmpFunctions {
    fn get_vehicle_option_doors_locked(&self, vehicle_id: VehicleId) -> bool {
        self.get_vehicle_option(vehicle_id, VcmpVehicleOption::DoorsLocked)
    }
    fn get_vehicle_option_alarm(&self, vehicle_id: VehicleId) -> bool {
        self.get_vehicle_option(vehicle_id, VcmpVehicleOption::Alarm)
    }
    fn get_vehicle_option_lights(&self, vehicle_id: VehicleId) -> bool {
        self.get_vehicle_option(vehicle_id, VcmpVehicleOption::Lights)
    }
    fn get_vehicle_option_radio_locked(&self, vehicle_id: VehicleId) -> bool {
        self.get_vehicle_option(vehicle_id, VcmpVehicleOption::RadioLocked)
    }
    fn get_vehicle_option_ghost(&self, vehicle_id: VehicleId) -> bool {
        self.get_vehicle_option(vehicle_id, VcmpVehicleOption::Ghost)
    }
    fn get_vehicle_option_siren(&self, vehicle_id: VehicleId) -> bool {
        self.get_vehicle_option(vehicle_id, VcmpVehicleOption::Siren)
    }
    fn get_vehicle_option_single_use(&self, vehicle_id: VehicleId) -> bool {
        self.get_vehicle_option(vehicle_id, VcmpVehicleOption::SingleUse)
    }
    fn get_vehicle_option_engine_disabled(&self, vehicle_id: VehicleId) -> bool {
        self.get_vehicle_option(vehicle_id, VcmpVehicleOption::EngineDisabled)
    }
    fn get_vehicle_option_boot_open(&self, vehicle_id: VehicleId) -> bool {
        self.get_vehicle_option(vehicle_id, VcmpVehicleOption::BootOpen)
    }
    fn get_vehicle_option_bonnet_open(&self, vehicle_id: VehicleId) -> bool {
        self.get_vehicle_option(vehicle_id, VcmpVehicleOption::BonnetOpen)
    }
}

pub trait SetVehicleOptions {
    fn set_vehicle_option_doors_locked(
        &self,
        vehicle_id: VehicleId,
        locked: bool,
    ) -> VcmpResult<()>;
    fn set_vehicle_option_alarm(&self, vehicle_id: VehicleId, alarm: bool) -> VcmpResult<()>;
    fn set_vehicle_option_lights(&self, vehicle_id: VehicleId, lights: bool) -> VcmpResult<()>;
    fn set_vehicle_option_radio_locked(
        &self,
        vehicle_id: VehicleId,
        locked: bool,
    ) -> VcmpResult<()>;
    fn set_vehicle_option_ghost(&self, vehicle_id: VehicleId, ghost: bool) -> VcmpResult<()>;
    fn set_vehicle_option_siren(&self, vehicle_id: VehicleId, siren: bool) -> VcmpResult<()>;
    fn set_vehicle_option_single_use(
        &self,
        vehicle_id: VehicleId,
        single_use: bool,
    ) -> VcmpResult<()>;
    fn set_vehicle_option_engine_disabled(
        &self,
        vehicle_id: VehicleId,
        disabled: bool,
    ) -> VcmpResult<()>;
    fn set_vehicle_option_boot_open(&self, vehicle_id: VehicleId, open: bool) -> VcmpResult<()>;
    fn set_vehicle_option_bonnet_open(&self, vehicle_id: VehicleId, open: bool) -> VcmpResult<()>;
}
impl SetVehicleOptions for VcmpFunctions {
    fn set_vehicle_option_doors_locked(
        &self,
        vehicle_id: VehicleId,
        locked: bool,
    ) -> VcmpResult<()> {
        self.set_vehicle_option(vehicle_id, VcmpVehicleOption::DoorsLocked, locked)
    }
    fn set_vehicle_option_alarm(&self, vehicle_id: VehicleId, alarm: bool) -> VcmpResult<()> {
        self.set_vehicle_option(vehicle_id, VcmpVehicleOption::Alarm, alarm)
    }
    fn set_vehicle_option_lights(&self, vehicle_id: VehicleId, lights: bool) -> VcmpResult<()> {
        self.set_vehicle_option(vehicle_id, VcmpVehicleOption::Lights, lights)
    }
    fn set_vehicle_option_radio_locked(
        &self,
        vehicle_id: VehicleId,
        locked: bool,
    ) -> VcmpResult<()> {
        self.set_vehicle_option(vehicle_id, VcmpVehicleOption::RadioLocked, locked)
    }
    fn set_vehicle_option_ghost(&self, vehicle_id: VehicleId, ghost: bool) -> VcmpResult<()> {
        self.set_vehicle_option(vehicle_id, VcmpVehicleOption::Ghost, ghost)
    }
    fn set_vehicle_option_siren(&self, vehicle_id: VehicleId, siren: bool) -> VcmpResult<()> {
        self.set_vehicle_option(vehicle_id, VcmpVehicleOption::Siren, siren)
    }
    fn set_vehicle_option_single_use(
        &self,
        vehicle_id: VehicleId,
        single_use: bool,
    ) -> VcmpResult<()> {
        self.set_vehicle_option(vehicle_id, VcmpVehicleOption::SingleUse, single_use)
    }
    fn set_vehicle_option_engine_disabled(
        &self,
        vehicle_id: VehicleId,
        disabled: bool,
    ) -> VcmpResult<()> {
        self.set_vehicle_option(vehicle_id, VcmpVehicleOption::EngineDisabled, disabled)
    }
    fn set_vehicle_option_boot_open(&self, vehicle_id: VehicleId, open: bool) -> VcmpResult<()> {
        self.set_vehicle_option(vehicle_id, VcmpVehicleOption::BootOpen, open)
    }
    fn set_vehicle_option_bonnet_open(&self, vehicle_id: VehicleId, open: bool) -> VcmpResult<()> {
        self.set_vehicle_option(vehicle_id, VcmpVehicleOption::BonnetOpen, open)
    }
}

pub trait SetVehicle {
    fn set_vehicle_world(&self, vehicle_id: VehicleId, world_id: i32) -> VcmpResult<()>;
    fn respawn_vehicle(&self, vehicle_id: VehicleId) -> VcmpResult<()>;
    fn set_vehicle_immunity(&self, vehicle_id: VehicleId, immunity: u32) -> VcmpResult<()>;
    fn explode_vehicle(&self, vehicle_id: VehicleId) -> VcmpResult<()>;
    fn set_vehicle_position(
        &self,
        vehicle_id: VehicleId,
        position: Vectorf32,
        remove_occupants: Option<bool>,
    ) -> VcmpResult<()>;
    fn set_vehicle_rotation(
        &self,
        vehicle_id: VehicleId,
        rotation: Quaternionf32,
    ) -> VcmpResult<()>;
    fn set_vehicle_rotation_euler(
        &self,
        vehicle_id: VehicleId,
        rotation: Vectorf32,
    ) -> VcmpResult<()>;
    fn set_vehicle_speed(&self, vehicle_id: VehicleId, speed: Vectorf32) -> VcmpResult<()>;
    fn set_vehicle_rel_speed(&self, vehicle_id: VehicleId, speed: Vectorf32) -> VcmpResult<()>;
    fn set_vehicle_turn_speed(&self, vehicle_id: VehicleId, speed: Vectorf32) -> VcmpResult<()>;
    fn set_vehicle_rel_turn_speed(&self, vehicle_id: VehicleId, speed: Vectorf32)
    -> VcmpResult<()>;
    fn set_vehicle_add_speed(&self, vehicle_id: VehicleId, speed: Vectorf32) -> VcmpResult<()>;
    fn set_vehicle_add_rel_speed(&self, vehicle_id: VehicleId, speed: Vectorf32) -> VcmpResult<()>;
    fn set_vehicle_add_turn_speed(&self, vehicle_id: VehicleId, speed: Vectorf32)
    -> VcmpResult<()>;
    fn set_vehicle_add_rel_turn_speed(
        &self,
        vehicle_id: VehicleId,
        speed: Vectorf32,
    ) -> VcmpResult<()>;
    fn set_vehicle_spawn_position(
        &self,
        vehicle_id: VehicleId,
        position: Vectorf32,
    ) -> VcmpResult<()>;
    fn set_vehicle_spawn_rotation(
        &self,
        vehicle_id: VehicleId,
        rotation: Quaternionf32,
    ) -> VcmpResult<()>;
    fn set_vehicle_spawn_rotation_euler(
        &self,
        vehicle_id: VehicleId,
        rotation: Vectorf32,
    ) -> VcmpResult<()>;
    fn set_vehicle_idle_respawn_timer(&self, vehicle_id: VehicleId, timer: u32) -> VcmpResult<()>;
    fn set_vehicle_health(&self, vehicle_id: VehicleId, health: f32) -> VcmpResult<()>;
    fn set_vehicle_color(
        &self,
        vehicle_id: VehicleId,
        primary_color: i32,
        secondary_color: i32,
    ) -> VcmpResult<()>;
    fn set_vehicle_part_status(
        &self,
        vehicle_id: VehicleId,
        part_id: i32,
        status: i32,
    ) -> VcmpResult<()>;
    fn set_vehicle_tyre_status(
        &self,
        vehicle_id: VehicleId,
        tyre_id: i32,
        status: i32,
    ) -> VcmpResult<()>;
    fn set_vehicle_damage_data(&self, vehicle_id: VehicleId, data: u32) -> VcmpResult<()>;
    fn set_vehicle_radio(&self, vehicle_id: VehicleId, radio_id: i32) -> VcmpResult<()>;
    fn set_vehicle_lights_data(&self, vehicle_id: VehicleId, data: u32) -> VcmpResult<()>;
}
impl SetVehicle for VcmpFunctions {
    fn set_vehicle_world(&self, vehicle_id: VehicleId, world_id: i32) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleWorld)(vehicle_id, world_id);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn respawn_vehicle(&self, vehicle_id: VehicleId) -> VcmpResult<()> {
        let code = (self.inner.RespawnVehicle)(vehicle_id);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_immunity(&self, vehicle_id: VehicleId, immunity: u32) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleImmunityFlags)(vehicle_id, immunity);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn explode_vehicle(&self, vehicle_id: VehicleId) -> VcmpResult<()> {
        let code = (self.inner.ExplodeVehicle)(vehicle_id);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_position(
        &self,
        vehicle_id: VehicleId,
        position: Vectorf32,
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
    fn set_vehicle_rotation(
        &self,
        vehicle_id: VehicleId,
        rotation: Quaternionf32,
    ) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleRotation)(
            vehicle_id, rotation.x, rotation.y, rotation.z, rotation.w,
        );
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_rotation_euler(
        &self,
        vehicle_id: VehicleId,
        rotation: Vectorf32,
    ) -> VcmpResult<()> {
        let code =
            (self.inner.SetVehicleRotationEuler)(vehicle_id, rotation.x, rotation.y, rotation.z);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_speed(&self, vehicle_id: VehicleId, speed: Vectorf32) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleSpeed)(vehicle_id, speed.x, speed.y, speed.z, 0, 0);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_rel_speed(&self, vehicle_id: VehicleId, speed: Vectorf32) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleSpeed)(vehicle_id, speed.x, speed.y, speed.z, 0, 1);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_turn_speed(&self, vehicle_id: VehicleId, speed: Vectorf32) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleTurnSpeed)(vehicle_id, speed.x, speed.y, speed.z, 0, 0);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_rel_turn_speed(
        &self,
        vehicle_id: VehicleId,
        speed: Vectorf32,
    ) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleTurnSpeed)(vehicle_id, speed.x, speed.y, speed.z, 0, 1);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_add_speed(&self, vehicle_id: VehicleId, speed: Vectorf32) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleSpeed)(vehicle_id, speed.x, speed.y, speed.z, 1, 0);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_add_rel_speed(&self, vehicle_id: VehicleId, speed: Vectorf32) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleSpeed)(vehicle_id, speed.x, speed.y, speed.z, 1, 1);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_add_turn_speed(
        &self,
        vehicle_id: VehicleId,
        speed: Vectorf32,
    ) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleTurnSpeed)(vehicle_id, speed.x, speed.y, speed.z, 1, 0);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_add_rel_turn_speed(
        &self,
        vehicle_id: VehicleId,
        speed: Vectorf32,
    ) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleTurnSpeed)(vehicle_id, speed.x, speed.y, speed.z, 1, 1);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_spawn_position(
        &self,
        vehicle_id: VehicleId,
        position: Vectorf32,
    ) -> VcmpResult<()> {
        let code =
            (self.inner.SetVehicleSpawnPosition)(vehicle_id, position.x, position.y, position.z);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_spawn_rotation(
        &self,
        vehicle_id: VehicleId,
        rotation: Quaternionf32,
    ) -> VcmpResult<()> {
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
        vehicle_id: VehicleId,
        rotation: Vectorf32,
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
    fn set_vehicle_idle_respawn_timer(&self, vehicle_id: VehicleId, timer: u32) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleIdleRespawnTimer)(vehicle_id, timer);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_health(&self, vehicle_id: VehicleId, health: f32) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleHealth)(vehicle_id, health);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_color(
        &self,
        vehicle_id: VehicleId,
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
        vehicle_id: VehicleId,
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
        vehicle_id: VehicleId,
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
    fn set_vehicle_damage_data(&self, vehicle_id: VehicleId, data: u32) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleDamageData)(vehicle_id, data);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    fn set_vehicle_radio(&self, vehicle_id: VehicleId, radio_id: i32) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleRadio)(vehicle_id, radio_id);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn set_vehicle_lights_data(&self, vehicle_id: VehicleId, data: u32) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleLightsData)(vehicle_id, data);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
}

pub trait QueryVehicle {
    fn is_vehicle_alive(&self, vehicle_id: VehicleId) -> bool;
    fn is_vehicle_streamed_for_player(&self, player_id: PlayerId, vehicle_id: VehicleId) -> bool;
    fn get_vehicle_world(&self, vehicle_id: VehicleId) -> i32;
    fn get_vehicle_model(&self, vehicle_id: VehicleId) -> i32;
    fn get_vehicle_occupant(&self, vehicle_id: VehicleId, seat_id: i32) -> i32;
    fn get_vehicle_immunity(&self, vehicle_id: VehicleId) -> u32;
    fn is_vehicle_wrecked(&self, vehicle_id: VehicleId) -> bool;
    fn get_vehicle_position(&self, vehicle_id: VehicleId) -> Vectorf32;
    fn get_vehicle_rotation(&self, vehicle_id: VehicleId) -> Quaternionf32;
    fn get_vehicle_rotation_euler(&self, vehicle_id: VehicleId) -> Vectorf32;
    fn get_vehicle_speed(&self, vehicle_id: VehicleId) -> Vectorf32;
    fn get_vehicle_rel_speed(&self, vehicle_id: VehicleId) -> Vectorf32;
    fn get_vehicle_turn_speed(&self, vehicle_id: VehicleId) -> Vectorf32;
    fn get_vehicle_rel_turn_speed(&self, vehicle_id: VehicleId) -> Vectorf32;
    fn get_vehicle_spawn_position(&self, vehicle_id: VehicleId) -> Vectorf32; // Vector3<f32>
    fn get_vehicle_spawn_rotation(&self, vehicle_id: VehicleId) -> Quaternionf32;
    fn get_vehicle_spawn_rotation_euler(&self, vehicle_id: VehicleId) -> Vectorf32;
    fn get_vehicle_idle_respawn_timer(&self, vehicle_id: VehicleId) -> u32;
    fn get_vehicle_health(&self, vehicle_id: VehicleId) -> f32;
    fn get_vehicle_color(&self, vehicle_id: VehicleId) -> (i32, i32);
    fn get_vehicle_part_status(&self, vehicle_id: VehicleId, part_id: i32) -> bool;
    fn get_vehicle_tyre_status(&self, vehicle_id: VehicleId, tyre_id: i32) -> bool;
    fn get_vehicle_damage_data(&self, vehicle_id: VehicleId) -> u32;
    fn get_vehicle_radio(&self, vehicle_id: VehicleId) -> i32;
    fn get_vehicle_turret_rotation(&self, vehicle_id: VehicleId) -> (f32, f32);

    fn get_vehicle_sync_source(&self, vehicle_id: VehicleId) -> i32;
    fn get_vehicle_sync_type(&self, vehicle_id: VehicleId) -> i32;
    fn get_vehicle_lights_data(&self, vehicle_id: VehicleId) -> u32;
}
impl QueryVehicle for VcmpFunctions {
    fn is_vehicle_alive(&self, vehicle_id: VehicleId) -> bool {
        (self.inner.CheckEntityExists)(VcmpEntityPool::Vehicle.into(), vehicle_id) != 0
    }
    fn is_vehicle_streamed_for_player(&self, player_id: PlayerId, vehicle_id: VehicleId) -> bool {
        (self.inner.IsVehicleStreamedForPlayer)(player_id, vehicle_id) != 0
    }

    fn get_vehicle_world(&self, vehicle_id: VehicleId) -> i32 {
        (self.inner.GetVehicleWorld)(vehicle_id)
    }

    fn get_vehicle_model(&self, vehicle_id: VehicleId) -> i32 {
        (self.inner.GetVehicleModel)(vehicle_id)
    }

    fn get_vehicle_occupant(&self, vehicle_id: VehicleId, seat_id: i32) -> i32 {
        (self.inner.GetVehicleOccupant)(vehicle_id, seat_id)
    }

    fn get_vehicle_immunity(&self, vehicle_id: VehicleId) -> u32 {
        (self.inner.GetVehicleImmunityFlags)(vehicle_id)
    }

    fn is_vehicle_wrecked(&self, vehicle_id: VehicleId) -> bool {
        (self.inner.IsVehicleWrecked)(vehicle_id) != 0
    }

    fn get_vehicle_position(&self, vehicle_id: VehicleId) -> Vectorf32 {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        (self.inner.GetVehiclePosition)(vehicle_id, &mut x, &mut y, &mut z);
        Vectorf32::new(x, y, z)
    }

    fn get_vehicle_rotation(&self, vehicle_id: VehicleId) -> Quaternionf32 {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let mut w = 0.0;
        (self.inner.GetVehicleRotation)(vehicle_id, &mut x, &mut y, &mut z, &mut w);
        Quaternionf32::new(x, y, z, w)
    }

    fn get_vehicle_rotation_euler(&self, vehicle_id: VehicleId) -> Vectorf32 {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        (self.inner.GetVehicleRotationEuler)(vehicle_id, &mut x, &mut y, &mut z);
        Vectorf32::new(x, y, z)
    }

    fn get_vehicle_speed(&self, vehicle_id: VehicleId) -> Vectorf32 {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        (self.inner.GetVehicleSpeed)(vehicle_id, &mut x, &mut y, &mut z, 0);
        Vectorf32::new(x, y, z)
    }

    fn get_vehicle_rel_speed(&self, vehicle_id: VehicleId) -> Vectorf32 {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        (self.inner.GetVehicleSpeed)(vehicle_id, &mut x, &mut y, &mut z, 0);
        Vectorf32::new(x, y, z)
    }

    fn get_vehicle_turn_speed(&self, vehicle_id: VehicleId) -> Vectorf32 {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        (self.inner.GetVehicleTurnSpeed)(vehicle_id, &mut x, &mut y, &mut z, 0);
        Vectorf32::new(x, y, z)
    }

    fn get_vehicle_rel_turn_speed(&self, vehicle_id: VehicleId) -> Vectorf32 {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        (self.inner.GetVehicleTurnSpeed)(vehicle_id, &mut x, &mut y, &mut z, 1);
        Vectorf32::new(x, y, z)
    }

    fn get_vehicle_spawn_position(&self, vehicle_id: VehicleId) -> Vectorf32 {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        (self.inner.GetVehicleSpawnPosition)(vehicle_id, &mut x, &mut y, &mut z);
        Vectorf32::new(x, y, z)
    }

    fn get_vehicle_spawn_rotation(&self, vehicle_id: VehicleId) -> Quaternionf32 {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let mut w = 0.0;
        (self.inner.GetVehicleSpawnRotation)(vehicle_id, &mut x, &mut y, &mut z, &mut w);
        Quaternionf32::new(x, y, z, w)
    }

    fn get_vehicle_spawn_rotation_euler(&self, vehicle_id: VehicleId) -> Vectorf32 {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        (self.inner.GetVehicleSpawnRotationEuler)(vehicle_id, &mut x, &mut y, &mut z);
        Vectorf32::new(x, y, z)
    }

    fn get_vehicle_idle_respawn_timer(&self, vehicle_id: VehicleId) -> u32 {
        (self.inner.GetVehicleIdleRespawnTimer)(vehicle_id)
    }

    fn get_vehicle_health(&self, vehicle_id: VehicleId) -> f32 {
        (self.inner.GetVehicleHealth)(vehicle_id)
    }

    fn get_vehicle_color(&self, vehicle_id: VehicleId) -> (i32, i32) {
        let mut primary = 0;
        let mut secondary = 0;
        (self.inner.GetVehicleColour)(vehicle_id, &mut primary, &mut secondary);
        (primary, secondary)
    }

    fn get_vehicle_part_status(&self, vehicle_id: VehicleId, part_id: i32) -> bool {
        (self.inner.GetVehiclePartStatus)(vehicle_id, part_id) != 0
    }

    fn get_vehicle_tyre_status(&self, vehicle_id: VehicleId, tyre_id: i32) -> bool {
        (self.inner.GetVehicleTyreStatus)(vehicle_id, tyre_id) != 0
    }

    fn get_vehicle_damage_data(&self, vehicle_id: VehicleId) -> u32 {
        (self.inner.GetVehicleDamageData)(vehicle_id)
    }

    fn get_vehicle_radio(&self, vehicle_id: VehicleId) -> i32 {
        (self.inner.GetVehicleRadio)(vehicle_id)
    }

    fn get_vehicle_turret_rotation(&self, vehicle_id: VehicleId) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        (self.inner.GetVehicleTurretRotation)(vehicle_id, &mut x, &mut y);
        (x, y)
    }

    fn get_vehicle_sync_source(&self, vehicle_id: VehicleId) -> i32 {
        (self.inner.GetVehicleSyncSource)(vehicle_id)
    }

    fn get_vehicle_sync_type(&self, vehicle_id: VehicleId) -> i32 {
        (self.inner.GetVehicleSyncType)(vehicle_id)
    }

    fn get_vehicle_lights_data(&self, vehicle_id: VehicleId) -> u32 {
        (self.inner.GetVehicleLightsData)(vehicle_id)
    }
}

pub trait VehicleHandlingMethods {
    fn reset_all_vehicle_handlings(&self);

    fn exists_handling_rule(&self, model_index: i32, rule_index: i32) -> bool;

    fn set_handling_rule(&self, model_index: i32, rule_index: i32, value: f64) -> VcmpResult<()>;
    fn get_handling_rule(&self, model_index: i32, rule_index: i32) -> f64;

    fn reset_handling_rule(&self, model_index: i32, rule_index: i32) -> VcmpResult<()>;

    fn reset_handling(&self, model_index: i32) -> VcmpResult<()>;
    fn exists_inst_handling_rule(&self, vehicle_id: VehicleId, rule_index: i32) -> bool;

    fn set_inst_handling_rule(
        &self,
        vehicle_id: VehicleId,
        rule_index: i32,
        value: f64,
    ) -> VcmpResult<()>;
    fn get_inst_handling_rule(&self, vehicle_id: VehicleId, rule_index: i32) -> f64;

    fn reset_inst_handling_rule(&self, vehicle_id: VehicleId, rule_index: i32) -> VcmpResult<()>;

    fn reset_inst_handling(&self, vehicle_id: VehicleId) -> VcmpResult<()>;
}

impl VehicleHandlingMethods for VcmpFunctions {
    fn reset_all_vehicle_handlings(&self) {
        (self.inner.ResetAllVehicleHandlings)();
    }

    /// 检查操控规则是否存在
    fn exists_handling_rule(&self, model_index: i32, rule_index: i32) -> bool {
        (self.inner.ExistsHandlingRule)(model_index, rule_index) != 0
    }

    /// 设置操控规则
    fn set_handling_rule(&self, model_index: i32, rule_index: i32, value: f64) -> VcmpResult<()> {
        let code = (self.inner.SetHandlingRule)(model_index, rule_index, value);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    /// 获取操控规则
    fn get_handling_rule(&self, model_index: i32, rule_index: i32) -> f64 {
        (self.inner.GetHandlingRule)(model_index, rule_index)
    }

    /// 重置操控规则
    fn reset_handling_rule(&self, model_index: i32, rule_index: i32) -> VcmpResult<()> {
        let code = (self.inner.ResetHandlingRule)(model_index, rule_index);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    /// 重置车辆操控设置
    fn reset_handling(&self, model_index: i32) -> VcmpResult<()> {
        let code = (self.inner.ResetHandling)(model_index);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    /// 检查实例操控规则是否存在
    fn exists_inst_handling_rule(&self, vehicle_id: VehicleId, rule_index: i32) -> bool {
        (self.inner.ExistsInstHandlingRule)(vehicle_id, rule_index) != 0
    }

    /// 设置实例操控规则
    fn set_inst_handling_rule(
        &self,
        vehicle_id: VehicleId,
        rule_index: i32,
        value: f64,
    ) -> VcmpResult<()> {
        let code = (self.inner.SetInstHandlingRule)(vehicle_id, rule_index, value);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    /// 获取实例操控规则
    fn get_inst_handling_rule(&self, vehicle_id: VehicleId, rule_index: i32) -> f64 {
        (self.inner.GetInstHandlingRule)(vehicle_id, rule_index)
    }

    /// 重置实例操控规则
    fn reset_inst_handling_rule(&self, vehicle_id: VehicleId, rule_index: i32) -> VcmpResult<()> {
        let code = (self.inner.ResetInstHandlingRule)(vehicle_id, rule_index);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    /// 重置车辆实例操控设置
    fn reset_inst_handling(&self, vehicle_id: VehicleId) -> VcmpResult<()> {
        let code = (self.inner.ResetInstHandling)(vehicle_id);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
}
