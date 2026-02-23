//! TimeSpec IDL message.
use serde::{Deserialize, Serialize};

/// Timestamp with seconds and nanoseconds.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TimeSpec {
    pub sec: i32,
    pub nanosec: u32,
}
