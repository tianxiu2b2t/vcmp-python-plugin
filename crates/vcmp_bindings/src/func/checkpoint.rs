use crate::{
    PlayerId, VcmpError, VcmpResult,
    func::VcmpFunctions,
    options::VcmpEntityPool,
    utils::{Color, Vectorf32},
};

pub trait CheckPointMethods {
    fn create_checkpoint(
        &self,
        player_id: Option<i32>,
        world: i32,
        is_sphere: bool,
        position: Vectorf32,
        color: Color,
        radius: f32,
    ) -> i32;

    fn is_checkpoint_alive(&self, checkpoint_id: i32) -> bool;

    fn delete_checkpoint(&self, checkpoint_id: i32) -> VcmpResult<()>;

    fn is_checkpoint_streamed_for_player(&self, checkpoint_id: i32, player_id: PlayerId) -> bool;

    fn is_checkpoint_sphere(&self, checkpoint_id: i32) -> bool;

    fn set_checkpoint_world(&self, checkpoint_id: i32, world: i32) -> VcmpResult<()>;

    fn get_checkpoint_world(&self, checkpoint_id: i32) -> i32;

    fn set_checkpoint_colour(&self, checkpoint_id: i32, color: Color) -> VcmpResult<()>;

    fn get_checkpoint_colour(&self, checkpoint_id: i32) -> VcmpResult<Color>;

    fn set_checkpoint_position(&self, checkpoint_id: i32, position: Vectorf32) -> VcmpResult<()>;

    fn get_checkpoint_position(&self, checkpoint_id: i32) -> VcmpResult<Vectorf32>;

    fn set_checkpoint_radius(&self, checkpoint_id: i32, radius: f32) -> VcmpResult<()>;

    fn get_checkpoint_radius(&self, checkpoint_id: i32) -> f32;

    fn get_checkpoint_owner(&self, checkpoint_id: i32) -> i32;
}

impl CheckPointMethods for VcmpFunctions {
    fn is_checkpoint_alive(&self, checkpoint_id: i32) -> bool {
        (self.inner.CheckEntityExists)(VcmpEntityPool::CheckPoint.into(), checkpoint_id) != 0
    }
    fn create_checkpoint(
        &self,
        player_id: Option<i32>,
        world: i32,
        is_sphere: bool,
        position: Vectorf32,
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

    fn delete_checkpoint(&self, checkpoint_id: i32) -> VcmpResult<()> {
        let code = (self.inner.DeleteCheckPoint)(checkpoint_id);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn is_checkpoint_streamed_for_player(&self, checkpoint_id: i32, player_id: PlayerId) -> bool {
        (self.inner.IsCheckPointStreamedForPlayer)(checkpoint_id, player_id) != 0
    }

    fn is_checkpoint_sphere(&self, checkpoint_id: i32) -> bool {
        (self.inner.IsCheckPointSphere)(checkpoint_id) != 0
    }

    fn set_checkpoint_world(&self, checkpoint_id: i32, world: i32) -> VcmpResult<()> {
        let code = (self.inner.SetCheckPointWorld)(checkpoint_id, world);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_checkpoint_world(&self, checkpoint_id: i32) -> i32 {
        (self.inner.GetCheckPointWorld)(checkpoint_id)
    }

    fn set_checkpoint_colour(&self, checkpoint_id: i32, color: Color) -> VcmpResult<()> {
        let code = (self.inner.SetCheckPointColour)(
            checkpoint_id,
            color.r as i32,
            color.g as i32,
            color.b as i32,
            color.a as i32,
        );
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_checkpoint_colour(&self, checkpoint_id: i32) -> VcmpResult<Color> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let mut alpha = 0;
        let code = (self.inner.GetCheckPointColour)(
            checkpoint_id,
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

    fn set_checkpoint_position(&self, checkpoint_id: i32, position: Vectorf32) -> VcmpResult<()> {
        let code =
            (self.inner.SetCheckPointPosition)(checkpoint_id, position.x, position.y, position.z);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_checkpoint_position(&self, checkpoint_id: i32) -> VcmpResult<Vectorf32> {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let code = (self.inner.GetCheckPointPosition)(checkpoint_id, &mut x, &mut y, &mut z);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(Vectorf32 { x, y, z })
        }
    }

    fn set_checkpoint_radius(&self, checkpoint_id: i32, radius: f32) -> VcmpResult<()> {
        let code = (self.inner.SetCheckPointRadius)(checkpoint_id, radius);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_checkpoint_radius(&self, checkpoint_id: i32) -> f32 {
        (self.inner.GetCheckPointRadius)(checkpoint_id)
    }

    fn get_checkpoint_owner(&self, checkpoint_id: i32) -> i32 {
        (self.inner.GetCheckPointOwner)(checkpoint_id)
    }
}
