//! Error types for the Unitree SDK.
//!
//! Provides [`SdkError`] for all error conditions in the SDK, and a
//! convenience [`Result`] type alias.

use thiserror::Error;

/// SDK-wide error type.
///
/// Covers channel communication errors, initialization failures,
/// serialization problems, timeouts, and API-level error codes.
///
/// # Examples
///
/// ```
/// use unitree_sdk2_rust::error::SdkError;
///
/// let err = SdkError::Timeout;
/// assert_eq!(format!("{err}"), "Timeout");
///
/// let err = SdkError::ApiError(42);
/// assert_eq!(format!("{err}"), "API error code: 42");
/// ```
#[derive(Debug, Error)]
pub enum SdkError {
    /// A channel-layer communication error.
    #[error("Channel error: {0}")]
    Channel(String),
    /// Failed to initialize a resource.
    #[error("Initialization error: {0}")]
    Init(String),
    /// Serialization or deserialization failure.
    #[error("Serialization error: {0}")]
    Serialization(String),
    /// An operation timed out.
    #[error("Timeout")]
    Timeout,
    /// A resource was used before being initialized.
    #[error("Not initialized")]
    NotInitialized,
    /// The robot API returned a non-zero error code.
    #[error("API error code: {0}")]
    ApiError(i32),
}

/// Convenience alias for `std::result::Result<T, SdkError>`.
pub type Result<T> = std::result::Result<T, SdkError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_channel_error() {
        let err = SdkError::Channel("connection refused".into());
        assert_eq!(format!("{err}"), "Channel error: connection refused");
    }

    #[test]
    fn display_init_error() {
        let err = SdkError::Init("already initialized".into());
        assert_eq!(format!("{err}"), "Initialization error: already initialized");
    }

    #[test]
    fn display_serialization_error() {
        let err = SdkError::Serialization("invalid json".into());
        assert_eq!(format!("{err}"), "Serialization error: invalid json");
    }

    #[test]
    fn display_timeout() {
        let err = SdkError::Timeout;
        assert_eq!(format!("{err}"), "Timeout");
    }

    #[test]
    fn display_not_initialized() {
        let err = SdkError::NotInitialized;
        assert_eq!(format!("{err}"), "Not initialized");
    }

    #[test]
    fn display_api_error() {
        let err = SdkError::ApiError(-1);
        assert_eq!(format!("{err}"), "API error code: -1");
    }

    #[test]
    fn debug_format() {
        let err = SdkError::Timeout;
        let debug = format!("{:?}", err);
        assert!(debug.contains("Timeout"));
    }

    #[test]
    fn is_std_error() {
        let err: Box<dyn std::error::Error> = Box::new(SdkError::Timeout);
        assert_eq!(format!("{err}"), "Timeout");
    }
}
