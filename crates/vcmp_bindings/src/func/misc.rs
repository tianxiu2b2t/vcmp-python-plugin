use crate::func::VcmpFunctions;

pub trait MiscMethods {
    fn create_explosion(
        &self,
        world: i32,
        explosion_type: i32,
        pos: (f32, f32, f32),
        responsible_player: i32,
        on_ground: bool,
    );
    fn play_sound(&self, world: i32, sound: i32, x: f32, y: f32, z: f32);
    fn hide_map_object(&self, object_id: i32, pos: (f32, f32, f32));
    fn show_map_object(&self, object_id: i32, pos: (f32, f32, f32));
    fn show_all_map_objects(&self);
    fn add_radio_stream(&self, id: i32, name: &str, url: &str, listed: bool);
    fn remove_radio_stream(&self, id: i32);
}

impl MiscMethods for VcmpFunctions {
    fn create_explosion(
        &self,
        world: i32,
        explosion_type: i32,
        pos: (f32, f32, f32),
        responsible_player: i32,
        on_ground: bool,
    ) {
        (self.inner.CreateExplosion)(
            world,
            explosion_type,
            pos.0,
            pos.1,
            pos.2,
            responsible_player,
            on_ground as u8,
        );
    }
    fn play_sound(&self, world: i32, sound: i32, x: f32, y: f32, z: f32) {
        (self.inner.PlaySound)(world, sound, x, y, z);
    }
    fn hide_map_object(&self, object_id: i32, pos: (f32, f32, f32)) {
        let (x, y, z) = pos;
        let x = ((x * 10.0).floor() + 0.5) as i16;
        let y = ((y * 10.0).floor() + 0.5) as i16;
        let z = ((z * 10.0).floor() + 0.5) as i16;

        (self.inner.HideMapObject)(object_id, x, y, z);
    }

    fn show_map_object(&self, object_id: i32, pos: (f32, f32, f32)) {
        let (x, y, z) = pos;
        let x = ((x * 10.0).floor() + 0.5) as i16;
        let y = ((y * 10.0).floor() + 0.5) as i16;
        let z = ((z * 10.0).floor() + 0.5) as i16;

        (self.inner.ShowMapObject)(object_id, x, y, z);
    }

    fn show_all_map_objects(&self) {
        (self.inner.ShowAllMapObjects)();
    }
    fn add_radio_stream(&self, id: i32, name: &str, url: &str, listed: bool) {
        (self.inner.AddRadioStream)(id, name.as_ptr() as _, url.as_ptr() as _, listed as u8);
    }

    fn remove_radio_stream(&self, id: i32) {
        (self.inner.RemoveRadioStream)(id);
    }
}
