use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

use pyo3::{
    Bound, PyResult, Python, pyclass, pymethods,
    types::{PyModule, PyModuleMethods},
};

use vcmp_bindings::{
    func::{
        CheckPointMethods, MarkerMethods, ObjectMethods, PickupMethods, PlayerMethods,
        QueryVehicle, SetVehicle,
    },
    setting::VcmpServerSettings,
    utils::Vectorf32,
    vcmp_func,
};
use vcmp_bindings::{
    options::VcmpEntityPool,
    utils::{Color, Quaternionf32, WastedSettings},
};

use crate::consts::EntityId;

#[derive(Clone, Debug, Copy)]
#[pyclass]
#[pyo3(name = "ServerSettings")]
pub struct ServerSettingsPy {
    pub inner: VcmpServerSettings,
}

impl From<VcmpServerSettings> for ServerSettingsPy {
    fn from(value: VcmpServerSettings) -> Self {
        Self { inner: value }
    }
}

#[pymethods]
impl ServerSettingsPy {
    #[getter]
    pub fn get_server_name(&self) -> String {
        self.inner.server_name()
    }

    #[getter]
    pub fn get_max_players(&self) -> u32 {
        self.inner.max_players()
    }

    #[getter]
    pub fn get_port(&self) -> u32 {
        self.inner.port()
    }

    #[getter]
    pub fn get_flags(&self) -> u32 {
        self.inner.flags()
    }

    fn __repr__(&self) -> String {
        format!(
            r#"ServerSettings(server_name='{}', max_players={}, port={}, flags={})"#,
            self.inner.server_name(),
            self.inner.max_players(),
            self.inner.port(),
            self.inner.flags()
        )
    }
}

#[derive(Clone, Debug, Copy)]
#[pyclass]
#[pyo3(name = "WastedSettings")]
pub struct WastedSettingsPy {
    pub inner: WastedSettings,
}

impl From<WastedSettings> for WastedSettingsPy {
    fn from(value: WastedSettings) -> Self {
        Self { inner: value }
    }
}

impl From<WastedSettingsPy> for WastedSettings {
    fn from(val: WastedSettingsPy) -> Self {
        val.inner
    }
}

#[pymethods]
impl WastedSettingsPy {
    #[new]
    pub fn new(
        death_timer: u32,
        fade_timer: u32,
        fade_in_speed: f32,
        fade_out_speed: f32,
        color: RGBPy,
        corpse_fade_start: u32,
        corpse_fade_time: u32,
    ) -> Self {
        Self {
            inner: WastedSettings::new(
                death_timer,
                fade_timer,
                fade_in_speed,
                fade_out_speed,
                color.into(),
                corpse_fade_start,
                corpse_fade_time,
            ),
        }
    }

    #[getter]
    pub fn get_death_timer(&self) -> u32 {
        self.inner.death_timer
    }

    #[getter]
    pub fn get_fade_timer(&self) -> u32 {
        self.inner.fade_timer
    }

    #[getter]
    pub fn get_fade_in_speed(&self) -> f32 {
        self.inner.fade_in_speed
    }

    #[getter]
    pub fn get_fade_out_speed(&self) -> f32 {
        self.inner.fade_out_speed
    }

    #[getter]
    pub fn get_color(&self) -> RGBPy {
        RGBPy::from(self.inner.color)
    }

    #[getter]
    pub fn get_corpse_fade_start(&self) -> u32 {
        self.inner.corpse_fade_start
    }

    #[getter]
    pub fn get_corpse_fade_time(&self) -> u32 {
        self.inner.corpse_fade_time
    }

    #[setter]
    pub fn set_death_timer(&mut self, value: u32) {
        self.inner.death_timer = value;
    }

    #[setter]
    pub fn set_fade_timer(&mut self, value: u32) {
        self.inner.fade_timer = value;
    }

    #[setter]
    pub fn set_fade_in_speed(&mut self, value: f32) {
        self.inner.fade_in_speed = value;
    }

    #[setter]
    pub fn set_fade_out_speed(&mut self, value: f32) {
        self.inner.fade_out_speed = value;
    }

    #[setter]
    pub fn set_color(&mut self, value: RGBPy) {
        self.inner.color = value.inner;
    }

    #[setter]
    pub fn set_corpse_fade_start(&mut self, value: u32) {
        self.inner.corpse_fade_start = value;
    }

    #[setter]
    pub fn set_corpse_fade_time(&mut self, value: u32) {
        self.inner.corpse_fade_time = value;
    }

    fn __repr__(&self) -> String {
        format!(
            r#"WastedSettings(death_timer={}, fade_timer={}, fade_in_speed={}, fade_out_speed={}, color={}, corpse_fade_start={}, corpse_fade_time={})"#,
            self.inner.death_timer,
            self.inner.fade_timer,
            self.inner.fade_in_speed,
            self.inner.fade_out_speed,
            self.inner.color,
            self.inner.corpse_fade_start,
            self.inner.corpse_fade_time
        )
    }
}

#[derive(Clone, Debug, Copy)]
#[pyclass]
#[pyo3(name = "RGB")]
pub struct RGBPy {
    pub inner: Color,
}

impl From<Color> for RGBPy {
    fn from(value: Color) -> Self {
        Self { inner: value }
    }
}

impl From<RGBPy> for Color {
    fn from(val: RGBPy) -> Self {
        val.inner
    }
}

#[pymethods]
impl RGBPy {
    #[getter]
    pub fn get_r(&self) -> u8 {
        self.inner.r
    }

    #[getter]
    pub fn get_g(&self) -> u8 {
        self.inner.g
    }

    #[getter]
    pub fn get_b(&self) -> u8 {
        self.inner.b
    }

    #[getter]
    pub fn get_a(&self) -> u8 {
        self.inner.a
    }

