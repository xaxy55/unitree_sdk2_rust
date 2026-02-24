//! IMU (Inertial Measurement Unit) state message.
//!
//! Contains the [`IMUState`] struct with orientation, angular velocity,
//! linear acceleration, and temperature data from the robot's IMU sensor.

use serde::{Deserialize, Serialize};

/// IMU sensor state.
///
/// Reports orientation as both a quaternion and roll/pitch/yaw angles,
/// along with gyroscope and accelerometer readings.
///
/// # Examples
///
/// ```
/// use unitree_sdk2_rust::idl::go2::IMUState;
///
/// let imu = IMUState::default();
/// assert_eq!(imu.quaternion, [0.0; 4]);
/// assert_eq!(imu.temperature, 0);
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct IMUState {
    /// Orientation quaternion `[w, x, y, z]`.
    pub quaternion: [f32; 4],
    /// Angular velocity `[x, y, z]` in rad/s.
    pub gyroscope: [f32; 3],
    /// Linear acceleration `[x, y, z]` in m/s^2.
    pub accelerometer: [f32; 3],
    /// Roll, pitch, yaw in radians.
    pub rpy: [f32; 3],
    /// Sensor temperature in degrees Celsius.
    pub temperature: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values() {
        let imu = IMUState::default();
        assert_eq!(imu.quaternion, [0.0; 4]);
        assert_eq!(imu.gyroscope, [0.0; 3]);
        assert_eq!(imu.accelerometer, [0.0; 3]);
        assert_eq!(imu.rpy, [0.0; 3]);
        assert_eq!(imu.temperature, 0);
    }

    #[test]
    fn serde_roundtrip() {
        let imu = IMUState {
            quaternion: [1.0, 0.0, 0.0, 0.0],
            gyroscope: [0.1, 0.2, 0.3],
            accelerometer: [0.0, 0.0, 9.81],
            rpy: [0.01, 0.02, 0.03],
            temperature: 35,
        };
        let json = serde_json::to_string(&imu).unwrap();
        let deserialized: IMUState = serde_json::from_str(&json).unwrap();
        assert_eq!(imu, deserialized);
    }

    #[test]
    fn clone_and_eq() {
        let imu = IMUState {
            quaternion: [1.0, 0.0, 0.0, 0.0],
            ..Default::default()
        };
        assert_eq!(imu, imu.clone());
    }
}
