//! LowState IDL message.
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use super::{BmsState, IMUState, MotorState};

/// Low-level state received from the robot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LowState {
    pub head: [u8; 2],
    pub level_flag: u8,
    pub frame_reserve: u8,
    pub sn: [u32; 2],
    pub version: [u32; 2],
    pub bandwidth: u16,
    pub imu_state: IMUState,
    pub motor_state: [MotorState; 20],
    pub bms_state: BmsState,
    #[serde(with = "BigArray")]
    pub wireless_remote: [u8; 40],
    pub foot_force: [i16; 4],
    pub foot_force_est: [i16; 4],
    pub tick: u32,
    #[serde(with = "BigArray")]
    pub wireless_remote2: [u8; 40],
    pub reserve: u8,
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
