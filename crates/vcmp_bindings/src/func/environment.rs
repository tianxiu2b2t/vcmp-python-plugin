use crate::utils::{Color, WastedSettings, WorldBounds};
use crate::{VcmpFunctions, options::VcmpServerOption};

pub trait QueryEnvironmentOption {
    fn get_sync_frame_limiter(&self) -> bool;
    fn get_frame_limiter(&self) -> bool;
    fn get_taxi_boost_jump(&self) -> bool;
    fn get_drive_on_water(&self) -> bool;
    fn get_fast_switch(&self) -> bool;
    fn get_friendly_fire(&self) -> bool;
    fn get_disable_drive_by(&self) -> bool;
    fn get_perfect_handling(&self) -> bool;
    fn get_flying_cars(&self) -> bool;
    fn get_jump_switch(&self) -> bool;
    fn get_show_markers(&self) -> bool;
    fn get_only_show_team_markers(&self) -> bool;
    fn get_stunt_bike(&self) -> bool;
    fn get_shoot_in_air(&self) -> bool;
    fn get_show_name_tags(&self) -> bool;
    fn get_join_messages(&self) -> bool;
    fn get_death_messages(&self) -> bool;
    fn get_chat_tags_enabled(&self) -> bool;
    fn get_use_classes(&self) -> bool;
    fn get_wall_glitch(&self) -> bool;
    fn get_disable_backface_culling(&self) -> bool;
    fn get_disable_heli_blade_damage(&self) -> bool;
    fn get_disable_crouch(&self) -> bool;
}
impl QueryEnvironmentOption for VcmpFunctions {
    fn get_sync_frame_limiter(&self) -> bool {
        self.get_server_option(VcmpServerOption::SyncFrameLimiter)
    }
    fn get_frame_limiter(&self) -> bool {
        self.get_server_option(VcmpServerOption::FrameLimiter)
    }
    fn get_taxi_boost_jump(&self) -> bool {
        self.get_server_option(VcmpServerOption::TaxiBoostJump)
    }
    fn get_drive_on_water(&self) -> bool {
        self.get_server_option(VcmpServerOption::DriveOnWater)
    }
    fn get_fast_switch(&self) -> bool {
        self.get_server_option(VcmpServerOption::FastSwitch)
    }
    fn get_friendly_fire(&self) -> bool {
        self.get_server_option(VcmpServerOption::FriendlyFire)
    }
    fn get_disable_drive_by(&self) -> bool {
        self.get_server_option(VcmpServerOption::DisableDriveBy)
    }
    fn get_perfect_handling(&self) -> bool {
        self.get_server_option(VcmpServerOption::PerfectHandling)
    }
    fn get_flying_cars(&self) -> bool {
        self.get_server_option(VcmpServerOption::FlyingCars)
    }
    fn get_jump_switch(&self) -> bool {
        self.get_server_option(VcmpServerOption::JumpSwitch)
    }
    fn get_show_markers(&self) -> bool {
        self.get_server_option(VcmpServerOption::ShowMarkers)
    }
    fn get_only_show_team_markers(&self) -> bool {
        self.get_server_option(VcmpServerOption::OnlyShowTeamMarkers)
    }
    fn get_stunt_bike(&self) -> bool {
        self.get_server_option(VcmpServerOption::StuntBike)
    }
    fn get_shoot_in_air(&self) -> bool {
        self.get_server_option(VcmpServerOption::ShootInAir)
    }
    fn get_show_name_tags(&self) -> bool {
        self.get_server_option(VcmpServerOption::ShowNameTags)
    }
    fn get_join_messages(&self) -> bool {
        self.get_server_option(VcmpServerOption::JoinMessages)
    }
    fn get_death_messages(&self) -> bool {
        self.get_server_option(VcmpServerOption::DeathMessages)
    }
    fn get_chat_tags_enabled(&self) -> bool {
        self.get_server_option(VcmpServerOption::ChatTagsEnabled)
    }
    fn get_use_classes(&self) -> bool {
        self.get_server_option(VcmpServerOption::UseClasses)
    }
    fn get_wall_glitch(&self) -> bool {
        self.get_server_option(VcmpServerOption::WallGlitch)
    }
    fn get_disable_backface_culling(&self) -> bool {
        self.get_server_option(VcmpServerOption::DisableBackfaceCulling)
    }
    fn get_disable_heli_blade_damage(&self) -> bool {
        self.get_server_option(VcmpServerOption::DisableHeliBladeDamage)
    }
    fn get_disable_crouch(&self) -> bool {
        self.get_server_option(VcmpServerOption::DisableCrouch)
    }
}

