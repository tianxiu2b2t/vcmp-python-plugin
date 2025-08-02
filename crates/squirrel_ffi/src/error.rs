use std::fmt::Display;

pub type SQFFIResult<T> = Result<T, SQError>;

#[derive(Debug)]
pub enum SQError {
    FailedToAttachPlugin,
}

impl Display for SQError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SQError::FailedToAttachPlugin => write!(f, "Failed to attach SQHost2 plugin"),
        }
    }
}