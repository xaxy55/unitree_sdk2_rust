//! Wireless controller IDL message.
//!
//! Contains the [`WirelessController`] struct representing the state of
//! the robot's wireless handheld controller (joystick positions and button presses).

use serde::{Deserialize, Serialize};

/// Wireless controller input state.
///
/// Reports the position of both analog sticks and the bitmask of pressed buttons.
///
/// # Examples
///
/// ```
/// use unitree_sdk2_rust::idl::go2::WirelessController;
///
/// let ctrl = WirelessController::default();
/// assert_eq!(ctrl.lx, 0.0);
/// assert_eq!(ctrl.keys, 0);
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WirelessController {
    /// Left stick X axis (-1.0 to 1.0).
    pub lx: f32,
    /// Left stick Y axis (-1.0 to 1.0).
    pub ly: f32,
    /// Right stick X axis (-1.0 to 1.0).
    pub rx: f32,
    /// Right stick Y axis (-1.0 to 1.0).
    pub ry: f32,
    /// Button bitmask (each bit represents a button).
    pub keys: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values() {
        let ctrl = WirelessController::default();
        assert_eq!(ctrl.lx, 0.0);
        assert_eq!(ctrl.ly, 0.0);
        assert_eq!(ctrl.rx, 0.0);
        assert_eq!(ctrl.ry, 0.0);
        assert_eq!(ctrl.keys, 0);
    }

    #[test]
    fn serde_roundtrip() {
        let ctrl = WirelessController {
            lx: -0.5,
            ly: 0.8,
            rx: 0.3,
            ry: -0.9,
            keys: 0b1010_0101,
        };
        let json = serde_json::to_string(&ctrl).unwrap();
        let deserialized: WirelessController = serde_json::from_str(&json).unwrap();
        assert_eq!(ctrl, deserialized);
    }
}
