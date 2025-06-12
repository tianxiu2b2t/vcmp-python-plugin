use crate::{
    VcmpError, VcmpResult,
    func::VcmpFunctions,
    utils::{Quaternionf32, Vectorf32},
};

pub trait ObjectMethods {
    fn create_object(&self, model_index: i32, world: i32, position: Vectorf32, alpha: i32) -> i32;

    fn delete_object(&self, object_id: i32) -> VcmpResult<()>;

    fn is_object_streamed_for_player(&self, object_id: i32, player_id: i32) -> bool;

    fn get_object_model(&self, object_id: i32) -> i32;

    fn set_object_world(&self, object_id: i32, world: i32) -> VcmpResult<()>;

    fn get_object_world(&self, object_id: i32) -> i32;

    fn set_object_alpha(&self, object_id: i32, alpha: i32, duration: u32) -> VcmpResult<()>;

    fn get_object_alpha(&self, object_id: i32) -> i32;

    fn move_object_to(
        &self,
        object_id: i32,
        x: f32,
        y: f32,
        z: f32,
        duration: u32,
    ) -> VcmpResult<()>;

    fn move_object_by(
        &self,
        object_id: i32,
        x: f32,
        y: f32,
        z: f32,
        duration: u32,
    ) -> VcmpResult<()>;

    fn set_object_position(&self, object_id: i32, x: f32, y: f32, z: f32) -> VcmpResult<()>;

    fn get_object_position(&self, object_id: i32) -> VcmpResult<Vectorf32>;

    fn rotate_object_to(
        &self,
        object_id: i32,
        rotation: Quaternionf32,
        duration: u32,
    ) -> VcmpResult<()>;

    fn rotate_object_to_euler(
        &self,
        object_id: i32,
        rotation: Vectorf32,
        duration: u32,
    ) -> VcmpResult<()>;

    fn rotate_object_by(
        &self,
        object_id: i32,
        rotation: Quaternionf32,
        duration: u32,
    ) -> VcmpResult<()>;

    fn rotate_object_by_euler(
        &self,
        object_id: i32,
        rotation: Vectorf32,
        duration: u32,
    ) -> VcmpResult<()>;

    fn get_object_rotation(&self, object_id: i32) -> VcmpResult<Quaternionf32>;

    fn get_object_rotation_euler(&self, object_id: i32) -> VcmpResult<Vectorf32>;

    fn set_object_shot_report_enabled(&self, object_id: i32, toggle: bool) -> VcmpResult<()>;

    fn is_object_shot_report_enabled(&self, object_id: i32) -> bool;

    fn set_object_touched_report_enabled(&self, object_id: i32, toggle: bool) -> VcmpResult<()>;

    fn is_object_touched_report_enabled(&self, object_id: i32) -> bool;
}

impl ObjectMethods for VcmpFunctions {
    fn create_object(&self, model_index: i32, world: i32, position: Vectorf32, alpha: i32) -> i32 {
        (self.inner.CreateObject)(
            model_index,
            world,
            position.x,
            position.y,
            position.z,
            alpha,
        )
    }

    fn delete_object(&self, object_id: i32) -> VcmpResult<()> {
        let code = (self.inner.DeleteObject)(object_id);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn is_object_streamed_for_player(&self, object_id: i32, player_id: i32) -> bool {
        (self.inner.IsObjectStreamedForPlayer)(object_id, player_id) != 0
    }

    fn get_object_model(&self, object_id: i32) -> i32 {
        (self.inner.GetObjectModel)(object_id)
    }

    fn set_object_world(&self, object_id: i32, world: i32) -> VcmpResult<()> {
        let code = (self.inner.SetObjectWorld)(object_id, world);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_object_world(&self, object_id: i32) -> i32 {
        (self.inner.GetObjectWorld)(object_id)
    }

    fn set_object_alpha(&self, object_id: i32, alpha: i32, duration: u32) -> VcmpResult<()> {
        let code = (self.inner.SetObjectAlpha)(object_id, alpha, duration);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_object_alpha(&self, object_id: i32) -> i32 {
        (self.inner.GetObjectAlpha)(object_id)
    }

    fn move_object_to(
        &self,
        object_id: i32,
        x: f32,
        y: f32,
        z: f32,
        duration: u32,
    ) -> VcmpResult<()> {
        let code = (self.inner.MoveObjectTo)(object_id, x, y, z, duration);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn move_object_by(
        &self,
        object_id: i32,
        x: f32,
        y: f32,
        z: f32,
        duration: u32,
    ) -> VcmpResult<()> {
        let code = (self.inner.MoveObjectBy)(object_id, x, y, z, duration);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn set_object_position(&self, object_id: i32, x: f32, y: f32, z: f32) -> VcmpResult<()> {
        let code = (self.inner.SetObjectPosition)(object_id, x, y, z);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_object_position(&self, object_id: i32) -> VcmpResult<Vectorf32> {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let code = (self.inner.GetObjectPosition)(object_id, &mut x, &mut y, &mut z);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(Vectorf32 { x, y, z })
        }
    }

    fn rotate_object_to(
        &self,
        object_id: i32,
        rotation: Quaternionf32,
        duration: u32,
    ) -> VcmpResult<()> {
        let code = (self.inner.RotateObjectTo)(
            object_id, rotation.x, rotation.y, rotation.z, rotation.w, duration,
        );
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn rotate_object_to_euler(
        &self,
        object_id: i32,
        rotation: Vectorf32,
        duration: u32,
    ) -> VcmpResult<()> {
        let code = (self.inner.RotateObjectToEuler)(
            object_id, rotation.x, rotation.y, rotation.z, duration,
        );
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn rotate_object_by(
        &self,
        object_id: i32,
        rotation: Quaternionf32,
        duration: u32,
    ) -> VcmpResult<()> {
        let code = (self.inner.RotateObjectBy)(
            object_id, rotation.x, rotation.y, rotation.z, rotation.w, duration,
        );
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn rotate_object_by_euler(
        &self,
        object_id: i32,
        rotation: Vectorf32,
        duration: u32,
    ) -> VcmpResult<()> {
        let code = (self.inner.RotateObjectByEuler)(
            object_id, rotation.x, rotation.y, rotation.z, duration,
        );
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_object_rotation(&self, object_id: i32) -> VcmpResult<Quaternionf32> {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let mut w = 0.0;
        let code = (self.inner.GetObjectRotation)(object_id, &mut x, &mut y, &mut z, &mut w);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(Quaternionf32 { x, y, z, w })
        }
    }

    fn get_object_rotation_euler(&self, object_id: i32) -> VcmpResult<Vectorf32> {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let code = (self.inner.GetObjectRotationEuler)(object_id, &mut x, &mut y, &mut z);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(Vectorf32 { x, y, z })
        }
    }

    fn set_object_shot_report_enabled(&self, object_id: i32, toggle: bool) -> VcmpResult<()> {
        let code = (self.inner.SetObjectShotReportEnabled)(object_id, toggle as u8);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn is_object_shot_report_enabled(&self, object_id: i32) -> bool {
        (self.inner.IsObjectShotReportEnabled)(object_id) != 0
    }

    fn set_object_touched_report_enabled(&self, object_id: i32, toggle: bool) -> VcmpResult<()> {
        let code = (self.inner.SetObjectTouchedReportEnabled)(object_id, toggle as u8);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn is_object_touched_report_enabled(&self, object_id: i32) -> bool {
        (self.inner.IsObjectTouchedReportEnabled)(object_id) != 0
    }
}
