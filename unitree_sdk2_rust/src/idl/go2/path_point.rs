//! Path point IDL message.
//!
//! Contains the [`PathPoint`] struct representing a single waypoint in
//! a planned trajectory, with position, heading, and velocity components.

use serde::{Deserialize, Serialize};

/// A point on a path with time offset, pose, and velocity.
///
/// Used in [`super::SportModeState`] to describe the robot's planned
/// trajectory as a sequence of waypoints.
///
/// # Examples
///
/// ```
/// use unitree_sdk2_rust::idl::go2::PathPoint;
///
/// let point = PathPoint {
///     t_from_start: 1.0,
///     x: 2.0,
///     y: 0.5,
///     yaw: 0.1,
///     ..Default::default()
/// };
/// assert_eq!(point.t_from_start, 1.0);
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PathPoint {
    /// Time offset from trajectory start in seconds.
    pub t_from_start: f32,
    /// X position in meters (body frame).
    pub x: f32,
    /// Y position in meters (body frame).
    pub y: f32,
    /// Heading angle in radians.
    pub yaw: f32,
    /// X velocity in m/s.
    pub vx: f32,
    /// Y velocity in m/s.
    pub vy: f32,
    /// Yaw rate in rad/s.
    pub vyaw: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values() {
        let p = PathPoint::default();
        assert_eq!(p.t_from_start, 0.0);
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
        assert_eq!(p.yaw, 0.0);
        assert_eq!(p.vx, 0.0);
        assert_eq!(p.vy, 0.0);
        assert_eq!(p.vyaw, 0.0);
    }

    #[test]
    fn serde_roundtrip() {
        let p = PathPoint {
            t_from_start: 0.5,
            x: 1.0,
            y: 2.0,
            yaw: 0.3,
            vx: 0.5,
            vy: 0.1,
            vyaw: 0.05,
        };
        let json = serde_json::to_string(&p).unwrap();
        let deserialized: PathPoint = serde_json::from_str(&json).unwrap();
        assert_eq!(p, deserialized);
    }
}
