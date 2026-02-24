//! Motor command IDL message.
//!
//! Contains the [`MotorCmd`] struct used to command individual joint motors
//! with position, velocity, torque, and PD gain targets.

use serde::{Deserialize, Serialize};

/// Motor command for a single joint.
///
/// Each motor on the robot receives position (`q`), velocity (`dq`),
/// torque (`tau`), and PD gain (`kp`, `kd`) targets.
///
/// # Examples
///
/// ```
/// use unitree_sdk2_rust::idl::go2::MotorCmd;
///
/// let cmd = MotorCmd {
///     mode: 1,
///     q: 0.5,
///     kp: 20.0,
///     kd: 0.5,
///     ..Default::default()
/// };
/// assert_eq!(cmd.mode, 1);
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MotorCmd {
    /// Control mode (0 = idle, 1 = position, etc.).
    pub mode: u8,
    /// Target joint position in radians.
    pub q: f32,
    /// Target joint velocity in rad/s.
    pub dq: f32,
    /// Feed-forward torque in Nm.
    pub tau: f32,
    /// Position proportional gain.
    pub kp: f32,
    /// Velocity derivative gain.
    pub kd: f32,
    /// Reserved for future use.
    pub reserve: [u32; 3],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values() {
        let cmd = MotorCmd::default();
        assert_eq!(cmd.mode, 0);
        assert_eq!(cmd.q, 0.0);
        assert_eq!(cmd.dq, 0.0);
        assert_eq!(cmd.tau, 0.0);
        assert_eq!(cmd.kp, 0.0);
        assert_eq!(cmd.kd, 0.0);
        assert_eq!(cmd.reserve, [0; 3]);
    }

    #[test]
    fn serde_roundtrip() {
        let cmd = MotorCmd {
            mode: 1,
            q: 1.57,
            dq: 0.5,
            tau: 2.0,
            kp: 20.0,
            kd: 0.5,
            reserve: [1, 2, 3],
        };
        let json = serde_json::to_string(&cmd).unwrap();
        let deserialized: MotorCmd = serde_json::from_str(&json).unwrap();
        assert_eq!(cmd, deserialized);
    }
}
