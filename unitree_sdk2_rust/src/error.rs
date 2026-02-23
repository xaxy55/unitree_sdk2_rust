//! Error types for the Unitree SDK.
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SdkError {
    #[error("Channel error: {0}")]
    Channel(String),
    #[error("Initialization error: {0}")]
    Init(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Timeout")]
    Timeout,
    #[error("Not initialized")]
    NotInitialized,
    #[error("API error code: {0}")]
    ApiError(i32),
}

pub type Result<T> = std::result::Result<T, SdkError>;
