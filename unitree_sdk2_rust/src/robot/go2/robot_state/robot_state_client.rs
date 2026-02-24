//! RobotStateClient — manages robot service states.
//!
//! Provides methods to list, enable/disable, and configure reporting
//! for the robot's internal services.
//!
//! > **Note:** Real implementation would use DDS RPC over CycloneDDS.

use super::robot_state_api::*;

/// Describes a robot service with its name, status, and protection flag.
///
/// Returned by [`RobotStateClient::service_list`].
#[derive(Debug, Clone)]
pub struct ServiceState {
    /// Service name (e.g. `"sport"`, `"obstacle_avoidance"`).
    pub name: String,
    /// Service status (0 = stopped, 1 = running).
    pub status: i32,
    /// Protection flag (1 = protected, cannot be stopped remotely).
    pub protect: i32,
}

/// Client for querying and controlling robot service states.
///
/// # Examples
///
/// ```
/// use unitree_sdk2_rust::robot::go2::robot_state::RobotStateClient;
///
/// let mut client = RobotStateClient::new();
/// client.init();
///
/// let services = client.service_list().unwrap();
/// assert!(services.is_empty()); // stub returns empty list
///
/// assert_eq!(client.set_report_freq(100, 1000), 0);
/// ```
pub struct RobotStateClient {
    timeout: f32,
}

impl RobotStateClient {
    /// Create a new `RobotStateClient`.
    pub fn new() -> Self {
        Self { timeout: 10.0 }
    }

    /// Initialize the client.
    pub fn init(&mut self) {
        log::info!(
            "RobotStateClient init: service={} version={}",
            ROBOT_STATE_SERVICE_NAME,
            ROBOT_STATE_API_VERSION
        );
    }

    /// List all services on the robot.
    ///
    /// Returns `Ok(Vec<ServiceState>)` on success, or `Err(error_code)`.
    pub fn service_list(&self) -> Result<Vec<ServiceState>, i32> {
        log::info!("RobotStateClient service_list: api_id={}", ROBOT_STATE_API_ID_SERVICE_LIST);
        // Stub: return an empty list
        Ok(vec![])
    }

    /// Switch a service on or off.
    ///
    /// - `name`: service name
    /// - `swit`: 1 = start, 0 = stop
    pub fn service_switch(&self, name: &str, swit: i32) -> Result<i32, i32> {
        log::info!(
            "RobotStateClient service_switch: api_id={} name={} switch={}",
            ROBOT_STATE_API_ID_SERVICE_SWITCH,
            name,
            swit
        );
        Ok(0)
    }

    /// Set report frequency.
    ///
    /// - `interval`: reporting interval in milliseconds
    /// - `duration`: total reporting duration in milliseconds
    pub fn set_report_freq(&self, interval: i32, duration: i32) -> i32 {
        log::info!(
            "RobotStateClient set_report_freq: api_id={} interval={} duration={}",
            ROBOT_STATE_API_ID_SET_REPORT_FREQ,
            interval,
            duration
        );
        let _ = self.timeout;
        0
    }
}

impl Default for RobotStateClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_and_default() {
        let a = RobotStateClient::new();
        let b = RobotStateClient::default();
        assert_eq!(a.timeout, b.timeout);
        assert_eq!(a.timeout, 10.0);
    }

    #[test]
    fn init_does_not_panic() {
        let mut client = RobotStateClient::new();
        client.init();
    }

    #[test]
    fn service_list_returns_empty() {
        let client = RobotStateClient::new();
        let list = client.service_list().unwrap();
        assert!(list.is_empty());
    }

    #[test]
    fn service_switch_returns_ok() {
        let client = RobotStateClient::new();
        assert_eq!(client.service_switch("sport", 1), Ok(0));
        assert_eq!(client.service_switch("sport", 0), Ok(0));
    }

    #[test]
    fn set_report_freq_returns_zero() {
        let client = RobotStateClient::new();
        assert_eq!(client.set_report_freq(100, 5000), 0);
    }

    #[test]
    fn service_state_debug_and_clone() {
        let state = ServiceState {
            name: "test_service".to_string(),
            status: 1,
            protect: 0,
        };
        let cloned = state.clone();
        assert_eq!(cloned.name, "test_service");
        assert_eq!(cloned.status, 1);
        assert_eq!(cloned.protect, 0);
        let debug = format!("{:?}", state);
        assert!(debug.contains("test_service"));
    }
}