pub trait SetEnvironmentOption {
    fn set_sync_frame_limiter(&self, toggle: bool);
    fn set_frame_limiter(&self, toggle: bool);
    fn set_taxi_boost_jump(&self, toggle: bool);
    fn set_drive_on_water(&self, toggle: bool);
    fn set_fast_switch(&self, toggle: bool);
    fn set_friendly_fire(&self, toggle: bool);
    fn set_disable_drive_by(&self, toggle: bool);
    fn set_perfect_handling(&self, toggle: bool);
    fn set_flying_cars(&self, toggle: bool);
    fn set_jump_switch(&self, toggle: bool);
    fn set_show_markers(&self, toggle: bool);
    fn set_only_show_team_markers(&self, toggle: bool);
    fn set_stunt_bike(&self, toggle: bool);
    fn set_shoot_in_air(&self, toggle: bool);
    fn set_show_name_tags(&self, toggle: bool);
    fn set_join_messages(&self, toggle: bool);
    fn set_death_messages(&self, toggle: bool);
    fn set_chat_tags_enabled(&self, toggle: bool);
    fn set_use_classes(&self, toggle: bool);
    fn set_wall_glitch(&self, toggle: bool);
    fn set_disable_backface_culling(&self, toggle: bool);
    fn set_disable_heli_blade_damage(&self, toggle: bool);
    fn set_disable_crouch(&self, toggle: bool);
}
impl SetEnvironmentOption for VcmpFunctions {
    fn set_sync_frame_limiter(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::SyncFrameLimiter, toggle)
    }
    fn set_frame_limiter(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::FrameLimiter, toggle)
    }
    fn set_taxi_boost_jump(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::TaxiBoostJump, toggle)
    }
    fn set_drive_on_water(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::DriveOnWater, toggle)
    }
    fn set_fast_switch(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::FastSwitch, toggle)
    }
    fn set_friendly_fire(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::FriendlyFire, toggle)
    }
    fn set_disable_drive_by(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::DisableDriveBy, toggle)
    }
    fn set_perfect_handling(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::PerfectHandling, toggle)
    }
    fn set_flying_cars(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::FlyingCars, toggle)
    }
    fn set_jump_switch(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::JumpSwitch, toggle)
    }
    fn set_show_markers(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::ShowMarkers, toggle)
    }
    fn set_only_show_team_markers(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::OnlyShowTeamMarkers, toggle)
    }
    fn set_stunt_bike(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::StuntBike, toggle)
    }
    fn set_shoot_in_air(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::ShootInAir, toggle)
    }
    fn set_show_name_tags(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::ShowNameTags, toggle)
    }
    fn set_join_messages(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::JoinMessages, toggle)
    }
    fn set_death_messages(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::DeathMessages, toggle)
    }
    fn set_chat_tags_enabled(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::ChatTagsEnabled, toggle)
    }
    fn set_use_classes(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::UseClasses, toggle)
    }
    fn set_wall_glitch(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::WallGlitch, toggle)
    }
    fn set_disable_backface_culling(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::DisableBackfaceCulling, toggle)
    }
    fn set_disable_heli_blade_damage(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::DisableHeliBladeDamage, toggle)
    }
    fn set_disable_crouch(&self, toggle: bool) {
        self.set_server_option(VcmpServerOption::DisableCrouch, toggle)
    }
}

/*
    World Bounds
*/
pub trait QueryEnvironmentWorldBounds {
    fn get_world_bounds(&self) -> WorldBounds;
}
impl QueryEnvironmentWorldBounds for VcmpFunctions {
    fn get_world_bounds(&self) -> WorldBounds {
        let (mut max_x, mut min_x, mut max_y, mut min_y) = (0f32, 0f32, 0f32, 0f32);
        (self.inner.GetWorldBounds)(&mut max_x, &mut min_x, &mut max_y, &mut min_y);
        WorldBounds {
            max_x,
            min_x,
            max_y,
            min_y,
        }
    }
}

pub trait SetEnvironmentWorldBounds {
    fn set_world_bounds(&self, bounds: WorldBounds);
}
impl SetEnvironmentWorldBounds for VcmpFunctions {
    fn set_world_bounds(&self, bounds: WorldBounds) {
        (self.inner.SetWorldBounds)(bounds.max_x, bounds.min_x, bounds.max_y, bounds.min_y);
    }
}

/*
    Wasted Settings
*/

pub trait QueryEnvironmentWastedSettings {
    fn get_wasted_settings(&self) -> WastedSettings;
}
impl QueryEnvironmentWastedSettings for VcmpFunctions {
    fn get_wasted_settings(&self) -> WastedSettings {
        let (
            mut death_timer,
            mut fade_timer,
            mut fade_in_speed,
            mut fade_out_speed,
            mut color,
            mut corpse_fade_start,
            mut corpse_fade_time,
        ) = (0, 0, 0f32, 0f32, 0, 0, 0);
        (self.inner.GetWastedSettings)(
            &mut death_timer,
            &mut fade_timer,
            &mut fade_in_speed,
            &mut fade_out_speed,
            &mut color,
            &mut corpse_fade_start,
            &mut corpse_fade_time,
        );

        WastedSettings {
            death_timer,
            fade_timer,
            fade_in_speed,
            fade_out_speed,
            color: Color::from_argb(color),
            corpse_fade_start,
            corpse_fade_time,
        }
    }
}

