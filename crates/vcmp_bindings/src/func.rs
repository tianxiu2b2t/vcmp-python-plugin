use crate::options::{VcmpNetworkStatisticsQueryOption, VcmpServerOption, VcmpVehicleOption};
use crate::{PlayerId, VcmpError, VcmpResult, VehicleId, raw::PluginFuncs};

use crate::encodes::encode_to_gbk;

pub mod admin;
pub mod checkpoint;
pub mod environment;
pub mod keybind;
pub mod marker;
pub mod misc;
pub mod network;
pub mod object;
pub mod pickup;
pub mod player;
pub mod plugin;
pub mod server;
pub mod vehicle;
pub mod weapon;

// use 所有的 trait
pub use admin::AdministrationMethods;
pub use checkpoint::CheckPointMethods;
pub use environment::{EnvironmentMethods, QueryEnvironmentOption, SetEnvironmentOption};
pub use keybind::KeybindMethods;
pub use marker::MarkerMethods;
pub use misc::MiscMethods;
pub use network::QueryNetworkStatistics;
pub use object::ObjectMethods;
pub use pickup::PickupMethods;
pub use player::PlayerMethods;
pub use plugin::PluginMethods;
pub use server::ServerMethods;
pub use vehicle::{
    QueryVehicle, QueryVehicleOptions, SetVehicle, SetVehicleOptions, VehicleHandlingMethods,
    VehicleMethods,
};
pub use weapon::WeaponMethods;

pub struct VcmpFunctions {
    inner: PluginFuncs,
}

unsafe impl Sync for PluginFuncs {}

unsafe impl Send for PluginFuncs {}

impl From<PluginFuncs> for VcmpFunctions {
    fn from(value: PluginFuncs) -> Self {
        Self { inner: value }
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
impl From<*mut PluginFuncs> for VcmpFunctions {
    fn from(value: *mut PluginFuncs) -> Self {
        let inner = unsafe { *value };
        Self { inner }
    }
}

impl VcmpFunctions {
    /// 获取 vcmp 服务器给过来的 struct 的大小
    pub fn inner_struct_size(&self) -> usize {
        self.inner.structSize as usize
    }

    /// 获取 bindgen 生成的结构体的大小
    pub fn inner_ffi_size(&self) -> usize {
        std::mem::size_of::<PluginFuncs>()
    }

    /// 全局性的 last error
    pub fn get_last_error(&self) -> VcmpError {
        VcmpError::from((self.inner.GetLastError)())
    }

    pub fn log_message(&self, message: &str) {
        let zero_msg = format!("{message}\0");
        let msg = encode_to_gbk(zero_msg.as_str());
        let msg_ptr = msg.as_ptr() as *const i8;
        let _ = (self.inner.LogMessage)(msg_ptr);
    }

    /// 载具相关的选项
    #[inline]
    pub fn set_vehicle_option(
        &self,
        vehicle_id: VehicleId,
        option: VcmpVehicleOption,
        toggle: bool,
    ) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleOption)(vehicle_id, option.into(), toggle as u8);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    /// 获取车辆选项
    #[inline]
    pub fn get_vehicle_option(&self, vehicle_id: VehicleId, option: VcmpVehicleOption) -> bool {
        (self.inner.GetVehicleOption)(vehicle_id, option.into()) != 0
    }

    /// 获取网络统计信息
    #[inline]
    pub fn get_network_statistics(
        &self,
        player_id: PlayerId,
        option: VcmpNetworkStatisticsQueryOption,
    ) -> f64 {
        (self.inner.GetNetworkStatistics)(player_id, option.into())
    }

    /// 获取指定的服务器设置
    ///
    /// 需要啥直接取
    #[inline]
    pub fn get_server_option(&self, option: VcmpServerOption) -> bool {
        (self.inner.GetServerOption)(option.into()) != 0
    }

    /// 设置指定的服务器设置
    #[inline]
    pub fn set_server_option(&self, option: VcmpServerOption, toggle: bool) {
        let _ = (self.inner.SetServerOption)(option.into(), toggle as u8);
    }
}
