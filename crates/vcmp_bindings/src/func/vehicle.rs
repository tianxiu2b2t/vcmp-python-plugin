use crate::{VcmpFunctions, Color, Vector};

pub trait QueryVehicle {
    pub fn is_vehicle_alive(&self, vehicle_id: i32) -> bool;
    pub fn is_vehicle_streamed_for_player(&self, player_id: i32, vehicle_id: i32) -> bool;
    pub fn get_vehicle_world(&self, vehicle_id: i32) -> i32;
    pub fn get_vehicle_model(&self, vehicle_id: i32) -> i32;
    pub fn get_vehicle_occupant(&self, vehicle_id: i32, seat_id: i32) -> i32;
    pub fn get_vehicle_immunity(&self, vehicle_id: i32) -> i32;
    pub fn is_vehicle_wrecked(&self, vehicle_id: i32) -> bool;
    pub fn get_vehicle_position(&self, vehicle_id: i32) -> Vector;
    pub fn get_vehicle_rotation(&self, vehicle_id: i32) -> Quaternion;
    pub fn get_vehicle_rotation_euler(&self, vehicle_id: i32) -> Vector;
    pub fn get_vehicle_speed(&self, vehicle_id: i32) -> Vector;
    pub fn get_vehicle_rel_speed(&self, vehicle_id: i32) -> Vector;
    pub fn get_vehicle_turn_speed(&self, vehicle_id: i32) -> Vector;
    pub fn get_vehicle_rel_turn_speed(&self, vehicle_id: i32) -> Vector;
    pub fn get_vehicle_spawn_position(&self, vehicle_id: i32) -> Vector; // Vector3<f32>
    pub fn get_vehicle_spawn_rotation(&self, vehicle_id: i32) -> Quaternion;
    pub fn get_vehicle_spawn_rotation_euler(&self, vehicle_id: i32) -> Vector;
    pub fn get_vehicle_idle_respawn_timer(&self, vehicle_id: i32) -> i32;
    pub fn get_vehicle_health(&self, vehicle_id: i32) -> f32;
    pub fn get_vehicle_color(&self, vehicle_id: i32) -> (i32, i32);
    pub fn get_vehicle_part_status(&self, vehicle_id: i32, part_id: i32) -> bool;
    pub fn get_vehicle_tyre_status(&self, vehicle_id: i32, tyre_id: i32) -> bool;
    pub fn get_vehicle_damage_data(&self, vehicle_id: i32) -> f32;
    pub fn get_vehicle_radio(&self, vehicle_id: i32) -> i32;
    pub fn get_vehicle_turret_rotation(&self, vehicle_id: i32) -> (f32, f32);
}

pub trait SetVehicle {
    pub fn set_vehicle_world(&self, vehicle_id: i32, world_id: i32);
    pub fn respawn_vehicle(&self, vehicle_id: i32);
    pub fn set_vehicle_immunity(&self, vehicle_id: i32, immunity: i32);
    pub fn explode_vehicle(&self, vehicle_id: i32);
    pub fn set_vehicle_position(&self, vehicle_id: i32, position: Vector, remove_occupants: Option<bool>);
    pub fn set_vehicle_rotation(&self, vehicle_id: i32, rotation: Quaternion);
    pub fn set_vehicle_rotation_euler(&self, vehicle_id: i32, rotation: Vector);

    pub fn set_vehicle_speed(&self, vehicle_id: i32, speed: Vector);
    pub fn set_vehicle_rel_speed(&self, vehicle_id: i32, speed: Vector);
    pub fn set_vehicle_turn_speed(&self, vehicle_id: i32, speed: Vector);
    pub fn set_vehicle_rel_turn_speed(&self, vehicle_id: i32, speed: Vector);

    pub fn set_vehicle_add_speed(&self, vehicle_id: i32, speed: Vector);
    pub fn set_vehicle_add_rel_speed(&self, vehicle_id: i32, speed: Vector);
    pub fn set_vehicle_add_turn_speed(&self, vehicle_id: i32, speed: Vector);
    pub fn set_vehicle_add_rel_turn_speed(&self, vehicle_id: i32, speed: Vector);

    pub fn set_vehicle_spawn_position(&self, vehicle_id: i32, position: Vector);
    pub fn set_vehicle_spawn_rotation(&self, vehicle_id: i32, rotation: Quaternion);
    pub fn set_vehicle_spawn_rotation_euler(&self, vehicle_id: i32, rotation: Vector);

    pub fn set_vehicle_idle_respawn_timer(&self, vehicle_id: i32, timer: i32);
    pub fn set_vehicle_health(&self, vehicle_id: i32, health: f32);
    pub fn set_vehicle_color(&self, vehicle_id: i32, primary_color: i32, secondary_color: i32);
    pub fn set_vehicle_part_status(&self, vehicle_id: i32, part_id: i32, status: i32);
    pub fn set_vehicle_tyre_status(&self, vehicle_id: i32, tyre_id: i32, status: i32);
    pub fn set_vehicle_damage_data(&self, vehicle_id: i32, data: f32);
    pub fn set_vehicle_radio(&self, vehicle_id: i32, radio_id: i32);

}

impl QueryVehicle for VcmpFunctions {
}
