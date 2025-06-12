use crate::{func::VcmpFunctions, utils::{Color, Vector}, VcmpError, VcmpResult};

pub trait CheckPointMethods {
fn create_check_point(
        &self,
        player_id: Option<i32>,
        world: i32,
        is_sphere: bool,
        position: Vector,
        color: Color,
        radius: f32,
    ) -> i32;

    fn delete_check_point(&self, check_point_id: i32) -> VcmpResult<()>;

    fn is_check_point_streamed_for_player(&self, check_point_id: i32, player_id: i32) -> bool;

    fn is_check_point_sphere(&self, check_point_id: i32) -> bool;

    fn set_check_point_world(&self, check_point_id: i32, world: i32) -> VcmpResult<()>;

    fn get_check_point_world(&self, check_point_id: i32) -> i32;

    fn set_check_point_colour(
        &self,
        check_point_id: i32,
        color: Color,
    ) -> VcmpResult<()>;

    fn get_check_point_colour(&self, check_point_id: i32) -> VcmpResult<Color>;

    fn set_check_point_position(
        &self,
        check_point_id: i32,
        position: Vector
    ) -> VcmpResult<()>;

    fn get_check_point_position(&self, check_point_id: i32) -> VcmpResult<Vector>;

    fn set_check_point_radius(&self, check_point_id: i32, radius: f32) -> VcmpResult<()>;

    fn get_check_point_radius(&self, check_point_id: i32) -> f32;

    fn get_check_point_owner(&self, check_point_id: i32) -> i32;
}

impl CheckPointMethods for VcmpFunctions {
    fn create_check_point(
        &self,
        player_id: Option<i32>,
        world: i32,
        is_sphere: bool,
        position: Vector,
        color: Color,
        radius: f32,
    ) -> i32 {
        (self.inner.CreateCheckPoint)(
            player_id.unwrap_or(-1),
            world,
            is_sphere as u8,
            position.x,
            position.y,
            position.z,
            color.r as i32,
            color.g as i32,
            color.b as i32,
            color.a as i32,
            radius,
        )
    }

    fn delete_check_point(&self, check_point_id: i32) -> VcmpResult<()> {
        let code = (self.inner.DeleteCheckPoint)(check_point_id);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn is_check_point_streamed_for_player(&self, check_point_id: i32, player_id: i32) -> bool {
        (self.inner.IsCheckPointStreamedForPlayer)(check_point_id, player_id) != 0
    }

    fn is_check_point_sphere(&self, check_point_id: i32) -> bool {
        (self.inner.IsCheckPointSphere)(check_point_id) != 0
    }

    fn set_check_point_world(&self, check_point_id: i32, world: i32) -> VcmpResult<()> {
        let code = (self.inner.SetCheckPointWorld)(check_point_id, world);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_check_point_world(&self, check_point_id: i32) -> i32 {
        (self.inner.GetCheckPointWorld)(check_point_id)
    }

    fn set_check_point_colour(
        &self,
        check_point_id: i32,
        color: Color,
    ) -> VcmpResult<()> {
        let code = (self.inner.SetCheckPointColour)(check_point_id, color.r as i32, color.g as i32, color.b as i32, color.a as i32);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_check_point_colour(&self, check_point_id: i32) -> VcmpResult<Color> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let mut alpha = 0;
        let code = (self.inner.GetCheckPointColour)(
            check_point_id,
            &mut red,
            &mut green,
            &mut blue,
            &mut alpha,
        );
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(Color {
                r: red as u8,
                g: green as u8,
                b: blue as u8,
                a: alpha as u8,
            })
        }
    }

    fn set_check_point_position(
        &self,
        check_point_id: i32,
        position: Vector
    ) -> VcmpResult<()> {
        let code = (self.inner.SetCheckPointPosition)(check_point_id, position.x, position.y, position.z);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_check_point_position(&self, check_point_id: i32) -> VcmpResult<Vector> {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let code = (self.inner.GetCheckPointPosition)(check_point_id, &mut x, &mut y, &mut z);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(Vector {x, y, z})
        }
    }

    fn set_check_point_radius(&self, check_point_id: i32, radius: f32) -> VcmpResult<()> {
        let code = (self.inner.SetCheckPointRadius)(check_point_id, radius);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_check_point_radius(&self, check_point_id: i32) -> f32 {
        (self.inner.GetCheckPointRadius)(check_point_id)
    }

    fn get_check_point_owner(&self, check_point_id: i32) -> i32 {
        (self.inner.GetCheckPointOwner)(check_point_id)
    }
}
