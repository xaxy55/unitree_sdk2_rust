//! Low-level state IDL message.
//!
//! Contains the [`LowState`] struct with the full low-level telemetry
//! received from the robot, including IMU data, motor states, battery
//! state, and foot force sensors.

use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use super::{BmsState, IMUState, MotorState};

/// Low-level state received from the robot.
///
/// This is the primary telemetry message for low-level (joint-space) feedback.
/// It contains IMU readings, motor states for all 20 joints, battery state,
/// foot force measurements, and wireless remote data.
///
/// # Examples
///
/// ```
/// use unitree_sdk2_rust::idl::go2::LowState;
///
/// let state = LowState::default();
/// assert_eq!(state.motor_state.len(), 20);
/// assert_eq!(state.foot_force, [0; 4]);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LowState {
    /// Message header bytes.
    pub head: [u8; 2],
    /// Control level flag.
    pub level_flag: u8,
    /// Reserved frame byte.
    pub frame_reserve: u8,
    /// Serial number (2 x u32).
    pub sn: [u32; 2],
    /// Firmware version (2 x u32).
    pub version: [u32; 2],
    /// Communication bandwidth.
    pub bandwidth: u16,
    /// IMU sensor state.
    pub imu_state: IMUState,
    /// Motor states for all 20 joints.
    pub motor_state: [MotorState; 20],
    /// Battery management system state.
    pub bms_state: BmsState,
    /// Wireless remote control data (40 bytes).
    #[serde(with = "BigArray")]
    pub wireless_remote: [u8; 40],
    /// Foot contact force sensors `[FL, FR, RL, RR]`.
    pub foot_force: [i16; 4],
    /// Estimated foot forces `[FL, FR, RL, RR]`.
    pub foot_force_est: [i16; 4],
    /// Monotonic tick counter.
    pub tick: u32,
    /// Secondary wireless remote data (40 bytes).
    #[serde(with = "BigArray")]
    pub wireless_remote2: [u8; 40],
    /// Reserved byte.
    pub reserve: u8,
    /// CRC32 checksum.
    pub crc: u32,
}

impl Default for LowState {
    fn default() -> Self {
        Self {
            head: Default::default(),
            level_flag: 0,
            frame_reserve: 0,
            sn: Default::default(),
            version: Default::default(),
            bandwidth: 0,
            imu_state: IMUState::default(),
            motor_state: std::array::from_fn(|_| MotorState::default()),
            bms_state: BmsState::default(),
            wireless_remote: [0u8; 40],
            foot_force: Default::default(),
            foot_force_est: Default::default(),
            tick: 0,
            wireless_remote2: [0u8; 40],
            reserve: 0,
            crc: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values() {
        let state = LowState::default();
        assert_eq!(state.head, [0, 0]);
        assert_eq!(state.motor_state.len(), 20);
        assert_eq!(state.foot_force, [0; 4]);
        assert_eq!(state.foot_force_est, [0; 4]);
        assert_eq!(state.tick, 0);
        assert_eq!(state.crc, 0);
    }

    #[test]
    fn serde_roundtrip() {
        let mut state = LowState::default();
        state.head = [0xFE, 0xEF];
        state.tick = 999;
        state.imu_state.temperature = 30;
        state.motor_state[0].q = 1.5;
        state.bms_state.soc = 85;
        let json = serde_json::to_string(&state).unwrap();
        let deserialized: LowState = serde_json::from_str(&json).unwrap();
        assert_eq!(state, deserialized);
    }

    #[test]
    fn clone_preserves_nested() {
        let mut state = LowState::default();
        state.bms_state.soc = 50;
        state.motor_state[10].temperature = 60;
        let cloned = state.clone();
        assert_eq!(cloned.bms_state.soc, 50);
        assert_eq!(cloned.motor_state[10].temperature, 60);
    }
}
