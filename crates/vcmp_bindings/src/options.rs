#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VcmpNetworkStatisticsQueryOption {
    DataSentPerSecond, // 第一个变体，值=0
    DataResentPerSecond,
    DataReceivedPerSecond,
    DataDiscardedPerSecond,
    AllBytesSentPerSecond,
    AllBytesReceivedPerSecond,
    DataSentTotal,
    DataResentTotal,
    DataReceivedTotal,
    DataDiscardedTotal,
    AllBytesSentTotal,
    AllBytesReceivedTotal,
    MessagesWaiting,
    MessagesResending,
    BytesResending,
    PacketLossPerSecond,
    PacketLossTotal,
}

impl From<i32> for VcmpNetworkStatisticsQueryOption {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::DataSentPerSecond,
            1 => Self::DataResentPerSecond,
            2 => Self::DataReceivedPerSecond,
            3 => Self::DataDiscardedPerSecond,
            4 => Self::AllBytesSentPerSecond,
            5 => Self::AllBytesReceivedPerSecond,
            6 => Self::DataSentTotal,
            7 => Self::DataResentTotal,
            8 => Self::DataReceivedTotal,
            9 => Self::DataDiscardedTotal,
            10 => Self::AllBytesSentTotal,
            11 => Self::AllBytesReceivedTotal,
            12 => Self::MessagesWaiting,
            13 => Self::MessagesResending,
            14 => Self::BytesResending,
            15 => Self::PacketLossPerSecond,
            16 => Self::PacketLossTotal,
            _ => Self::DataSentPerSecond, // 未知值转为第一个变体
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VcmpServerOption {
    SyncFrameLimiter, // 第一个变体，值=0
    FrameLimiter,
    TaxiBoostJump,
    DriveOnWater,
    FastSwitch,
    FriendlyFire,
    DisableDriveBy,
    PerfectHandling,
    FlyingCars,
    JumpSwitch,
    ShowMarkers,
    OnlyShowTeamMarkers,
    StuntBike,
    ShootInAir,
    ShowNameTags,
    JoinMessages,
    DeathMessages,
    ChatTagsEnabled,
    UseClasses,
    WallGlitch,
    DisableBackfaceCulling,
    DisableHeliBladeDamage,
    DisableCrouch,
}

impl From<i32> for VcmpServerOption {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::SyncFrameLimiter,
            1 => Self::FrameLimiter,
            2 => Self::TaxiBoostJump,
            3 => Self::DriveOnWater,
            4 => Self::FastSwitch,
            5 => Self::FriendlyFire,
            6 => Self::DisableDriveBy,
            7 => Self::PerfectHandling,
            8 => Self::FlyingCars,
            9 => Self::JumpSwitch,
            10 => Self::ShowMarkers,
            11 => Self::OnlyShowTeamMarkers,
            12 => Self::StuntBike,
            13 => Self::ShootInAir,
            14 => Self::ShowNameTags,
            15 => Self::JoinMessages,
            16 => Self::DeathMessages,
            17 => Self::ChatTagsEnabled,
            18 => Self::UseClasses,
            19 => Self::WallGlitch,
            20 => Self::DisableBackfaceCulling,
            21 => Self::DisableHeliBladeDamage,
            22 => Self::DisableCrouch,
            _ => Self::SyncFrameLimiter, // 未知值转为第一个变体
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VcmpPlayerOption {
    Controllable, // 第一个变体，值=0
    DriveBy,
    WhiteScanlines,
    GreenScanlines,
    Widescreen,
    ShowMarkers,
    CanAttack,
    HasMarker,
    ChatTagsEnabled,
    DrunkEffectsDeprecated,
    Bleeding,
}

impl From<i32> for VcmpPlayerOption {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Controllable,
            1 => Self::DriveBy,
            2 => Self::WhiteScanlines,
            3 => Self::GreenScanlines,
            4 => Self::Widescreen,
            5 => Self::ShowMarkers,
            6 => Self::CanAttack,
            7 => Self::HasMarker,
            8 => Self::ChatTagsEnabled,
            9 => Self::DrunkEffectsDeprecated,
            10 => Self::Bleeding,
            _ => Self::Controllable, // 未知值转为第一个变体
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VcmpVehicleOption {
    DoorsLocked, // 第一个变体，值=0
    Alarm,
    Lights,
    RadioLocked,
    Ghost,
    Siren,
    SingleUse,
    EngineDisabled,
    BootOpen,
    BonnetOpen,
}

impl From<i32> for VcmpVehicleOption {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::DoorsLocked,
            1 => Self::Alarm,
            2 => Self::Lights,
            3 => Self::RadioLocked,
            4 => Self::Ghost,
            5 => Self::Siren,
            6 => Self::SingleUse,
            7 => Self::EngineDisabled,
            8 => Self::BootOpen,
            9 => Self::BonnetOpen,
            _ => Self::DoorsLocked, // 未知值转为第一个变体
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VcmpPickupOption {
    SingleUse, // 第一个变体，值=0
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VcmpEntityPool {
    Vehicle,
    Object,
    Pickup,
    Radio,
    Player,
    Reserved1,
    Blip,
    CheckPoint,
}

impl From<i32> for VcmpEntityPool {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Vehicle,
            2 => Self::Object,
            3 => Self::Pickup,
            4 => Self::Radio,
            5 => Self::Player,
            6 => Self::Reserved1,
            7 => Self::Blip,
            8 => Self::CheckPoint,
            _ => Self::Reserved1, // 未知值转为第一个变体
        }
    }
}

impl From<VcmpEntityPool> for i32 {
    fn from(val: VcmpEntityPool) -> Self {
        val as i32
    }
}

impl From<i32> for VcmpPickupOption {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::SingleUse,
            _ => Self::SingleUse, // 所有未知值也转为SingleUse
        }
    }
}

impl From<VcmpNetworkStatisticsQueryOption> for i32 {
    fn from(val: VcmpNetworkStatisticsQueryOption) -> Self {
        val as i32
    }
}

impl From<VcmpServerOption> for i32 {
    fn from(val: VcmpServerOption) -> Self {
        val as i32
    }
}

impl From<VcmpPlayerOption> for i32 {
    fn from(val: VcmpPlayerOption) -> Self {
        val as i32
    }
}

impl From<VcmpVehicleOption> for i32 {
    fn from(val: VcmpVehicleOption) -> Self {
        val as i32
    }
}

impl From<VcmpPickupOption> for i32 {
    fn from(val: VcmpPickupOption) -> Self {
        val as i32
    }
}
