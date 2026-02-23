//! BmsCmd IDL message.
use serde::{Deserialize, Serialize};

/// Battery management system command.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BmsCmd {
    pub off: u8,
    pub reserve: [u8; 3],
}
