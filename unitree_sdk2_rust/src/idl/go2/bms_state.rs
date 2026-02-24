//! BMS (Battery Management System) state message.
//!
//! Contains the [`BmsState`] struct representing the current state of the
//! robot's battery, including voltage, current, temperature, and charge level.

use serde::{Deserialize, Serialize};

/// Battery management system state.
///
/// Reports the robot's battery status including firmware version, state of
/// charge (`soc`), current draw, cycle count, temperatures, and individual
/// cell voltages.
///
/// # Examples
///
/// ```
/// use unitree_sdk2_rust::idl::go2::BmsState;
///
/// let state = BmsState::default();
/// assert_eq!(state.soc, 0);
/// assert_eq!(state.cell_vol.len(), 15);
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BmsState {
    /// Firmware version high byte.
    pub version_high: u8,
    /// Firmware version low byte.
    pub version_low: u8,
    /// BMS status flags.
    pub status: u8,
    /// State of charge (0-100%).
    pub soc: u8,
    /// Battery current in mA (negative = discharging).
    pub current: i32,
    /// Charge/discharge cycle count.
    pub cycle: u16,
    /// BQ chip NTC temperature readings.
    pub bq_ntc: [u8; 2],
    /// MCU NTC temperature readings.
    pub mcu_ntc: [u8; 2],
    /// Individual cell voltages in mV (15 cells).
    pub cell_vol: [u16; 15],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values() {
        let state = BmsState::default();
        assert_eq!(state.version_high, 0);
        assert_eq!(state.version_low, 0);
        assert_eq!(state.status, 0);
        assert_eq!(state.soc, 0);
        assert_eq!(state.current, 0);
        assert_eq!(state.cycle, 0);
        assert_eq!(state.bq_ntc, [0, 0]);
        assert_eq!(state.mcu_ntc, [0, 0]);
        assert_eq!(state.cell_vol, [0u16; 15]);
    }

    #[test]
    fn serde_roundtrip() {
        let state = BmsState {
            version_high: 1,
            version_low: 2,
            status: 3,
            soc: 80,
            current: -1500,
            cycle: 42,
            bq_ntc: [25, 26],
            mcu_ntc: [30, 31],
            cell_vol: [4200; 15],
        };
        let json = serde_json::to_string(&state).unwrap();
        let deserialized: BmsState = serde_json::from_str(&json).unwrap();
        assert_eq!(state, deserialized);
    }

    #[test]
    fn clone_and_eq() {
        let state = BmsState {
            soc: 95,
            current: -500,
            ..Default::default()
        };
        assert_eq!(state, state.clone());
    }
}
