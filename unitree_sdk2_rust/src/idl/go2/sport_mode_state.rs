//! Sport mode state IDL message.
//!
//! Contains the [`SportModeState`] struct with the full sport-mode telemetry
//! published by the robot, including pose, velocity, foot forces, and
//! planned trajectory.

use serde::{Deserialize, Serialize};
use super::{IMUState, PathPoint, TimeSpec};

/// Sport mode state published by the robot.
///
/// This is the primary telemetry message for sport/locomotion mode. It
/// includes the robot's pose, velocity, gait parameters, foot contact
/// forces, and a look-ahead trajectory of up to 10 [`PathPoint`]s.
///
/// # Examples
///
/// ```
/// use unitree_sdk2_rust::idl::go2::SportModeState;
///
/// let state = SportModeState::default();
/// assert_eq!(state.mode, 0);
/// assert_eq!(state.position, [0.0; 3]);
/// assert_eq!(state.path_point.len(), 10);
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SportModeState {
    /// Message timestamp.
    pub stamp: TimeSpec,
    /// Non-zero indicates an error condition.
    pub error_code: u32,
    /// IMU sensor readings.
    pub imu_state: IMUState,
    /// Current locomotion mode.
    pub mode: u8,
    /// Motion progress (0.0 to 1.0).
    pub progress: f32,
    /// Gait type identifier.
    pub gait_type: u8,
    /// Foot raise height in meters.
    pub foot_raise_height: f32,
    /// Position `[x, y, z]` in meters.
    pub position: [f32; 3],
    /// Body height in meters.
    pub body_height: f32,
    /// Velocity `[vx, vy, vz]` in m/s.
    pub velocity: [f32; 3],
    /// Yaw angular velocity in rad/s.
    pub yaw_speed: f32,
    /// Range obstacle distances `[front, right, rear, left]` in meters.
    pub range_obstacle: [f32; 4],
    /// Foot contact forces `[FL, FR, RL, RR]` in N.
    pub foot_force: [i16; 4],
    /// Foot positions relative to body (4 legs x 3 axes).
    pub foot_position_body: [f32; 12],
    /// Foot velocities relative to body (4 legs x 3 axes).
    pub foot_speed_body: [f32; 12],
    /// Planned trajectory (up to 10 waypoints).
    pub path_point: [PathPoint; 10],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values() {
        let state = SportModeState::default();
        assert_eq!(state.mode, 0);
        assert_eq!(state.error_code, 0);
        assert_eq!(state.position, [0.0; 3]);
        assert_eq!(state.velocity, [0.0; 3]);
        assert_eq!(state.foot_force, [0; 4]);
        assert_eq!(state.path_point.len(), 10);
    }

    #[test]
    fn serde_roundtrip() {
        let state = SportModeState {
            stamp: TimeSpec { sec: 10, nanosec: 100 },
            mode: 2,
            position: [1.0, 2.0, 0.3],
            velocity: [0.5, 0.0, 0.0],
            body_height: 0.32,
            ..Default::default()
        };
        let json = serde_json::to_string(&state).unwrap();
        let deserialized: SportModeState = serde_json::from_str(&json).unwrap();
        assert_eq!(state, deserialized);
    }
}