pub trait SetEnvironmentWastedSettings {
    fn set_wasted_settings(&self, settings: WastedSettings);
}
impl SetEnvironmentWastedSettings for VcmpFunctions {
    fn set_wasted_settings(&self, settings: WastedSettings) {
        (self.inner.SetWastedSettings)(
            settings.death_timer,
            settings.fade_timer,
            settings.fade_in_speed,
            settings.fade_out_speed,
            settings.color.to_rgba(),
            settings.corpse_fade_start,
            settings.corpse_fade_time,
        );
    }
}

/*
    World
*/

pub trait SetEnvironmentWorld {
    fn set_time_rate(&self, rate: i32);
    fn set_time(&self, time: i32);
    fn set_hour(&self, hour: i32);
    fn set_minute(&self, minute: i32);
    fn set_water_level(&self, level: f32);
    fn set_weather(&self, weather: i32);
    fn set_gravity(&self, gravity: f32);
    fn set_gamespeed(&self, gamespeed: f32);
    fn set_maximum_flight_altitude(&self, height: f32);
    fn set_kill_command_delay(&self, delay: i32);
    fn disable_kill_command_delay(&self);
    fn set_vehicles_forced_respawn_height(&self, height: f32);
}
impl SetEnvironmentWorld for VcmpFunctions {
    fn set_time_rate(&self, rate: i32) {
        (self.inner.SetTimeRate)(rate);
    }
    fn set_time(&self, time: i32) {
        // total 1440 = 0
        let hour = time % 24;
        let minute = time % 60;
        self.set_hour(hour);
        self.set_minute(minute)
    }
    fn set_hour(&self, hour: i32) {
        (self.inner.SetHour)(hour);
    }
    fn set_minute(&self, minute: i32) {
        (self.inner.SetMinute)(minute);
    }
    fn set_water_level(&self, level: f32) {
        (self.inner.SetWaterLevel)(level);
    }
    fn set_weather(&self, weather: i32) {
        (self.inner.SetWeather)(weather);
    }
    fn set_gravity(&self, gravity: f32) {
        (self.inner.SetGravity)(gravity);
    }
    fn set_gamespeed(&self, gamespeed: f32) {
        (self.inner.SetGameSpeed)(gamespeed);
    }
    fn set_maximum_flight_altitude(&self, height: f32) {
        (self.inner.SetMaximumFlightAltitude)(height);
    }
    fn set_kill_command_delay(&self, delay: i32) {
        (self.inner.SetKillCommandDelay)(delay);
    }
    fn disable_kill_command_delay(&self) {
        self.set_kill_command_delay(2147483647);
    }
    fn set_vehicles_forced_respawn_height(&self, height: f32) {
        (self.inner.SetVehiclesForcedRespawnHeight)(height);
    }
}

pub trait QueryEnvironmentWorld {
    fn get_time_rate(&self) -> i32;
    fn get_time(&self) -> i32;
    fn get_hour(&self) -> i32;
    fn get_minute(&self) -> i32;
    fn get_water_level(&self) -> f32;
    fn get_weather(&self) -> i32;
    fn get_gravity(&self) -> f32;
    fn get_gamespeed(&self) -> f32;
    fn get_maximum_flight_altitude(&self) -> f32;
    fn get_kill_command_delay(&self) -> i32;
    fn get_vehicles_forced_respawn_height(&self) -> f32;
}
impl QueryEnvironmentWorld for VcmpFunctions {
    fn get_time_rate(&self) -> i32 {
        (self.inner.GetTimeRate)()
    }
    fn get_time(&self) -> i32 {
        self.get_hour() * 60 + self.get_minute()
    }
    fn get_hour(&self) -> i32 {
        (self.inner.GetHour)()
    }
    fn get_minute(&self) -> i32 {
        (self.inner.GetMinute)()
    }
    fn get_water_level(&self) -> f32 {
        (self.inner.GetWaterLevel)()
    }
    fn get_weather(&self) -> i32 {
        (self.inner.GetWeather)()
    }
    fn get_gravity(&self) -> f32 {
        (self.inner.GetGravity)()
    }
    fn get_gamespeed(&self) -> f32 {
        (self.inner.GetGameSpeed)()
    }
    fn get_maximum_flight_altitude(&self) -> f32 {
        (self.inner.GetMaximumFlightAltitude)()
    }
    fn get_kill_command_delay(&self) -> i32 {
        (self.inner.GetKillCommandDelay)()
    }
    fn get_vehicles_forced_respawn_height(&self) -> f32 {
        (self.inner.GetVehiclesForcedRespawnHeight)()
    }
}