    #[setter]
    pub fn set_a(&mut self, value: u8) {
        self.inner.a = value;
    }

    #[setter]
    pub fn set_b(&mut self, value: u8) {
        self.inner.b = value;
    }

    #[setter]
    pub fn set_g(&mut self, value: u8) {
        self.inner.g = value;
    }

    #[setter]
    pub fn set_r(&mut self, value: u8) {
        self.inner.r = value;
    }

    #[staticmethod]
    #[pyo3(signature = (value, a = None))]
    pub fn from_rgb(value: u32, a: Option<u8>) -> Self {
        Self {
            inner: Color::from_rgb(value, a),
        }
    }

    #[staticmethod]
    pub fn from_rgba(value: u32) -> Self {
        Self {
            inner: Color::from_rgba(value),
        }
    }

    #[staticmethod]
    pub fn from_argb(value: u32) -> Self {
        Self {
            inner: Color::from_argb(value),
        }
    }

    pub fn as_rgba(&self) -> u32 {
        self.inner.as_rgba()
    }

    pub fn as_argb(&self) -> u32 {
        self.inner.as_argb()
    }

    pub fn as_rgb(&self) -> u32 {
        self.inner.as_rgb()
    }

    pub fn as_hex(&self) -> String {
        format!("#{:06X}", self.as_rgb())
    }

    pub fn as_hexa(&self) -> String {
        format!("#{:08X}", self.as_rgba())
    }

    #[new]
    #[pyo3(signature = (r, g, b, a = 255))]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            inner: Color::from_rgba(
                ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32),
            ),
        }
    }

    fn __repr__(&self) -> String {
        let (r, b, g, a) = (self.inner.r, self.inner.b, self.inner.g, self.inner.a);
        format!("RGB(r={r}, b={b}, g={g}, a={a})")
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntityVectorType {
    PlayerPosition,
    PlayerSpeed,
    VehiclePosition,
    VehicleSpeed,
    VehicleTurnSpeed,
    VehicleRelSpeed,
    VehicleRelTurnSpeed,
    VehicleRotationEuler,
    VehicleSpawnRotationEuler,
    VehicleSpawnPosition,
    ObjectPosition,
    ObjectRotationEuler,
    PickupPosition,
    CheckPointPosition,
    MarkerPosition,
    Ignore,
}

#[derive(Clone, Copy)]
#[pyclass]
#[pyo3(name = "Vector")]
pub struct VectorPy {
    pub entity_type: EntityVectorType,
    pub entity_id: EntityId,
    pub inner: Option<Vectorf32>,
}

impl From<(EntityVectorType, EntityId)> for VectorPy {
    fn from(value: (EntityVectorType, EntityId)) -> Self {
        Self {
            entity_type: value.0,
            entity_id: value.1,
            inner: None,
        }
    }
}

impl From<VectorPy> for Vectorf32 {
    fn from(val: VectorPy) -> Self {
        val.get_entity_pos()
    }
}

impl From<Vectorf32> for VectorPy {
    fn from(val: Vectorf32) -> Self {
        Self {
            entity_type: EntityVectorType::Ignore,
            entity_id: 0,
            inner: Some(val),
        }
    }
}

impl VectorPy {
    pub fn get_entity_pos(&self) -> Vectorf32 {
        match self.entity_type {
            EntityVectorType::PlayerPosition => {
                let res = vcmp_func().get_player_position(self.entity_id);
                res.unwrap_or_default()
            }
            EntityVectorType::PlayerSpeed => {
                let res = vcmp_func().get_player_speed(self.entity_id);
                res.unwrap_or_default()
            }
            EntityVectorType::VehiclePosition => vcmp_func().get_vehicle_position(self.entity_id),
            EntityVectorType::VehicleSpeed => vcmp_func().get_vehicle_speed(self.entity_id),
            EntityVectorType::VehicleRelSpeed => vcmp_func().get_vehicle_rel_speed(self.entity_id),
            EntityVectorType::VehicleRelTurnSpeed => {
                vcmp_func().get_vehicle_rel_turn_speed(self.entity_id)
            }
            EntityVectorType::ObjectPosition => {
                let res = vcmp_func().get_object_position(self.entity_id);
                res.unwrap_or_default()
            }
            EntityVectorType::PickupPosition => {
                let res = vcmp_func().get_pickup_position(self.entity_id);
                res.unwrap_or_default()
            }
            EntityVectorType::CheckPointPosition => {
                let res = vcmp_func().get_checkpoint_position(self.entity_id);
                res.unwrap_or_default()
            }
            EntityVectorType::MarkerPosition => {
                let info = vcmp_func().get_marker_info(self.entity_id);
                info.position
            }
            EntityVectorType::VehicleTurnSpeed => {
                vcmp_func().get_vehicle_turn_speed(self.entity_id)
            }
            EntityVectorType::Ignore => self.inner.unwrap_or_default(),
            EntityVectorType::VehicleRotationEuler => {
                vcmp_func().get_vehicle_rotation_euler(self.entity_id)
            }
            EntityVectorType::VehicleSpawnRotationEuler => {
                vcmp_func().get_vehicle_spawn_rotation_euler(self.entity_id)
            }
            EntityVectorType::VehicleSpawnPosition => {
                vcmp_func().get_vehicle_spawn_position(self.entity_id)
            }
            EntityVectorType::ObjectRotationEuler => vcmp_func()
                .get_object_rotation_euler(self.entity_id)
                .unwrap_or_default(),
        }
    }

    pub fn set_entity_pos(&mut self, x: Option<f32>, y: Option<f32>, z: Option<f32>) {
        let mut origin = self.get_entity_pos();
        if let Some(x) = x {
            origin.x = x;
        }
        if let Some(y) = y {
            origin.y = y;
        }
        if let Some(z) = z {
            origin.z = z;
        }

        match self.entity_type {
            EntityVectorType::PlayerPosition => {
                let _ = vcmp_func().set_player_position(self.entity_id, origin);
            }
            EntityVectorType::PlayerSpeed => {
                let _ = vcmp_func().set_player_speed(self.entity_id, origin);
            }
            EntityVectorType::VehiclePosition => {
                let _ = vcmp_func().set_vehicle_position(self.entity_id, origin, Some(false));
            }
            EntityVectorType::VehicleSpeed => {
                let _ = vcmp_func().set_vehicle_speed(self.entity_id, origin);
            }
            EntityVectorType::VehicleRelSpeed => {
                let _ = vcmp_func().set_vehicle_rel_speed(self.entity_id, origin);
            }
            EntityVectorType::VehicleRelTurnSpeed => {
                let _ = vcmp_func().set_vehicle_rel_turn_speed(self.entity_id, origin);
            }
            EntityVectorType::ObjectPosition => {
                let _ = vcmp_func().set_object_position(self.entity_id, origin);
            }
            EntityVectorType::PickupPosition => {
                let _ = vcmp_func().set_pickup_position(self.entity_id, origin);
            }
            EntityVectorType::CheckPointPosition => {
                let _ = vcmp_func().set_checkpoint_position(self.entity_id, origin);
            }
            EntityVectorType::VehicleTurnSpeed => {
                let _ = vcmp_func().set_vehicle_turn_speed(self.entity_id, origin);
            }
            EntityVectorType::MarkerPosition => {
                // ignore
            }
            EntityVectorType::Ignore => {
                self.inner = Some(origin);
                // ignore
            }
            EntityVectorType::VehicleRotationEuler => {
                let _ = vcmp_func().set_vehicle_rotation_euler(self.entity_id, origin);
            }
            EntityVectorType::VehicleSpawnRotationEuler => {
                let _ = vcmp_func().set_vehicle_spawn_rotation_euler(self.entity_id, origin);
            }
            EntityVectorType::VehicleSpawnPosition => {
                let _ = vcmp_func().set_vehicle_spawn_position(self.entity_id, origin);
            }
            EntityVectorType::ObjectRotationEuler => {
                let _ = vcmp_func().rotate_object_to_euler(self.entity_id, origin, 0);
            }
        };
    }
}

impl Add for VectorPy {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        let origin = self.get_entity_pos();
        let other = rhs.get_entity_pos();

        self.set_entity_pos(
            Some(origin.x + other.x),
            Some(origin.y + other.y),
            Some(origin.z + other.z),
        );

        self
    }
}

