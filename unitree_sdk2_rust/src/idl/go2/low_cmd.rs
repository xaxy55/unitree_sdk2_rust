//! LowCmd IDL message.
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use super::{BmsCmd, MotorCmd};

/// Low-level command sent to the robot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LowCmd {
    pub head: [u8; 2],
    pub level_flag: u8,
    pub frame_reserve: u8,
    pub sn: [u32; 2],
    pub version: [u32; 2],
    pub bandwidth: u16,
    pub motor_cmd: [MotorCmd; 20],
    pub bms_cmd: BmsCmd,
    #[serde(with = "BigArray")]
    pub wireless_remote: [u8; 40],
    pub led: [u8; 12],
    pub fan: [u8; 2],
    pub gpio: u8,
    pub reserve: u32,
    pub crc: u32,
}

impl Default for LowCmd {
    fn default() -> Self {
        Self {
            head: Default::default(),
            level_flag: 0,
            frame_reserve: 0,
            sn: Default::default(),
            version: Default::default(),
            bandwidth: 0,
            motor_cmd: std::array::from_fn(|_| MotorCmd::default()),
            bms_cmd: BmsCmd::default(),
            wireless_remote: [0u8; 40],
            led: [0u8; 12],
            fan: Default::default(),
            gpio: 0,
            reserve: 0,
            crc: 0,
        }
    }
}
