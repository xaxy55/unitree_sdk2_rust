//! MotorState IDL message.
use serde::{Deserialize, Serialize};

/// Motor state reported by the robot.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MotorState {
    pub mode: u8,
    pub q: f32,
    pub dq: f32,
    pub ddq: f32,
    pub tau_est: f32,
    pub q_raw: f32,
    pub dq_raw: f32,
    pub ddq_raw: f32,
    pub temperature: u8,
    pub lost: u32,
    pub reserve: [u32; 2],
}
