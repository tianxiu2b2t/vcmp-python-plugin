use crate::{VcmpFunctions, options::VcmpServerOption};

pub trait QueryEnvironment {
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

impl QueryEnvironment for VcmpFunctions {
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

pub trait SetEnvironment {
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

impl SetEnvironment for VcmpFunctions {
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
