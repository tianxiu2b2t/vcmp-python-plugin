use std::{error::Error, fmt::Display};

pub type VcmpResult<T> = Result<T, VcmpError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VcmpError {
    None,
    NoSuchEntity,
    BufferTooSmall,
    TooLargeInput,
    ArgumentOutOfBounds,
    NullArgument,
    PoolExhausted,
    InvalidName,
    RequestDenied,
    ForceSize,
    Unknown(i32),
}

impl From<i32> for VcmpError {
    fn from(value: i32) -> Self {
        match value {
            0 => VcmpError::None,
            1 => VcmpError::NoSuchEntity,
            2 => VcmpError::BufferTooSmall,
            3 => VcmpError::TooLargeInput,
            4 => VcmpError::ArgumentOutOfBounds,
            5 => VcmpError::NullArgument,
            6 => VcmpError::PoolExhausted,
            7 => VcmpError::InvalidName,
            8 => VcmpError::RequestDenied,
            i32::MAX => VcmpError::ForceSize,
            _ => VcmpError::Unknown(value),
        }
    }
}

impl From<VcmpError> for i32 {
    fn from(val: VcmpError) -> Self {
        match val {
            VcmpError::None => 0,
            VcmpError::NoSuchEntity => 1,
            VcmpError::BufferTooSmall => 2,
            VcmpError::TooLargeInput => 3,
            VcmpError::ArgumentOutOfBounds => 4,
            VcmpError::NullArgument => 5,
            VcmpError::PoolExhausted => 6,
            VcmpError::InvalidName => 7,
            VcmpError::RequestDenied => 8,
            VcmpError::ForceSize => i32::MAX,
            VcmpError::Unknown(x) => x,
        }
    }
}

impl Error for VcmpError {}

impl Display for VcmpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VcmpError::None => write!(f, "无错误"),
            VcmpError::NoSuchEntity => write!(f, "目标实体不存在"),
            VcmpError::BufferTooSmall => write!(f, "缓冲区大小不足"),
            VcmpError::TooLargeInput => write!(f, "输入数据过大"),
            VcmpError::ArgumentOutOfBounds => write!(f, "参数超出有效范围"),
            VcmpError::NullArgument => write!(f, "接收到非法空参数"),
            VcmpError::PoolExhausted => write!(f, "资源池已耗尽"),
            VcmpError::InvalidName => write!(f, "无效名称"),
            VcmpError::RequestDenied => write!(f, "请求被拒绝"),
            VcmpError::ForceSize => write!(f, "vcmp 不知道发生了什么, 所以丢了个 ForceSize 过来"),
            VcmpError::Unknown(code) => write!(f, "未知错误码 {code}"),
        }
    }
}
