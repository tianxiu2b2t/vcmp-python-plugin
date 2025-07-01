use std::ops::Add;

use pyo3::{
    Bound, PyResult, Python, pyclass, pymethods,
    types::{PyModule, PyModuleMethods},
};

use vcmp_bindings::utils::Color;
use vcmp_bindings::{
    func::{
        CheckPointMethods, MarkerMethods, ObjectMethods, PickupMethods, PlayerMethods,
        QueryVehicle, SetVehicle,
    },
    setting::VcmpServerSettings,
    utils::Vectorf32,
    vcmp_func,
};

use crate::consts::EntityId;

#[derive(Clone)]
#[pyclass]
#[pyo3(name = "ServerSettings")]
pub struct ServerSettingsPy {
    pub inner: VcmpServerSettings,
}

impl ServerSettingsPy {
    pub fn from(value: VcmpServerSettings) -> Self {
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

#[derive(Clone)]
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

    fn __repr__(&self) -> String {
        let (r, b, g, a) = (self.inner.r, self.inner.b, self.inner.g, self.inner.a);
        format!("RGB(r={r}, b={b}, g={g}, a={a})")
    }
}

#[derive(Clone)]
pub enum EntityVectorType {
    PlayerPosition = 0,
    PlayerSpeed = 1,
    VehiclePosition = 2,
    VehicleSpeed = 3,
    VehicleRelSpeed = 4,
    VehicleRelTurnSpeed = 5,
    ObjectPosition = 6,
    PickupPosition = 7,
    CheckPointPosition = 8,
    MarkerPosition = 9,
    Ignore = -1,
}

#[derive(Clone)]
#[pyclass]
#[pyo3(name = "EntityVector")]
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
            inner: Some(val)
        }
    }
}

impl VectorPy {
    pub fn get_entity_pos(&self) -> Vectorf32 {
        match self.entity_type {
            EntityVectorType::PlayerPosition => {
                let res = vcmp_func().get_player_position(self.entity_id);
                if let Ok(res) = res {
                    res
                } else {
                    Vectorf32::default()
                }
            }
            EntityVectorType::PlayerSpeed => {
                let res = vcmp_func().get_player_speed(self.entity_id);
                if let Ok(res) = res {
                    res
                } else {
                    Vectorf32::default()
                }
            }
            EntityVectorType::VehiclePosition => vcmp_func().get_vehicle_position(self.entity_id),
            EntityVectorType::VehicleSpeed => vcmp_func().get_vehicle_speed(self.entity_id),
            EntityVectorType::VehicleRelSpeed => vcmp_func().get_vehicle_rel_speed(self.entity_id),
            EntityVectorType::VehicleRelTurnSpeed => {
                vcmp_func().get_vehicle_rel_turn_speed(self.entity_id)
            }
            EntityVectorType::ObjectPosition => {
                let res = vcmp_func().get_object_position(self.entity_id);
                if let Ok(res) = res {
                    res
                } else {
                    Vectorf32::default()
                }
            }
            EntityVectorType::PickupPosition => {
                let res = vcmp_func().get_pickup_position(self.entity_id);
                if let Ok(res) = res {
                    res
                } else {
                    Vectorf32::default()
                }
            }
            EntityVectorType::CheckPointPosition => {
                let res = vcmp_func().get_check_point_position(self.entity_id);
                if let Ok(res) = res {
                    res
                } else {
                    Vectorf32::default()
                }
            }
            EntityVectorType::MarkerPosition => {
                let info = vcmp_func().get_marker_info(self.entity_id);
                info.position
            }
            EntityVectorType::Ignore => self.inner.unwrap_or_default(),
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
                let _ = vcmp_func().set_check_point_position(self.entity_id, origin);
            }
            EntityVectorType::MarkerPosition => {
                // ignore
            }
            EntityVectorType::Ignore => {
                self.inner = Some(origin);
                // ignore
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

impl Default for VectorPy {
    fn default() -> Self {
        Self {
            inner: None,
            entity_type: EntityVectorType::Ignore,
            entity_id: 0,
        }
    }
}

#[pymethods]
impl VectorPy {
    #[new]
    fn new(x: Option<f32>, y: Option<f32>, z: Option<f32>) -> Self {
        Self {
            inner: Some(Vectorf32::new(x.unwrap_or(0.0), y.unwrap_or(0.0), z.unwrap_or(0.0))),
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
    pub fn set_z(&mut self, value: f32) {
        self.set_entity_pos(None, None, Some(value));
    }

    #[setter]
    pub fn set_y(&mut self, value: f32) {
        self.set_entity_pos(None, Some(value), None);
    }

    #[setter]
    pub fn set_x(&mut self, value: f32) {
        self.set_entity_pos(Some(value), None, None);
    }

    fn __repr__(&self) -> String {
        let pos = self.get_entity_pos();
        format!("Vector(x={}, y={}, z={})", pos.x, pos.y, pos.z)
    }
}

pub fn module_define(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ServerSettingsPy>()?;
    m.add_class::<RGBPy>()?;
    m.add_class::<VectorPy>()?;
    Ok(())
}
