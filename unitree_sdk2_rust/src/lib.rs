//! # Unitree SDK2 Rust
//!
//! Idiomatic Rust SDK for controlling Unitree quadruped robots (Go2 and compatible).
//!
//! ## Architecture
//!
//! The SDK is organized into four top-level modules:
//!
//! - **[`idl`]** — Message type definitions (IDL structs) for all robot data:
//!   motor commands/states, IMU, BMS, sport-mode telemetry, and more.
//! - **[`channel`]** — DDS channel abstraction providing typed pub/sub
//!   communication via [`ChannelPublisher`](channel::ChannelPublisher) and
//!   [`ChannelSubscriber`](channel::ChannelSubscriber).
//! - **[`robot`]** — High-level client APIs:
//!   - [`SportClient`](robot::go2::sport::SportClient) — 40+ sport/locomotion commands
//!   - [`RobotStateClient`](robot::go2::robot_state::RobotStateClient) — service management
//! - **[`error`]** — SDK error types.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use unitree_sdk2_rust::channel::ChannelFactory;
//! use unitree_sdk2_rust::robot::go2::sport::SportClient;
//!
//! fn main() {
//!     ChannelFactory::init(0, "eth0");
//!     let mut client = SportClient::new(false);
//!     client.init();
//!     client.stand_up();
//!     client.move_cmd(0.5, 0.0, 0.0);
//!     client.stop_move();
//! }
//! ```
//!
//! > **Note:** The DDS channel layer is currently a stub. Real DDS connectivity
//! > requires a Rust DDS binding such as [`dust_dds`](https://crates.io/crates/dust_dds)
//! > or [`cyclonedds-rs`](https://crates.io/crates/cyclonedds-rs).

pub mod error;
pub mod idl;
pub mod channel;
pub mod robot;
