//! Sport mode client for Go2.
//!
//! Provides [`SportClient`] with 40+ methods for controlling the robot's
//! locomotion, gestures, tricks, and gait modes.

pub mod sport_api;
pub mod sport_client;

pub use sport_client::SportClient;