impl Sub for VectorPy {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        let origin = self.get_entity_pos();
        let other = rhs.get_entity_pos();

        self.set_entity_pos(
            Some(origin.x - other.x),
            Some(origin.y - other.y),
            Some(origin.z - other.z),
        );

        self
    }
}

impl Div for VectorPy {
    type Output = Self;
    fn div(mut self, rhs: Self) -> Self {
        let origin = self.get_entity_pos();
        let other = rhs.get_entity_pos();

        self.set_entity_pos(
            Some(origin.x / other.x),
            Some(origin.y / other.y),
            Some(origin.z / other.z),
        );

        self
    }
}

impl Mul for VectorPy {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self {
        let origin = self.get_entity_pos();
        let other = rhs.get_entity_pos();

        self.set_entity_pos(
            Some(origin.x * other.x),
            Some(origin.y * other.y),
            Some(origin.z * other.z),
        );

        self
    }
}

impl Default for VectorPy {
    fn default() -> Self {
        Self {
            inner: None,
            entity_type: EntityVectorType::Ignore,
            entity_id: 0,
        }
    }
}

impl Debug for VectorPy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pos = self.get_entity_pos();
        if self.entity_type == EntityVectorType::Ignore {
            f.debug_struct("VectorPy")
                .field("x", &pos.x)
                .field("y", &pos.y)
                .field("z", &pos.z)
                .finish()
        } else {
            f.debug_struct("VectorPy")
                .field("entity_type", &self.entity_type)
                .field("entity_id", &self.entity_id)
                .field("x", &pos.x)
                .field("y", &pos.y)
                .field("z", &pos.z)
                .finish()
        }
    }
}

#[pymethods]
impl VectorPy {
    #[new]
    fn new(x: Option<f32>, y: Option<f32>, z: Option<f32>) -> Self {
        Self {
            inner: Some(Vectorf32::new(
                x.unwrap_or(0.0),
                y.unwrap_or(0.0),
                z.unwrap_or(0.0),
            )),
            entity_type: EntityVectorType::Ignore,
            entity_id: 0,
        }
    }

    #[getter]
    pub fn get_x(&self) -> f32 {
        self.get_entity_pos().x
    }

    #[getter]
    pub fn get_y(&self) -> f32 {
        self.get_entity_pos().y
    }

    #[getter]
    pub fn get_z(&self) -> f32 {
        self.get_entity_pos().z
    }

    #[setter]
    pub fn set_z(&mut self, py: Python<'_>, value: f32) {
        py.allow_threads(|| {
            self.set_entity_pos(None, None, Some(value));
        });
    }

    #[setter]
    pub fn set_y(&mut self, py: Python<'_>, value: f32) {
        py.allow_threads(|| {
            self.set_entity_pos(None, Some(value), None);
        });
    }

    #[setter]
    pub fn set_x(&mut self, py: Python<'_>, value: f32) {
        py.allow_threads(|| {
            self.set_entity_pos(Some(value), None, None);
        });
    }

    fn __repr__(&self) -> String {
        let pos = self.get_entity_pos();
        format!("Vector(x={}, y={}, z={})", pos.x, pos.y, pos.z)
    }

