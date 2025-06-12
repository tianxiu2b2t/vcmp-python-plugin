use crate::func::VcmpFunctions;
use crate::utils::Vector;
use crate::{VcmpError, VcmpResult};

trait PickupMethods {
    fn create_pickup(
        &self,
        model_index: i32,
        world: i32,
        quantity: i32,
        position: Vector,
        alpha: i32,
        is_automatic: bool,
    ) -> i32;

    fn delete_pickup(&self, pickup_id: i32) -> VcmpResult<()>;

    fn is_pickup_streamed_for_player(&self, pickup_id: i32, player_id: i32) -> bool;

    fn set_pickup_world(&self, pickup_id: i32, world: i32) -> VcmpResult<()>;

    fn get_pickup_world(&self, pickup_id: i32) -> i32;

    fn set_pickup_alpha(&self, pickup_id: i32, alpha: i32) -> VcmpResult<()>;

    fn get_pickup_alpha(&self, pickup_id: i32) -> i32;
    fn set_pickup_is_automatic(&self, pickup_id: i32, toggle: bool) -> VcmpResult<()>;

    fn is_pickup_automatic(&self, pickup_id: i32) -> bool;

    fn set_pickup_auto_timer(&self, pickup_id: i32, duration_millis: u32) -> VcmpResult<()>;

    fn get_pickup_auto_timer(&self, pickup_id: i32) -> u32;

    fn refresh_pickup(&self, pickup_id: i32) -> VcmpResult<()>;

    fn set_pickup_position(&self, pickup_id: i32, position: Vector) -> VcmpResult<()>;

    fn get_pickup_position(&self, pickup_id: i32) -> VcmpResult<Vector>;

    fn get_pickup_model(&self, pickup_id: i32) -> i32;

    fn get_pickup_quantity(&self, pickup_id: i32) -> i32;

    fn is_pickup_single_use(&self, pickup_id: i32) -> bool;

    fn set_pickup_single_use(&self, pickup_id: i32, toggle: bool) -> VcmpResult<()>;
}

impl PickupMethods for VcmpFunctions {
    fn create_pickup(
        &self,
        model_index: i32,
        world: i32,
        quantity: i32,
        position: Vector,
        alpha: i32,
        is_automatic: bool,
    ) -> i32 {
        (self.inner.CreatePickup)(
            model_index,
            world,
            quantity,
            position.x,
            position.y,
            position.z,
            alpha,
            is_automatic as u8,
        )
    }

    fn delete_pickup(&self, pickup_id: i32) -> VcmpResult<()> {
        let code = (self.inner.DeletePickup)(pickup_id);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn is_pickup_streamed_for_player(&self, pickup_id: i32, player_id: i32) -> bool {
        (self.inner.IsPickupStreamedForPlayer)(pickup_id, player_id) != 0
    }

    fn set_pickup_world(&self, pickup_id: i32, world: i32) -> VcmpResult<()> {
        let code = (self.inner.SetPickupWorld)(pickup_id, world);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_pickup_world(&self, pickup_id: i32) -> i32 {
        (self.inner.GetPickupWorld)(pickup_id)
    }

    fn set_pickup_alpha(&self, pickup_id: i32, alpha: i32) -> VcmpResult<()> {
        let code = (self.inner.SetPickupAlpha)(pickup_id, alpha);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_pickup_alpha(&self, pickup_id: i32) -> i32 {
        (self.inner.GetPickupAlpha)(pickup_id)
    }

    fn set_pickup_is_automatic(&self, pickup_id: i32, toggle: bool) -> VcmpResult<()> {
        let code = (self.inner.SetPickupIsAutomatic)(pickup_id, toggle as u8);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn is_pickup_automatic(&self, pickup_id: i32) -> bool {
        (self.inner.IsPickupAutomatic)(pickup_id) != 0
    }

    fn set_pickup_auto_timer(&self, pickup_id: i32, duration_millis: u32) -> VcmpResult<()> {
        let code = (self.inner.SetPickupAutoTimer)(pickup_id, duration_millis);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_pickup_auto_timer(&self, pickup_id: i32) -> u32 {
        (self.inner.GetPickupAutoTimer)(pickup_id)
    }

    fn refresh_pickup(&self, pickup_id: i32) -> VcmpResult<()> {
        let code = (self.inner.RefreshPickup)(pickup_id);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn set_pickup_position(&self, pickup_id: i32, position: Vector) -> VcmpResult<()> {
        let code = (self.inner.SetPickupPosition)(pickup_id, position.x, position.y, position.z);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }

    fn get_pickup_position(&self, pickup_id: i32) -> VcmpResult<Vector> {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let code = (self.inner.GetPickupPosition)(pickup_id, &mut x, &mut y, &mut z);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(Vector { x, y, z })
        }
    }

    fn get_pickup_model(&self, pickup_id: i32) -> i32 {
        (self.inner.GetPickupModel)(pickup_id)
    }

    fn get_pickup_quantity(&self, pickup_id: i32) -> i32 {
        (self.inner.GetPickupQuantity)(pickup_id)
    }

    fn is_pickup_single_use(&self, pickup_id: i32) -> bool {
        (self.inner.GetPickupOption)(pickup_id, 0) != 0
    }

    fn set_pickup_single_use(&self, pickup_id: i32, toggle: bool) -> VcmpResult<()> {
        let code = (self.inner.SetPickupOption)(pickup_id, 0, toggle as u8);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
}
