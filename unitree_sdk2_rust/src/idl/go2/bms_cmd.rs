//! BMS (Battery Management System) command message.
//!
//! Contains the [`BmsCmd`] struct used to send battery management commands
//! to the robot, such as powering off.

use serde::{Deserialize, Serialize};

/// Battery management system command.
///
/// Used to control the robot's battery management system. The `off` field
/// triggers a power-off when set to a non-zero value.
///
/// # Examples
///
/// ```
/// use unitree_sdk2_rust::idl::go2::BmsCmd;
///
/// let cmd = BmsCmd::default();
/// assert_eq!(cmd.off, 0);
/// assert_eq!(cmd.reserve, [0, 0, 0]);
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BmsCmd {
    /// Power-off flag. Non-zero triggers shutdown.
    pub off: u8,
    /// Reserved bytes for future use.
    pub reserve: [u8; 3],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values() {
        let cmd = BmsCmd::default();
        assert_eq!(cmd.off, 0);
        assert_eq!(cmd.reserve, [0, 0, 0]);
    }

    #[test]
    fn serde_roundtrip() {
        let cmd = BmsCmd {
            off: 1,
            reserve: [2, 3, 4],
        };
        let json = serde_json::to_string(&cmd).unwrap();
        let deserialized: BmsCmd = serde_json::from_str(&json).unwrap();
        assert_eq!(cmd, deserialized);
    }

    #[test]
    fn clone_and_eq() {
        let cmd = BmsCmd {
            off: 5,
            reserve: [10, 20, 30],
        };
        let cloned = cmd.clone();
        assert_eq!(cmd, cloned);
    }

    #[test]
    fn debug_format() {
        let cmd = BmsCmd::default();
        let debug = format!("{:?}", cmd);
        assert!(debug.contains("BmsCmd"));
    }
}