    // support - + / * for VectorPy
    fn __add__(&self, py: Python<'_>, other: &VectorPy) -> VectorPy {
        py.allow_threads(|| self.clone() + other.clone())
    }

    fn __sub__(&self, py: Python<'_>, other: &VectorPy) -> VectorPy {
        py.allow_threads(|| self.clone() - other.clone())
    }

    fn __mul__(&self, py: Python<'_>, other: &VectorPy) -> VectorPy {
        py.allow_threads(|| self.clone() * other.clone())
    }

    fn __truediv__(&self, py: Python<'_>, other: &VectorPy) -> VectorPy {
        py.allow_threads(|| self.clone() / other.clone())
    }

    pub fn distance(&self, py: Python<'_>, other: &VectorPy) -> f32 {
        py.allow_threads(|| {
            let origin = self.get_entity_pos();
            let other = other.get_entity_pos();

            ((origin.x - other.x).powi(2) + (origin.y - other.y).powi(2)).sqrt()
        })
    }

    pub fn distance_with_z(&self, py: Python<'_>, other: &VectorPy) -> f32 {
        py.allow_threads(|| {
            let origin = self.get_entity_pos();
            let other = other.get_entity_pos();

            ((origin.x - other.x).powi(2)
                + (origin.y - other.y).powi(2)
                + (origin.z - other.z).powi(2))
            .sqrt()
        })
    }

    // distance from
}

#[derive(Clone, Debug, Copy)]
pub enum EntityQuaternionType {
    VehicleRotation,
    VehicleSpawnRotation,
    ObjectRotation,
    Ignore,
}

#[derive(Clone, Debug, Copy)]
#[pyclass]
#[pyo3(name = "Quaternion")]
pub struct QuaternionPy {
    pub entity_type: EntityQuaternionType,
    pub entity_id: EntityId,
    pub inner: Option<Quaternionf32>,
}

impl From<(EntityQuaternionType, EntityId)> for QuaternionPy {
    fn from(value: (EntityQuaternionType, EntityId)) -> Self {
        Self {
            entity_type: value.0,
            entity_id: value.1,
            inner: None,
        }
    }
}

impl From<QuaternionPy> for Quaternionf32 {
    fn from(val: QuaternionPy) -> Self {
        val.get_entity_quaternion()
    }
}

impl From<Quaternionf32> for QuaternionPy {
    fn from(val: Quaternionf32) -> Self {
        Self {
            entity_type: EntityQuaternionType::Ignore,
            entity_id: 0,
            inner: Some(val),
        }
    }
}

impl QuaternionPy {
    pub fn get_entity_quaternion(&self) -> Quaternionf32 {
        match self.entity_type {
            EntityQuaternionType::Ignore => self.inner.unwrap_or_default(),
            EntityQuaternionType::VehicleRotation => {
                vcmp_func().get_vehicle_rotation(self.entity_id)
            }
            EntityQuaternionType::VehicleSpawnRotation => {
                vcmp_func().get_vehicle_spawn_rotation(self.entity_id)
            }
            EntityQuaternionType::ObjectRotation => vcmp_func()
                .get_object_rotation(self.entity_id)
                .unwrap_or_default(),
        }
    }

    pub fn set_entity_quaternion(
        &mut self,
        x: Option<f32>,
        y: Option<f32>,
        z: Option<f32>,
        w: Option<f32>,
    ) {
        let mut origin = self.get_entity_quaternion();
        if let Some(x) = x {
            origin.x = x;
        }
        if let Some(y) = y {
            origin.y = y;
        }
        if let Some(z) = z {
            origin.z = z;
        }
        if let Some(w) = w {
            origin.w = w;
        }

        match self.entity_type {
            EntityQuaternionType::Ignore => {
                self.inner = Some(origin);
                // ignore
            }
            EntityQuaternionType::VehicleRotation => {
                let _ = vcmp_func().set_vehicle_rotation(self.entity_id, origin);
            }
            EntityQuaternionType::VehicleSpawnRotation => {
                let _ = vcmp_func().set_vehicle_spawn_rotation(self.entity_id, origin);
            }
            EntityQuaternionType::ObjectRotation => {
                let _ = vcmp_func().rotate_object_to(self.entity_id, origin, 0);
            }
        };
    }
}

impl Add for QuaternionPy {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        let origin = self.get_entity_quaternion();
        let other = rhs.get_entity_quaternion();

        self.set_entity_quaternion(
            Some(origin.x + other.x),
            Some(origin.y + other.y),
            Some(origin.z + other.z),
            Some(origin.w + other.w),
        );

        self
    }
}

impl Sub for QuaternionPy {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        let origin = self.get_entity_quaternion();
        let other = rhs.get_entity_quaternion();

        self.set_entity_quaternion(
            Some(origin.x - other.x),
            Some(origin.y - other.y),
            Some(origin.z - other.z),
            Some(origin.w - other.w),
        );

        self
    }
}

impl Mul for QuaternionPy {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self {
        let origin = self.get_entity_quaternion();
        let other = rhs.get_entity_quaternion();

        self.set_entity_quaternion(
            Some(origin.x * other.x),
            Some(origin.y * other.y),
            Some(origin.z * other.z),
            Some(origin.w * other.w),
        );

        self
    }
}

impl Div for QuaternionPy {
    type Output = Self;
    fn div(mut self, rhs: Self) -> Self {
        let origin = self.get_entity_quaternion();
        let other = rhs.get_entity_quaternion();

        self.set_entity_quaternion(
            Some(origin.x / other.x),
            Some(origin.y / other.y),
            Some(origin.z / other.z),
            Some(origin.w / other.w),
        );

        self
    }
}

impl Default for QuaternionPy {
    fn default() -> Self {
        Self {
            inner: None,
            entity_type: EntityQuaternionType::Ignore,
            entity_id: 0,
        }
    }
}

