//! Timestamp IDL message.
//!
//! Contains the [`TimeSpec`] struct for representing timestamps with
//! second and nanosecond precision.

use serde::{Deserialize, Serialize};

/// Timestamp with seconds and nanoseconds.
///
/// Compatible with POSIX `timespec`. Used in [`super::SportModeState`]
/// to timestamp state messages.
///
/// # Examples
///
/// ```
/// use unitree_sdk2_rust::idl::go2::TimeSpec;
///
/// let ts = TimeSpec { sec: 100, nanosec: 500_000_000 };
/// assert_eq!(ts.sec, 100);
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TimeSpec {
    /// Whole seconds since epoch.
    pub sec: i32,
    /// Nanosecond fraction (0-999_999_999).
    pub nanosec: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values() {
        let ts = TimeSpec::default();
        assert_eq!(ts.sec, 0);
        assert_eq!(ts.nanosec, 0);
    }

    #[test]
    fn serde_roundtrip() {
        let ts = TimeSpec {
            sec: 1700000000,
            nanosec: 123456789,
        };
        let json = serde_json::to_string(&ts).unwrap();
        let deserialized: TimeSpec = serde_json::from_str(&json).unwrap();
        assert_eq!(ts, deserialized);
    }
}
