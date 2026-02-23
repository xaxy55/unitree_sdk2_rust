//! MotorCmd IDL message.
use serde::{Deserialize, Serialize};

/// Motor command.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MotorCmd {
    pub mode: u8,
    pub q: f32,
    pub dq: f32,
    pub tau: f32,
    pub kp: f32,
    pub kd: f32,
    pub reserve: [u32; 3],
}