#[pymethods]
impl QuaternionPy {
    #[new]
    fn new(x: Option<f32>, y: Option<f32>, z: Option<f32>, w: Option<f32>) -> Self {
        Self {
            inner: Some(Quaternionf32::new(
                x.unwrap_or(0.0),
                y.unwrap_or(0.0),
                z.unwrap_or(0.0),
                w.unwrap_or(1.0),
            )),
            entity_type: EntityQuaternionType::Ignore,
            entity_id: 0,
        }
    }

    #[getter]
    pub fn get_x(&self) -> f32 {
        self.get_entity_quaternion().x
    }

    #[getter]
    pub fn get_y(&self) -> f32 {
        self.get_entity_quaternion().y
    }

    #[getter]
    pub fn get_z(&self) -> f32 {
        self.get_entity_quaternion().z
    }

    #[getter]
    pub fn get_w(&self) -> f32 {
        self.get_entity_quaternion().w
    }

    #[setter]
    pub fn set_w(&mut self, py: Python<'_>, value: f32) {
        py.allow_threads(|| {
            self.set_entity_quaternion(None, None, None, Some(value));
        });
    }

    #[setter]
    pub fn set_z(&mut self, py: Python<'_>, value: f32) {
        py.allow_threads(|| {
            self.set_entity_quaternion(None, None, Some(value), None);
        });
    }

    #[setter]
    pub fn set_y(&mut self, py: Python<'_>, value: f32) {
        py.allow_threads(|| {
            self.set_entity_quaternion(None, Some(value), None, None);
        });
    }

    #[setter]
    pub fn set_x(&mut self, py: Python<'_>, value: f32) {
        py.allow_threads(|| {
            self.set_entity_quaternion(None, Some(value), None, None);
        });
    }

    fn __add__(&self, py: Python<'_>, other: &QuaternionPy) -> QuaternionPy {
        py.allow_threads(|| self.clone() + other.clone())
    }

    fn __sub__(&self, py: Python<'_>, other: &QuaternionPy) -> QuaternionPy {
        py.allow_threads(|| self.clone() - other.clone())
    }

    fn __mul__(&self, py: Python<'_>, other: &QuaternionPy) -> QuaternionPy {
        py.allow_threads(|| self.clone() * other.clone())
    }

    fn __div__(&self, py: Python<'_>, other: &QuaternionPy) -> QuaternionPy {
        py.allow_threads(|| self.clone() / other.clone())
    }

    fn __repr__(&self) -> String {
        let pos = self.get_entity_quaternion();
        format!(
            "Quaternion(x={}, y={}, z={}, w={})",
            pos.x, pos.y, pos.z, pos.w
        )
    }
}

#[pyclass]
#[derive(PartialEq, Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum Version {
    /// 66215
    v04rel002(),
    v04rel003(),
    v04rel004(),
    v04rel006(),
    v0_4_7_0(),
    v0_4_7_1(),
    Unknown(i32),
}

impl From<Version> for i32 {
    fn from(value: Version) -> Self {
        match value {
            Version::v04rel002() => 66215,
            Version::v04rel003() => 66230,
            Version::v04rel004() => 67000,
            Version::v04rel006() => 67400,
            Version::v0_4_7_0() => 67700,
            Version::v0_4_7_1() => 67710,
            Version::Unknown(x) => x,
        }
    }
}

#[pymethods]
impl Version {
    #[new]
    fn new(value: i32) -> Self {
        Self::from(value)
    }

    #[getter]
    fn value(&self) -> i32 {
        (*self).into()
    }

    fn __repr__(&self) -> String {
        match self {
            Version::v04rel002() => "04rel002(66215)".to_string(),
            Version::v04rel003() => "04rel003(66230)".to_string(),
            Version::v04rel004() => "04rel004(67000)".to_string(),
            Version::v04rel006() => "04rel006(67400)".to_string(),
            Version::v0_4_7_0() => "0.4.7.0(67700)".to_string(),
            Version::v0_4_7_1() => "0.4.7.1(67710)".to_string(),
            Version::Unknown(x) => format!("Unknown({})", x),
        }
    }
}

impl From<i32> for Version {
    fn from(value: i32) -> Self {
        match value {
            66215 => Version::v04rel002(),
            66230 => Version::v04rel003(),
            67000 => Version::v04rel004(),
            67400 => Version::v04rel006(),
            67700 => Version::v0_4_7_0(),
            67710 => Version::v0_4_7_1(),
            x => Version::Unknown(x),
        }
    }
}

