//! IMUState IDL message.
use serde::{Deserialize, Serialize};

/// IMU sensor state.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct IMUState {
    pub quaternion: [f32; 4],
    pub gyroscope: [f32; 3],
    pub accelerometer: [f32; 3],
    pub rpy: [f32; 3],
    pub temperature: u8,
}
