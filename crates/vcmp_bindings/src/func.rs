use crate::options::{VcmpNetworkStatisticsQueryOption, VcmpServerOption, VcmpVehicleOption};
use crate::{VcmpError, raw::PluginFuncs, VcmpResult};

use crate::encodes::{encode_to_gbk};

pub mod environment;
pub mod keybind;
pub mod marker;
pub mod misc;
pub mod network;
pub mod player;
pub mod plugin;
pub mod server;
pub mod vehicle;
pub mod weapon;
pub mod admin;

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
        let msg = encode_to_gbk(message);
        let msg_ptr = msg.as_ptr() as *const i8;
        let _ = (self.inner.LogMessage)(msg_ptr);
    }

    /// 载具相关的选项
    #[inline]
    pub fn set_vehicle_option(&self, vehicle_id: i32, option: VcmpVehicleOption, toggle: bool) -> VcmpResult<()> {
        let code = (self.inner.SetVehicleOption)(vehicle_id, option.into(), toggle as u8);
        if code != 0 {
            Err(VcmpError::from(code))
        } else {
            Ok(())
        }
    }
    /// 获取车辆选项
    #[inline]
    pub fn get_vehicle_option(&self, vehicle_id: i32, option: VcmpVehicleOption) -> bool {
        (self.inner.GetVehicleOption)(vehicle_id, option.into()) != 0
    }

    /// 获取网络统计信息
    #[inline]
    pub fn get_network_statistics(
        &self,
        player_id: i32,
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

    /*
         * 拾取物相关
         *

        /// 创建拾取物
        pub fn create_pickup(
            &self,
            model_index: i32,
            world: i32,
            quantity: i32,
            x: f32,
            y: f32,
            z: f32,
            alpha: i32,
            is_automatic: bool,
        ) -> i32 {
            (self.inner.CreatePickup)(
                model_index,
                world,
                quantity,
                x,
                y,
                z,
                alpha,
                is_automatic as u8,
            )
        }

        /// 删除拾取物
        pub fn delete_pickup(&self, pickup_id: i32) -> VcmpError {
            let code = (self.inner.DeletePickup)(pickup_id);
            VcmpError::from(code)
        }

        /// 检查拾取物是否对玩家可见
        pub fn is_pickup_streamed_for_player(&self, pickup_id: i32, player_id: i32) -> bool {
            (self.inner.IsPickupStreamedForPlayer)(pickup_id, player_id) != 0
        }

        /// 设置拾取物世界
        pub fn set_pickup_world(&self, pickup_id: i32, world: i32) -> VcmpError {
            let code = (self.inner.SetPickupWorld)(pickup_id, world);
            VcmpError::from(code)
        }

        /// 获取拾取物世界
        pub fn get_pickup_world(&self, pickup_id: i32) -> i32 {
            (self.inner.GetPickupWorld)(pickup_id)
        }

        /// 设置拾取物透明度
        pub fn set_pickup_alpha(&self, pickup_id: i32, alpha: i32) -> VcmpError {
            let code = (self.inner.SetPickupAlpha)(pickup_id, alpha);
            VcmpError::from(code)
        }

        /// 获取拾取物透明度
        pub fn get_pickup_alpha(&self, pickup_id: i32) -> i32 {
            (self.inner.GetPickupAlpha)(pickup_id)
        }

        /// 设置拾取物是否自动刷新
        pub fn set_pickup_is_automatic(&self, pickup_id: i32, toggle: bool) -> VcmpError {
            let code = (self.inner.SetPickupIsAutomatic)(pickup_id, toggle as u8);
            VcmpError::from(code)
        }

        /// 检查拾取物是否自动刷新
        pub fn is_pickup_automatic(&self, pickup_id: i32) -> bool {
            (self.inner.IsPickupAutomatic)(pickup_id) != 0
        }

        /// 设置拾取物自动刷新计时器
        pub fn set_pickup_auto_timer(&self, pickup_id: i32, duration_millis: u32) -> VcmpError {
            let code = (self.inner.SetPickupAutoTimer)(pickup_id, duration_millis);
            VcmpError::from(code)
        }

        /// 获取拾取物自动刷新计时器
        pub fn get_pickup_auto_timer(&self, pickup_id: i32) -> u32 {
            (self.inner.GetPickupAutoTimer)(pickup_id)
        }

        /// 刷新拾取物
        pub fn refresh_pickup(&self, pickup_id: i32) -> VcmpError {
            let code = (self.inner.RefreshPickup)(pickup_id);
            VcmpError::from(code)
        }

        /// 设置拾取物位置
        pub fn set_pickup_position(&self, pickup_id: i32, x: f32, y: f32, z: f32) -> VcmpError {
            let code = (self.inner.SetPickupPosition)(pickup_id, x, y, z);
            VcmpError::from(code)
        }

        /// 获取拾取物位置
        pub fn get_pickup_position(&self, pickup_id: i32) -> Result<(f32, f32, f32), VcmpError> {
            let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
            let code = (self.inner.GetPickupPosition)(pickup_id, &mut x, &mut y, &mut z);

            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok((x, y, z))
            }
        }

        /// 获取拾取物模型
        pub fn get_pickup_model(&self, pickup_id: i32) -> i32 {
            (self.inner.GetPickupModel)(pickup_id)
        }

        /// 获取拾取物数量
        pub fn get_pickup_quantity(&self, pickup_id: i32) -> i32 {
            (self.inner.GetPickupQuantity)(pickup_id)
        }

        /*
         * 检查点相关
         */

        /// 创建检查点
        pub fn create_check_point(
            &self,
            player_id: i32,
            world: i32,
            is_sphere: bool,
            x: f32,
            y: f32,
            z: f32,
            red: i32,
            green: i32,
            blue: i32,
            alpha: i32,
            radius: f32,
        ) -> i32 {
            (self.inner.CreateCheckPoint)(
                player_id,
                world,
                is_sphere as u8,
                x,
                y,
                z,
                red,
                green,
                blue,
                alpha,
                radius,
            )
        }

        /// 删除检查点
        pub fn delete_check_point(&self, check_point_id: i32) -> VcmpError {
            let code = (self.inner.DeleteCheckPoint)(check_point_id);
            VcmpError::from(code)
        }

        /// 检查检查点是否对玩家可见
        pub fn is_check_point_streamed_for_player(&self, check_point_id: i32, player_id: i32) -> bool {
            (self.inner.IsCheckPointStreamedForPlayer)(check_point_id, player_id) != 0
        }

        /// 检查检查点是否为球形
        pub fn is_check_point_sphere(&self, check_point_id: i32) -> bool {
            (self.inner.IsCheckPointSphere)(check_point_id) != 0
        }

        /// 设置检查点世界
        pub fn set_check_point_world(&self, check_point_id: i32, world: i32) -> VcmpError {
            let code = (self.inner.SetCheckPointWorld)(check_point_id, world);
            VcmpError::from(code)
        }

        /// 获取检查点世界
        pub fn get_check_point_world(&self, check_point_id: i32) -> i32 {
            (self.inner.GetCheckPointWorld)(check_point_id)
        }

        /// 设置检查点颜色
        pub fn set_check_point_colour(
            &self,
            check_point_id: i32,
            red: i32,
            green: i32,
            blue: i32,
            alpha: i32,
        ) -> VcmpError {
            let code = (self.inner.SetCheckPointColour)(check_point_id, red, green, blue, alpha);
            VcmpError::from(code)
        }

        /// 获取检查点颜色
        pub fn get_check_point_colour(
            &self,
            check_point_id: i32,
        ) -> Result<(i32, i32, i32, i32), VcmpError> {
            let (mut r, mut g, mut b, mut a) = (0, 0, 0, 0);
            let code = (self.inner.GetCheckPointColour)(check_point_id, &mut r, &mut g, &mut b, &mut a);

            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok((r, g, b, a))
            }
        }

        /// 设置检查点位置
        pub fn set_check_point_position(
            &self,
            check_point_id: i32,
            x: f32,
            y: f32,
            z: f32,
        ) -> VcmpError {
            let code = (self.inner.SetCheckPointPosition)(check_point_id, x, y, z);
            VcmpError::from(code)
        }

        /// 获取检查点位置
        pub fn get_check_point_position(
            &self,
            check_point_id: i32,
        ) -> Result<(f32, f32, f32), VcmpError> {
            let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
            let code = (self.inner.GetCheckPointPosition)(check_point_id, &mut x, &mut y, &mut z);

            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok((x, y, z))
            }
        }

        /// 设置检查点半径
        pub fn set_check_point_radius(&self, check_point_id: i32, radius: f32) -> VcmpError {
            let code = (self.inner.SetCheckPointRadius)(check_point_id, radius);
            VcmpError::from(code)
        }

        /// 获取检查点半径
        pub fn get_check_point_radius(&self, check_point_id: i32) -> f32 {
            (self.inner.GetCheckPointRadius)(check_point_id)
        }

        /// 获取检查点所有者
        pub fn get_check_point_owner(&self, check_point_id: i32) -> i32 {
            (self.inner.GetCheckPointOwner)(check_point_id)
        }

        /*
         * 对象相关
         */

        /// 创建对象
        pub fn create_object(
            &self,
            model_index: i32,
            world: i32,
            x: f32,
            y: f32,
            z: f32,
            alpha: i32,
        ) -> i32 {
            (self.inner.CreateObject)(model_index, world, x, y, z, alpha)
        }

        /// 删除对象
        pub fn delete_object(&self, object_id: i32) -> VcmpError {
            let code = (self.inner.DeleteObject)(object_id);
            VcmpError::from(code)
        }

        /// 检查对象是否对玩家可见
        pub fn is_object_streamed_for_player(&self, object_id: i32, player_id: i32) -> bool {
            (self.inner.IsObjectStreamedForPlayer)(object_id, player_id) != 0
        }

        /// 获取对象模型
        pub fn get_object_model(&self, object_id: i32) -> i32 {
            (self.inner.GetObjectModel)(object_id)
        }

        /// 设置对象世界
        pub fn set_object_world(&self, object_id: i32, world: i32) -> VcmpError {
            let code = (self.inner.SetObjectWorld)(object_id, world);
            VcmpError::from(code)
        }

        /// 获取对象世界
        pub fn get_object_world(&self, object_id: i32) -> i32 {
            (self.inner.GetObjectWorld)(object_id)
        }

        /// 设置对象透明度
        pub fn set_object_alpha(&self, object_id: i32, alpha: i32, duration: u32) -> VcmpError {
            let code = (self.inner.SetObjectAlpha)(object_id, alpha, duration);
            VcmpError::from(code)
        }

        /// 获取对象透明度
        pub fn get_object_alpha(&self, object_id: i32) -> i32 {
            (self.inner.GetObjectAlpha)(object_id)
        }

        /// 将对象移动到指定位置
        pub fn move_object_to(
            &self,
            object_id: i32,
            x: f32,
            y: f32,
            z: f32,
            duration: u32,
        ) -> VcmpError {
            let code = (self.inner.MoveObjectTo)(object_id, x, y, z, duration);
            VcmpError::from(code)
        }
        pub fn move_object_by(
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

        /// 设置对象位置
        pub fn set_object_position(&self, object_id: i32, x: f32, y: f32, z: f32) -> VcmpResult<()> {
            let code = (self.inner.SetObjectPosition)(object_id, x, y, z);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取对象位置
        pub fn get_object_position(&self, object_id: i32) -> Result<(f32, f32, f32), VcmpError> {
            let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
            let code = (self.inner.GetObjectPosition)(object_id, &mut x, &mut y, &mut z);

            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok((x, y, z))
            }
        }

        /// 将对象旋转到指定方向
        pub fn rotate_object_to(
            &self,
            object_id: i32,
            x: f32,
            y: f32,
            z: f32,
            w: f32,
            duration: u32,
        ) -> VcmpResult<()> {
            let code = (self.inner.RotateObjectTo)(object_id, x, y, z, w, duration);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 将对象旋转到指定欧拉角
        pub fn rotate_object_to_euler(
            &self,
            object_id: i32,
            x: f32,
            y: f32,
            z: f32,
            duration: u32,
        ) -> VcmpResult<()> {
            let code = (self.inner.RotateObjectToEuler)(object_id, x, y, z, duration);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 将对象按指定方式旋转
        pub fn rotate_object_by(
            &self,
            object_id: i32,
            x: f32,
            y: f32,
            z: f32,
            w: f32,
            duration: u32,
        ) -> VcmpResult<()> {
            let code = (self.inner.RotateObjectBy)(object_id, x, y, z, w, duration);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 将对象按指定欧拉角旋转
        pub fn rotate_object_by_euler(
            &self,
            object_id: i32,
            x: f32,
            y: f32,
            z: f32,
            duration: u32,
        ) -> VcmpResult<()> {
            let code = (self.inner.RotateObjectByEuler)(object_id, x, y, z, duration);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取对象旋转
        pub fn get_object_rotation(&self, object_id: i32) -> Result<(f32, f32, f32, f32), VcmpError> {
            let (mut x, mut y, mut z, mut w) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
            let code = (self.inner.GetObjectRotation)(object_id, &mut x, &mut y, &mut z, &mut w);

            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok((x, y, z, w))
            }
        }

        /// 获取对象欧拉旋转
        pub fn get_object_rotation_euler(&self, object_id: i32) -> Result<(f32, f32, f32), VcmpError> {
            let (mut x, mut y, mut z) = (0.0f32, 0.0f32, 0.0f32);
            let code = (self.inner.GetObjectRotationEuler)(object_id, &mut x, &mut y, &mut z);

            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok((x, y, z))
            }
        }

        /// 设置对象被射击报告
        pub fn set_object_shot_report_enabled(&self, object_id: i32, toggle: bool) -> VcmpResult<()> {
            let code = (self.inner.SetObjectShotReportEnabled)(object_id, toggle as u8);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 检查对象被射击报告是否启用
        pub fn is_object_shot_report_enabled(&self, object_id: i32) -> bool {
            (self.inner.IsObjectShotReportEnabled)(object_id) != 0
        }

        /// 设置对象被触摸报告
        pub fn set_object_touched_report_enabled(
            &self,
            object_id: i32,
            toggle: bool,
        ) -> VcmpResult<()> {
            let code = (self.inner.SetObjectTouchedReportEnabled)(object_id, toggle as u8);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 检查对象被触摸报告是否启用
        pub fn is_object_touched_report_enabled(&self, object_id: i32) -> bool {
            (self.inner.IsObjectTouchedReportEnabled)(object_id) != 0
        }

        /*
         * 其他功能
         */

        /// 设置跌落计时器
        pub fn set_fall_timer(&self, time_rate: u16) {
            (self.inner.SetFallTimer)(time_rate);
        }

        /// 获取跌落计时器
        pub fn get_fall_timer(&self) -> u16 {
            (self.inner.GetFallTimer)()
        }

        /// 设置车辆灯光数据
        pub fn set_vehicle_lights_data(&self, vehicle_id: i32, lights_data: u32) -> VcmpResult<()> {
            let code = (self.inner.SetVehicleLightsData)(vehicle_id, lights_data);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取车辆灯光数据
        pub fn get_vehicle_lights_data(&self, vehicle_id: i32) -> u32 {
            (self.inner.GetVehicleLightsData)(vehicle_id)
        }

        /// 杀死玩家
        pub fn kill_player(&self, player_id: i32) -> VcmpResult<()> {
            let code = (self.inner.KillPlayer)(player_id);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 为玩家设置车辆 3D 箭头
        pub fn set_vehicle_3d_arrow_for_player(
            &self,
            vehicle_id: i32,
            target_player_id: i32,
            is_enabled: bool,
        ) -> VcmpResult<()> {
            let code =
                (self.inner.SetVehicle3DArrowForPlayer)(vehicle_id, target_player_id, is_enabled as u8);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家是否为车辆设置 3D 箭头
        pub fn get_vehicle_3d_arrow_for_player(&self, vehicle_id: i32, target_player_id: i32) -> bool {
            (self.inner.GetVehicle3DArrowForPlayer)(vehicle_id, target_player_id) != 0
        }

        /// 为玩家设置玩家 3D 箭头
        pub fn set_player_3d_arrow_for_player(
            &self,
            player_id: i32,
            target_player_id: i32,
            is_enabled: bool,
        ) -> VcmpResult<()> {
            let code =
                (self.inner.SetPlayer3DArrowForPlayer)(player_id, target_player_id, is_enabled as u8);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家是否为玩家设置 3D 箭头
        pub fn get_player_3d_arrow_for_player(&self, player_id: i32, target_player_id: i32) -> bool {
            (self.inner.GetPlayer3DArrowForPlayer)(player_id, target_player_id) != 0
        }

        /// 设置玩家醉酒处理
        pub fn set_player_drunk_handling(&self, player_id: i32, drunk_level: u32) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerDrunkHandling)(player_id, drunk_level);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家醉酒处理级别
        pub fn get_player_drunk_handling(&self, player_id: i32) -> u32 {
            (self.inner.GetPlayerDrunkHandling)(player_id)
        }

        /// 设置玩家醉酒视觉效果
        pub fn set_player_drunk_visuals(&self, player_id: i32, drunk_level: u8) -> VcmpResult<()> {
            let code = (self.inner.SetPlayerDrunkVisuals)(player_id, drunk_level);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }

        /// 获取玩家醉酒视觉效果级别
        pub fn get_player_drunk_visuals(&self, player_id: i32) -> u8 {
            (self.inner.GetPlayerDrunkVisuals)(player_id)
        }

        /// 插值摄像机观察目标
        pub fn interpolate_camera_look_at(
            &self,
            player_id: i32,
            look_x: f32,
            look_y: f32,
            look_z: f32,
            interp_time_ms: u32,
        ) -> VcmpResult<()> {
            let code =
                (self.inner.InterpolateCameraLookAt)(player_id, look_x, look_y, look_z, interp_time_ms);
            if code != 0 {
                Err(VcmpError::from(code))
            } else {
                Ok(())
            }
        }
    */

}
