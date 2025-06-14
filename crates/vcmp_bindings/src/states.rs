#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VcmpPlayerState {
    None,
    Normal,
    Aim,
    Driver,
    Passenger,
    EnterDriver,
    EnterPassenger,
    Exit,
    Unspawned,
}

impl From<i32> for VcmpPlayerState {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Normal,
            2 => Self::Aim,
            3 => Self::Driver,
            4 => Self::Passenger,
            5 => Self::EnterDriver,
            6 => Self::EnterPassenger,
            7 => Self::Exit,
            8 => Self::Unspawned,
            _ => Self::None, // 未知值转为第一个变体
        }
    }
}