#[pyclass]
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum KeyCode {
    VK_LBUTTON = 0x01,
    VK_RBUTTON = 0x02,
    VK_CANCEL = 0x03,
    VK_MBUTTON = 0x04,
    VK_XBUTTON1 = 0x05,
    VK_XBUTTON2 = 0x06,
    VK_BACK = 0x08,
    VK_TAB = 0x09,
    VK_CLEAR = 0x0C,
    VK_RETURN = 0x0D,
    VK_SHIFT = 0x10,
    VK_CONTROL = 0x11,
    VK_MENU = 0x12,
    VK_PAUSE = 0x13,
    VK_CAPITAL = 0x14,
    VK_KANA = 0x15,
    VK_JUNJA = 0x17,
    VK_FINAL = 0x18,
    VK_HANJA = 0x19,
    VK_IME_OFF = 0x1A,
    VK_ESCAPE = 0x1B,
    VK_CONVERT = 0x1C,
    VK_NONCONVERT = 0x1D,
    VK_ACCEPT = 0x1E,
    VK_MODECHANGE = 0x1F,
    VK_SPACE = 0x20,
    VK_PRIOR = 0x21,
    VK_NEXT = 0x22,
    VK_END = 0x23,
    VK_HOME = 0x24,
    VK_LEFT = 0x25,
    VK_UP = 0x26,
    VK_RIGHT = 0x27,
    VK_DOWN = 0x28,
    VK_SELECT = 0x29,
    VK_PRINT = 0x2A,
    VK_EXECUTE = 0x2B,
    VK_SNAPSHOT = 0x2C,
    VK_INSERT = 0x2D,
    VK_DELETE = 0x2E,
    VK_HELP = 0x2F,
    VK_0 = 0x30,
    VK_1 = 0x31,
    VK_2 = 0x32,
    VK_3 = 0x33,
    VK_4 = 0x34,
    VK_5 = 0x35,
    VK_6 = 0x36,
    VK_7 = 0x37,
    VK_8 = 0x38,
    VK_9 = 0x39,
    VK_A = 0x41,
    VK_B = 0x42,
    VK_C = 0x43,
    VK_D = 0x44,
    VK_E = 0x45,
    VK_F = 0x46,
    VK_G = 0x47,
    VK_H = 0x48,
    VK_I = 0x49,
    VK_J = 0x4A,
    VK_K = 0x4B,
    VK_L = 0x4C,
    VK_M = 0x4D,
    VK_N = 0x4E,
    VK_O = 0x4F,
    VK_P = 0x50,
    VK_Q = 0x51,
    VK_R = 0x52,
    VK_S = 0x53,
    VK_T = 0x54,
    VK_U = 0x55,
    VK_V = 0x56,
    VK_W = 0x57,
    VK_X = 0x58,
    VK_Y = 0x59,
    VK_Z = 0x5A,
    VK_LWIN = 0x5B,
    VK_RWIN = 0x5C,
    VK_APPS = 0x5D,
    VK_SLEEP = 0x5F,
    VK_NUMPAD0 = 0x60,
    VK_NUMPAD1 = 0x61,
    VK_NUMPAD2 = 0x62,
    VK_NUMPAD3 = 0x63,
    VK_NUMPAD4 = 0x64,
    VK_NUMPAD5 = 0x65,
    VK_NUMPAD6 = 0x66,
    VK_NUMPAD7 = 0x67,
    VK_NUMPAD8 = 0x68,
    VK_NUMPAD9 = 0x69,
    VK_MULTIPLY = 0x6A,
    VK_ADD = 0x6B,
    VK_SEPARATOR = 0x6C,
    VK_SUBTRACT = 0x6D,
    VK_DECIMAL = 0x6E,
    VK_DIVIDE = 0x6F,
    VK_F1 = 0x70,
    VK_F2 = 0x71,
    VK_F3 = 0x72,
    VK_F4 = 0x73,
    VK_F5 = 0x74,
    VK_F6 = 0x75,
    VK_F7 = 0x76,
    VK_F8 = 0x77,
    VK_F9 = 0x78,
    VK_F10 = 0x79,
    VK_F11 = 0x7A,
    VK_F12 = 0x7B,
    VK_F13 = 0x7C,
    VK_F14 = 0x7D,
    VK_F15 = 0x7E,
    VK_F16 = 0x7F,
    VK_F17 = 0x80,
    VK_F18 = 0x81,
    VK_F19 = 0x82,
    VK_F20 = 0x83,
    VK_F21 = 0x84,
    VK_F22 = 0x85,
    VK_F23 = 0x86,
    VK_F24 = 0x87,
    VK_NUMLOCK = 0x90,
    VK_SCROLL = 0x91,
    VK_LSHIFT = 0xA0,
    VK_RSHIFT = 0xA1,
    VK_LCONTROL = 0xA2,
    VK_RCONTROL = 0xA3,
    VK_LMENU = 0xA4,
    VK_RMENU = 0xA5,
    VK_BROWSER_BACK = 0xA6,
    VK_BROWSER_FORWARD = 0xA7,
    VK_BROWSER_REFRESH = 0xA8,
    VK_BROWSER_STOP = 0xA9,
    VK_BROWSER_SEARCH = 0xAA,
    VK_BROWSER_FAVORITES = 0xAB,
    VK_BROWSER_HOME = 0xAC,
    VK_VOLUME_MUTE = 0xAD,
    VK_VOLUME_DOWN = 0xAE,
    VK_VOLUME_UP = 0xAF,
    VK_MEDIA_NEXT_TRACK = 0xB0,
    VK_MEDIA_PREV_TRACK = 0xB1,
    VK_MEDIA_STOP = 0xB2,
    VK_MEDIA_PLAY_PAUSE = 0xB3,
    VK_LAUNCH_MAIL = 0xB4,
    VK_LAUNCH_MEDIA_SELECT = 0xB5,
    VK_LAUNCH_APP1 = 0xB6,
    VK_LAUNCH_APP2 = 0xB7,
    VK_OEM_1 = 0xBA,
    VK_OEM_PLUS = 0xBB,
    VK_OEM_COMMA = 0xBC,
    VK_OEM_MINUS = 0xBD,
    VK_OEM_PERIOD = 0xBE,
    VK_OEM_2 = 0xBF,
    VK_OEM_3 = 0xC0,
    VK_OEM_4 = 0xDB,
    VK_OEM_5 = 0xDC,
    VK_OEM_6 = 0xDD,
    VK_OEM_7 = 0xDE,
    VK_OEM_8 = 0xDF,
    VK_OEM_102 = 0xE2,
    VK_PROCESSKEY = 0xE5,
    VK_PACKET = 0xE7,
    VK_ATTN = 0xF6,
    VK_CRSEL = 0xF7,
    VK_EXSEL = 0xF8,
    VK_EREOF = 0xF9,
    VK_PLAY = 0xFA,
    VK_ZOOM = 0xFB,
    VK_NONAME = 0xFC,
    VK_PA1 = 0xFD,
    VK_OEM_CLEAR = 0xFE,
    VK_UNKNOWN = 0xFF,
}

