use crate::func::VcmpFunctions;
use crate::utils::{Color, Marker, Vector};

pub trait MarkerMethods {
    fn create_marker(
        &self,
        world: i32,
        pos: (f32, f32, f32),
        scale: i32,
        color: Color,
        sprite: i32,
        index: Option<i32>,
    ) -> i32;
    fn destory_marker(&self, marker: i32);
    fn get_marker_info(&self, marker: i32) -> Marker;
}

impl MarkerMethods for VcmpFunctions {
    fn create_marker(
        &self,
        world: i32,
        pos: (f32, f32, f32),
        scale: i32,
        color: Color,
        sprite: i32,
        index: Option<i32>,
    ) -> i32 {
        let idx = index.unwrap_or(-1);
        (self.inner.CreateCoordBlip)(idx, world, pos.0, pos.1, pos.2, scale, color.into(), sprite)
    }
    fn destory_marker(&self, marker: i32) {
        (self.inner.DestroyCoordBlip)(marker);
    }

    fn get_marker_info(&self, marker: i32) -> Marker {
        let (mut world, mut x, mut y, mut z, mut scale, mut color, mut sprite) =
            (0, 0.0, 0.0, 0.0, 0, 0, 0);
        (self.inner.GetCoordBlipInfo)(
            marker,
            &mut world,
            &mut x,
            &mut y,
            &mut z,
            &mut scale,
            &mut color,
            &mut sprite,
        );

        Marker {
            marker,
            world,
            position: Vector { x, y, z },
            scale,
            color: Color::from_argba,
            sprite,
        }
    }
}
