//! PathPoint IDL message.
use serde::{Deserialize, Serialize};

/// A point on a path with time offset and pose/velocity.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PathPoint {
    pub t_from_start: f32,
    pub x: f32,
    pub y: f32,
    pub yaw: f32,
    pub vx: f32,
    pub vy: f32,
    pub vyaw: f32,
}