impl From<i32> for KeyCode {
    fn from(value: i32) -> Self {
        match value {
            0x01 => KeyCode::VK_LBUTTON,
            0x02 => KeyCode::VK_RBUTTON,
            0x03 => KeyCode::VK_CANCEL,
            0x04 => KeyCode::VK_MBUTTON,
            0x05 => KeyCode::VK_XBUTTON1,
            0x06 => KeyCode::VK_XBUTTON2,
            0x08 => KeyCode::VK_BACK,
            0x09 => KeyCode::VK_TAB,
            0x0C => KeyCode::VK_CLEAR,
            0x0D => KeyCode::VK_RETURN,
            0x10 => KeyCode::VK_SHIFT,
            0x11 => KeyCode::VK_CONTROL,
            0x12 => KeyCode::VK_MENU,
            0x13 => KeyCode::VK_PAUSE,
            0x14 => KeyCode::VK_CAPITAL,
            0x15 => KeyCode::VK_KANA,
            0x17 => KeyCode::VK_JUNJA,
            0x18 => KeyCode::VK_FINAL,
            0x19 => KeyCode::VK_HANJA,
            0x1A => KeyCode::VK_IME_OFF,
            0x1B => KeyCode::VK_ESCAPE,
            0x1C => KeyCode::VK_CONVERT,
            0x1D => KeyCode::VK_NONCONVERT,
            0x1E => KeyCode::VK_ACCEPT,
            0x1F => KeyCode::VK_MODECHANGE,
            0x20 => KeyCode::VK_SPACE,
            0x21 => KeyCode::VK_PRIOR,
            0x22 => KeyCode::VK_NEXT,
            0x23 => KeyCode::VK_END,
            0x24 => KeyCode::VK_HOME,
            0x25 => KeyCode::VK_LEFT,
            0x26 => KeyCode::VK_UP,
            0x27 => KeyCode::VK_RIGHT,
            0x28 => KeyCode::VK_DOWN,
            0x29 => KeyCode::VK_SELECT,
            0x2A => KeyCode::VK_PRINT,
            0x2B => KeyCode::VK_EXECUTE,
            0x2C => KeyCode::VK_SNAPSHOT,
            0x2D => KeyCode::VK_INSERT,
            0x2E => KeyCode::VK_DELETE,
            0x2F => KeyCode::VK_HELP,
            0x30 => KeyCode::VK_0,
            0x31 => KeyCode::VK_1,
            0x32 => KeyCode::VK_2,
            0x33 => KeyCode::VK_3,
            0x34 => KeyCode::VK_4,
            0x35 => KeyCode::VK_5,
            0x36 => KeyCode::VK_6,
            0x37 => KeyCode::VK_7,
            0x38 => KeyCode::VK_8,
            0x39 => KeyCode::VK_9,
            0x41 => KeyCode::VK_A,
            0x42 => KeyCode::VK_B,
            0x43 => KeyCode::VK_C,
            0x44 => KeyCode::VK_D,
            0x45 => KeyCode::VK_E,
            0x46 => KeyCode::VK_F,
            0x47 => KeyCode::VK_G,
            0x48 => KeyCode::VK_H,
            0x49 => KeyCode::VK_I,
            0x4A => KeyCode::VK_J,
            0x4B => KeyCode::VK_K,
            0x4C => KeyCode::VK_L,
            0x4D => KeyCode::VK_M,
            0x4E => KeyCode::VK_N,
            0x4F => KeyCode::VK_O,
            0x50 => KeyCode::VK_P,
            0x51 => KeyCode::VK_Q,
            0x52 => KeyCode::VK_R,
            0x53 => KeyCode::VK_S,
            0x54 => KeyCode::VK_T,
            0x55 => KeyCode::VK_U,
            0x56 => KeyCode::VK_V,
            0x57 => KeyCode::VK_W,
            0x58 => KeyCode::VK_X,
            0x59 => KeyCode::VK_Y,
            0x5A => KeyCode::VK_Z,
            0x5B => KeyCode::VK_LWIN,
            0x5C => KeyCode::VK_RWIN,
            0x5D => KeyCode::VK_APPS,
            0x5F => KeyCode::VK_SLEEP,
            0x60 => KeyCode::VK_NUMPAD0,
            0x61 => KeyCode::VK_NUMPAD1,
            0x62 => KeyCode::VK_NUMPAD2,
            0x63 => KeyCode::VK_NUMPAD3,
            0x64 => KeyCode::VK_NUMPAD4,
            0x65 => KeyCode::VK_NUMPAD5,
            0x66 => KeyCode::VK_NUMPAD6,
            0x67 => KeyCode::VK_NUMPAD7,
            0x68 => KeyCode::VK_NUMPAD8,
            0x69 => KeyCode::VK_NUMPAD9,
            0x6A => KeyCode::VK_MULTIPLY,
            0x6B => KeyCode::VK_ADD,
            0x6C => KeyCode::VK_SEPARATOR,
            0x6D => KeyCode::VK_SUBTRACT,
            0x6E => KeyCode::VK_DECIMAL,
            0x6F => KeyCode::VK_DIVIDE,
            0x70 => KeyCode::VK_F1,
            0x71 => KeyCode::VK_F2,
            0x72 => KeyCode::VK_F3,
            0x73 => KeyCode::VK_F4,
            0x74 => KeyCode::VK_F5,
            0x75 => KeyCode::VK_F6,
            0x76 => KeyCode::VK_F7,
            0x77 => KeyCode::VK_F8,
            0x78 => KeyCode::VK_F9,
            0x79 => KeyCode::VK_F10,
            0x7A => KeyCode::VK_F11,
            0x7B => KeyCode::VK_F12,
            0x7C => KeyCode::VK_F13,
            0x7D => KeyCode::VK_F14,
            0x7E => KeyCode::VK_F15,
            0x7F => KeyCode::VK_F16,
            0x80 => KeyCode::VK_F17,
            0x81 => KeyCode::VK_F18,
            0x82 => KeyCode::VK_F19,
            0x83 => KeyCode::VK_F20,
            0x84 => KeyCode::VK_F21,
            0x85 => KeyCode::VK_F22,
            0x86 => KeyCode::VK_F23,
            0x87 => KeyCode::VK_F24,
            0x90 => KeyCode::VK_NUMLOCK,
            0x91 => KeyCode::VK_SCROLL,
            0xA0 => KeyCode::VK_LSHIFT,
            0xA1 => KeyCode::VK_RSHIFT,
            0xA2 => KeyCode::VK_LCONTROL,
            0xA3 => KeyCode::VK_RCONTROL,
            0xA4 => KeyCode::VK_LMENU,
            0xA5 => KeyCode::VK_RMENU,
            0xA6 => KeyCode::VK_BROWSER_BACK,
            0xA7 => KeyCode::VK_BROWSER_FORWARD,
            0xA8 => KeyCode::VK_BROWSER_REFRESH,
            0xA9 => KeyCode::VK_BROWSER_STOP,
            0xAA => KeyCode::VK_BROWSER_SEARCH,
            0xAB => KeyCode::VK_BROWSER_FAVORITES,
            0xAC => KeyCode::VK_BROWSER_HOME,
            0xAD => KeyCode::VK_VOLUME_MUTE,
            0xAE => KeyCode::VK_VOLUME_DOWN,
            0xAF => KeyCode::VK_VOLUME_UP,
            0xB0 => KeyCode::VK_MEDIA_NEXT_TRACK,
            0xB1 => KeyCode::VK_MEDIA_PREV_TRACK,
            0xB2 => KeyCode::VK_MEDIA_STOP,
            0xB3 => KeyCode::VK_MEDIA_PLAY_PAUSE,
            0xB4 => KeyCode::VK_LAUNCH_MAIL,
            0xB => KeyCode::VK_LAUNCH_MEDIA_SELECT,
            0xB6 => KeyCode::VK_LAUNCH_APP1,
            0xB7 => KeyCode::VK_LAUNCH_APP2,
            0xBA => KeyCode::VK_OEM_1,
            0xBB => KeyCode::VK_OEM_PLUS,
            0xBC => KeyCode::VK_OEM_COMMA,
            0xBD => KeyCode::VK_OEM_MINUS,
            0xBE => KeyCode::VK_OEM_PERIOD,
            0xBF => KeyCode::VK_OEM_2,
            0xC0 => KeyCode::VK_OEM_3,
            0xDB => KeyCode::VK_OEM_4,
            0xDC => KeyCode::VK_OEM_5,
            0xDD => KeyCode::VK_OEM_6,
            0xDE => KeyCode::VK_OEM_7,
            0xDF => KeyCode::VK_OEM_8,
            0xE2 => KeyCode::VK_OEM_102,
            0xE5 => KeyCode::VK_PROCESSKEY,
            0xE7 => KeyCode::VK_PACKET,
            0xF6 => KeyCode::VK_ATTN,
            0xF7 => KeyCode::VK_CRSEL,
            0xF8 => KeyCode::VK_EXSEL,
            0xF9 => KeyCode::VK_EREOF,
            0xFA => KeyCode::VK_PLAY,
            0xFB => KeyCode::VK_ZOOM,
            0xFC => KeyCode::VK_NONAME,
            0xFD => KeyCode::VK_PA1,
            0xFE => KeyCode::VK_OEM_CLEAR,
            _ => KeyCode::VK_UNKNOWN,
        }
    }
}

