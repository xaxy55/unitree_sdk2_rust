//! BmsState IDL message.
use serde::{Deserialize, Serialize};

/// Battery management system state.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BmsState {
    pub version_high: u8,
    pub version_low: u8,
    pub status: u8,
    pub soc: u8,
    pub current: i32,
    pub cycle: u16,
    pub bq_ntc: [u8; 2],
    pub mcu_ntc: [u8; 2],
    pub cell_vol: [u16; 15],
}
