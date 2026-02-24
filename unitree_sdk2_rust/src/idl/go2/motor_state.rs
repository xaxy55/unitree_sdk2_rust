//! Motor state IDL message.
//!
//! Contains the [`MotorState`] struct reporting the measured state of a
//! single joint motor, including position, velocity, acceleration, and
//! estimated torque.

use serde::{Deserialize, Serialize};

/// Motor state reported by a single joint.
///
/// Provides both filtered and raw readings for position, velocity, and
/// acceleration, along with estimated torque and motor temperature.
///
/// # Examples
///
/// ```
/// use unitree_sdk2_rust::idl::go2::MotorState;
///
/// let state = MotorState::default();
/// assert_eq!(state.mode, 0);
/// assert_eq!(state.temperature, 0);
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MotorState {
    /// Current control mode.
    pub mode: u8,
    /// Filtered joint position in radians.
    pub q: f32,
    /// Filtered joint velocity in rad/s.
    pub dq: f32,
    /// Filtered joint acceleration in rad/s^2.
    pub ddq: f32,
    /// Estimated joint torque in Nm.
    pub tau_est: f32,
    /// Raw (unfiltered) joint position in radians.
    pub q_raw: f32,
    /// Raw (unfiltered) joint velocity in rad/s.
    pub dq_raw: f32,
    /// Raw (unfiltered) joint acceleration in rad/s^2.
    pub ddq_raw: f32,
    /// Motor temperature in degrees Celsius.
    pub temperature: u8,
    /// Lost packet count.
    pub lost: u32,
    /// Reserved for future use.
    pub reserve: [u32; 2],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values() {
        let state = MotorState::default();
        assert_eq!(state.mode, 0);
        assert_eq!(state.q, 0.0);
        assert_eq!(state.tau_est, 0.0);
        assert_eq!(state.temperature, 0);
        assert_eq!(state.lost, 0);
    }

    #[test]
    fn serde_roundtrip() {
        let state = MotorState {
            mode: 1,
            q: 1.0,
            dq: 0.5,
            ddq: 0.1,
            tau_est: 3.0,
            q_raw: 1.01,
            dq_raw: 0.51,
            ddq_raw: 0.11,
            temperature: 45,
            lost: 2,
            reserve: [0, 0],
        };
        let json = serde_json::to_string(&state).unwrap();
        let deserialized: MotorState = serde_json::from_str(&json).unwrap();
        assert_eq!(state, deserialized);
    }
}
