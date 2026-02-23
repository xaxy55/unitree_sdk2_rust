//! Go2 IDL messages.
pub mod bms_cmd;
pub mod bms_state;
pub mod imu_state;
pub mod low_cmd;
pub mod low_state;
pub mod motor_cmd;
pub mod motor_state;
pub mod path_point;
pub mod sport_mode_state;
pub mod time_spec;
pub mod wireless_controller;

pub use bms_cmd::BmsCmd;
pub use bms_state::BmsState;
pub use imu_state::IMUState;
pub use low_cmd::LowCmd;
pub use low_state::LowState;
pub use motor_cmd::MotorCmd;
pub use motor_state::MotorState;
pub use path_point::PathPoint;
pub use sport_mode_state::SportModeState;
pub use time_spec::TimeSpec;
pub use wireless_controller::WirelessController;
