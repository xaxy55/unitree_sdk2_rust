//! Low-level command IDL message.
//!
//! Contains the [`LowCmd`] struct used to send direct motor commands,
//! BMS commands, and LED/GPIO controls to the robot.

use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use super::{BmsCmd, MotorCmd};

/// Low-level command sent to the robot.
///
/// This is the primary control message for low-level (joint-space) control.
/// It contains motor commands for all 20 joints, a BMS command, wireless
/// remote data, LED settings, and a CRC checksum.
///
/// # Examples
///
/// ```
/// use unitree_sdk2_rust::idl::go2::LowCmd;
///
/// let cmd = LowCmd::default();
/// assert_eq!(cmd.motor_cmd.len(), 20);
/// assert_eq!(cmd.crc, 0);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LowCmd {
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
    /// Motor commands for all 20 joints.
    pub motor_cmd: [MotorCmd; 20],
    /// Battery management system command.
    pub bms_cmd: BmsCmd,
    /// Wireless remote control data (40 bytes).
    #[serde(with = "BigArray")]
    pub wireless_remote: [u8; 40],
    /// LED control data (12 bytes).
    pub led: [u8; 12],
    /// Fan control (2 bytes).
    pub fan: [u8; 2],
    /// GPIO output byte.
    pub gpio: u8,
    /// Reserved field.
    pub reserve: u32,
    /// CRC32 checksum.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values() {
        let cmd = LowCmd::default();
        assert_eq!(cmd.head, [0, 0]);
        assert_eq!(cmd.level_flag, 0);
        assert_eq!(cmd.motor_cmd.len(), 20);
        assert_eq!(cmd.wireless_remote, [0u8; 40]);
        assert_eq!(cmd.crc, 0);
        for mc in &cmd.motor_cmd {
            assert_eq!(*mc, MotorCmd::default());
        }
    }

    #[test]
    fn serde_roundtrip() {
        let mut cmd = LowCmd::default();
        cmd.head = [0xFE, 0xEF];
        cmd.level_flag = 0xFF;
        cmd.motor_cmd[0].mode = 1;
        cmd.motor_cmd[0].q = 0.5;
        cmd.crc = 12345;
        let json = serde_json::to_string(&cmd).unwrap();
        let deserialized: LowCmd = serde_json::from_str(&json).unwrap();
        assert_eq!(cmd, deserialized);
    }

    #[test]
    fn clone_preserves_motor_cmds() {
        let mut cmd = LowCmd::default();
        cmd.motor_cmd[5].kp = 42.0;
        let cloned = cmd.clone();
        assert_eq!(cloned.motor_cmd[5].kp, 42.0);
    }
}
