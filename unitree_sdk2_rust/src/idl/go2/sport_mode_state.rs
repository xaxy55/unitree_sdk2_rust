//! SportModeState IDL message.
use serde::{Deserialize, Serialize};
use super::{IMUState, PathPoint, TimeSpec};

/// Sport mode state published by the robot.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SportModeState {
    pub stamp: TimeSpec,
    pub error_code: u32,
    pub imu_state: IMUState,
    pub mode: u8,
    pub progress: f32,
    pub gait_type: u8,
    pub foot_raise_height: f32,
    pub position: [f32; 3],
    pub body_height: f32,
    pub velocity: [f32; 3],
    pub yaw_speed: f32,
    pub range_obstacle: [f32; 4],
    pub foot_force: [i16; 4],
    pub foot_position_body: [f32; 12],
    pub foot_speed_body: [f32; 12],
    pub path_point: [PathPoint; 10],
}
