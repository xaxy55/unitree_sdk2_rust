# Roadmap

This document tracks the current status of the Rust SDK and the planned work ahead.

---

## Legend

| Symbol | Meaning |
|--------|---------|
| ✅ | Completed / available |
| 🚧 | In progress |
| 🗓️ | Planned |
| 💡 | Idea / under consideration |

---

## Current Status

### Core Infrastructure

| Feature | Status | Notes |
|---------|--------|-------|
| IDL message types (`LowCmd`, `LowState`, `MotorCmd`, `MotorState`, `ImuState`, `BmsCmd`, `BmsState`, `SportModeState`, `WirelessController`, `PathPoint`, `TimeSpec`) | ✅ | Serde-serialisable structs |
| `ChannelPublisher<T>` | ✅ | Stub — API shape finalised |
| `ChannelSubscriber<T>` | ✅ | Stub — API shape finalised |
| `ChannelFactory` (init, create pub/sub) | ✅ | Stub — API shape finalised |
| Error types (`SdkError`) | ✅ | `thiserror`-based |
| `cargo build` / `cargo test` CI | ✅ | Makefile targets wired up |

### Robot Clients

| Feature | Status | Notes |
|---------|--------|-------|
| `SportClient` (40+ sport-mode commands) | ✅ | Stub — logs calls, returns 0 |
| `RobotStateClient` (service management) | ✅ | Stub — logs calls |
| Low-level motor control example (`go2_low_level`) | ✅ | Demonstrates `LowCmd` usage |
| Sport-mode example (`go2_sport_client`) | ✅ | Demonstrates `SportClient` |

---

## Roadmap

### v0.2 — Real DDS Transport

| Feature | Status | Priority |
|---------|--------|----------|
| Integrate a Rust DDS binding ([`dust_dds`](https://crates.io/crates/dust_dds) or [`cyclonedds-rs`](https://crates.io/crates/cyclonedds-rs)) | 🗓️ | High |
| Wire `ChannelFactory` to a real DDS participant | 🗓️ | High |
| Implement `ChannelPublisher::publish` over DDS | 🗓️ | High |
| Implement `ChannelSubscriber::on_data` / async receive over DDS | 🗓️ | High |
| Replace `SportClient` stubs with real DDS-RPC calls | 🗓️ | High |
| Replace `RobotStateClient` stubs with real DDS-RPC calls | 🗓️ | High |
| Network interface auto-detection | 🗓️ | Medium |

### v0.3 — Async & Ergonomics

| Feature | Status | Priority |
|---------|--------|----------|
| `async`/`await` API for subscribers (tokio or async-std) | 🗓️ | High |
| `async` RPC calls in `SportClient` / `RobotStateClient` | 🗓️ | High |
| Builder pattern for client configuration | 🗓️ | Medium |
| Result-returning API throughout (remove bare `i32` return codes) | 🗓️ | Medium |
| Callback-based subscriber API | 🗓️ | Medium |

### v0.4 — Extended Robot & Sensor Support

| Feature | Status | Priority |
|---------|--------|----------|
| H1 / G1 humanoid robot clients | 🗓️ | Medium |
| B2 quadruped robot client | 🗓️ | Medium |
| Additional IDL types (arm, gripper, lidar, depth camera) | 🗓️ | Medium |
| Video / image stream subscriber | 🗓️ | Low |
| Odometry and localisation helpers | 🗓️ | Low |

### v0.5 — Safety & Reliability

| Feature | Status | Priority |
|---------|--------|----------|
| Lease management (hardware safety interlock) | 🗓️ | High |
| Watchdog / heartbeat mechanism | 🗓️ | High |
| Reconnection logic on DDS participant loss | 🗓️ | Medium |
| Rate-limiter for high-frequency command topics | 🗓️ | Medium |

### Future Ideas

| Idea | Status |
|------|--------|
| ROS 2 bridge (publish/subscribe to ROS 2 topics) | 💡 |
| Python bindings via PyO3 | 💡 |
| C FFI layer for embedding in other languages | 💡 |
| Simulation back-end (Gazebo / Mujoco) | 💡 |
| CLI tool (`unitree-cli`) for quick robot inspection | 💡 |
| `no_std` / embedded-friendly build feature flag | 💡 |

---

## Contributing

Contributions are welcome!
Pick any `🗓️ Planned` item, open an issue to discuss the approach, then submit a pull request.
See the [README](README.md) for build and test instructions.
