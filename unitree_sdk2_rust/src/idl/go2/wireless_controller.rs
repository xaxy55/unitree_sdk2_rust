//! WirelessController IDL message.
use serde::{Deserialize, Serialize};

/// Wireless controller input state.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WirelessController {
    pub lx: f32,
    pub ly: f32,
    pub rx: f32,
    pub ry: f32,
    pub keys: u16,
}
