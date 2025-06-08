use std::fmt::Display;

/*
    Color 类
*/
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn from_rgb(value: u32, a: Option<u8>) -> Self {
        Self {
            r: (value >> 16) as u8,
            g: (value >> 8) as u8,
            b: value as u8,
            a: a.unwrap_or(255),
        }
    }
    pub fn from_rgba(value: u32) -> Self {
        Self {
            r: (value >> 24) as u8,
            g: (value >> 16) as u8,
            b: (value >> 8) as u8,
            a: value as u8,
        }
    }
    pub fn from_argb(value: u32) -> Self {
        Self {
            a: (value >> 24) as u8,
            r: (value >> 16) as u8,
            g: (value >> 8) as u8,
            b: value as u8,
        }
    }
    pub fn as_rgba(&self) -> u32 {
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8) | (self.a as u32)
    }
    pub fn as_argb(&self) -> u32 {
        ((self.a as u32) << 24) | ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
    pub fn as_rgb(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Color(r={}, g={}, b={}, a={})",
            self.r, self.g, self.b, self.a
        )
    }
}

/*
    Vector 类
*/

pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn distance_from(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector(x={}, y={}, z={})", self.x, self.y, self.z)
    }
}

/*
    Quaternion 类
*/

pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl Display for Quaternion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Quaternion(x={}, y={}, z={}, w={})",
            self.x, self.y, self.z, self.w
        )
    }
}

/*
    World Bounds
*/

pub struct WorldBounds {
    pub max_x: f32,
    pub max_y: f32,
    pub min_x: f32,
    pub min_y: f32,
}

impl WorldBounds {
    pub fn new(max_x: f32, max_y: f32, min_x: f32, min_y: f32) -> Self {
        Self {
            max_x,
            max_y,
            min_x,
            min_y,
        }
    }
}

impl Display for WorldBounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WorldBounds(max_x={}, max_y={}, min_x={}, min_y={})",
            self.max_x, self.max_y, self.min_x, self.min_y
        )
    }
}

/*
    Wasted Settings
*/

pub struct WastedSettings {
    pub death_timer: u32,
    pub fade_timer: u32,
    pub fade_in_speed: f32,
    pub fade_out_speed: f32,
    pub color: Color,
    pub corpse_fade_start: u32,
    pub corpse_fade_time: u32,
}

impl WastedSettings {
    pub fn new(
        death_timer: u32,
        fade_timer: u32,
        fade_in_speed: f32,
        fade_out_speed: f32,
        color: Color,
        corpse_fade_start: u32,
        corpse_fade_time: u32,
    ) -> Self {
        Self {
            death_timer,
            fade_timer,
            fade_in_speed,
            fade_out_speed,
            color,
            corpse_fade_start,
            corpse_fade_time,
        }
    }
}

impl Display for WastedSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WastedSettings(death_timer={}, fade_timer={}, fade_in_speed={}, fade_out_speed={}, color={}, corpse_fade_start={}, corpse_fade_time={})",
            self.death_timer,
            self.fade_timer,
            self.fade_in_speed,
            self.fade_out_speed,
            self.color,
            self.corpse_fade_start,
            self.corpse_fade_time
        )
    }
}

/*
    Keybind
*/

pub struct Keybind {
    pub slot: i32,
    pub can_release: bool,
    pub key: i32,
    pub key2: i32,
    pub key3: i32,
}

impl Keybind {
    pub fn new(slot: i32, can_release: bool, key: i32, key2: i32, key3: i32) -> Self {
        Self {
            slot,
            can_release,
            key,
            key2,
            key3,
        }
    }
}

/*
    Marker
*/

pub struct Marker {
    pub marker: i32,
    pub world: i32,
    pub position: Vector,
    pub scale: i32,
    pub color: Color,
    pub sprite: i32,
}

impl Marker {
    pub fn new(
        marker: i32,
        world: i32,
        position: Vector,
        scale: i32,
        color: Color,
        sprite: i32,
    ) -> Self {
        Self {
            marker,
            world,
            position,
            scale,
            color,
            sprite,
        }
    }
}
