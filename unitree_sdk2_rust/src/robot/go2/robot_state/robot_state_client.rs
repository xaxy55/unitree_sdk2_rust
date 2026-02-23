//! RobotStateClient - manages robot service states.
//!
//! TODO: Real implementation would use DDS RPC over CycloneDDS.

use super::robot_state_api::*;

/// Describes a robot service with its name, status, and protection flag.
#[derive(Debug, Clone)]
pub struct ServiceState {
    pub name: String,
    pub status: i32,
    pub protect: i32,
}

/// Client for querying and controlling robot service states.
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
    pub fn service_list(&self) -> Result<Vec<ServiceState>, i32> {
        log::info!("RobotStateClient service_list: api_id={}", ROBOT_STATE_API_ID_SERVICE_LIST);
        // Stub: return an empty list
        Ok(vec![])
    }

    /// Switch a service on or off.
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
