//! Robot state service API constants.
//!
//! These constants define the service name, version, and RPC method IDs
//! for the robot state management service.

/// Robot state service name.
pub const ROBOT_STATE_SERVICE_NAME: &str = "robot_state";
/// Robot state API version string.
pub const ROBOT_STATE_API_VERSION: &str = "1.0.0.1";

/// List all registered services.
pub const ROBOT_STATE_API_ID_SERVICE_LIST: i32 = 1;
/// Start or stop a named service.
pub const ROBOT_STATE_API_ID_SERVICE_SWITCH: i32 = 2;
/// Set the telemetry report frequency.
pub const ROBOT_STATE_API_ID_SET_REPORT_FREQ: i32 = 3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_constants() {
        assert_eq!(ROBOT_STATE_SERVICE_NAME, "robot_state");
        assert_eq!(ROBOT_STATE_API_VERSION, "1.0.0.1");
    }

    #[test]
    fn api_ids_are_unique() {
        let ids = [
            ROBOT_STATE_API_ID_SERVICE_LIST,
            ROBOT_STATE_API_ID_SERVICE_SWITCH,
            ROBOT_STATE_API_ID_SET_REPORT_FREQ,
        ];
        let mut sorted = ids.to_vec();
        sorted.sort();
        sorted.dedup();
        assert_eq!(ids.len(), sorted.len());
    }
}