impl From<KeyCode> for i32 {
    fn from(val: KeyCode) -> Self {
        val as i32
    }
}

#[pymethods]
impl KeyCode {
    #[getter]
    fn value(&self) -> i32 {
        (*self).into()
    }
}

#[pyclass]
#[derive(Clone, Debug)]
#[pyo3(name = "EntityPool")]
pub struct VcmpEntityPoolPy {
    pub inner: VcmpEntityPool,
}

impl From<VcmpEntityPoolPy> for VcmpEntityPool {
    fn from(value: VcmpEntityPoolPy) -> Self {
        value.inner
    }
}

#[pymethods]
impl VcmpEntityPoolPy {
    #[staticmethod]
    pub fn vehicle() -> Self {
        Self {
            inner: VcmpEntityPool::Vehicle,
        }
    }

    #[staticmethod]
    pub fn object() -> Self {
        Self {
            inner: VcmpEntityPool::Object,
        }
    }

    #[staticmethod]
    pub fn pickup() -> Self {
        Self {
            inner: VcmpEntityPool::Pickup,
        }
    }

    #[staticmethod]
    pub fn radio() -> Self {
        Self {
            inner: VcmpEntityPool::Radio,
        }
    }

    #[staticmethod]
    pub fn player() -> Self {
        Self {
            inner: VcmpEntityPool::Player,
        }
    }

    #[staticmethod]
    pub fn reserved1() -> Self {
        Self {
            inner: VcmpEntityPool::Reserved1,
        }
    }

    #[staticmethod]
    pub fn marker() -> Self {
        Self {
            inner: VcmpEntityPool::Marker,
        }
    }

    #[staticmethod]
    pub fn checkpoint() -> Self {
        Self {
            inner: VcmpEntityPool::CheckPoint,
        }
    }
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ServerSettingsPy>()?;
    m.add_class::<RGBPy>()?;
    m.add_class::<VectorPy>()?;
    m.add_class::<QuaternionPy>()?;
    m.add_class::<Version>()?;
    m.add_class::<KeyCode>()?;
    Ok(())
}
