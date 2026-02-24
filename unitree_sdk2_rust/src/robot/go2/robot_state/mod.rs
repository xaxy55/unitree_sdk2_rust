//! Robot state client for Go2.
//!
//! Provides [`RobotStateClient`] for querying and managing the robot's
//! internal services, and [`ServiceState`] describing each service.

pub mod robot_state_api;
pub mod robot_state_client;

pub use robot_state_client::{RobotStateClient, ServiceState};
