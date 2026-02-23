# unitree_sdk2
Unitree robot SDK version 2.

This repository contains two SDK implementations:

| Directory | Language | Status |
|-----------|----------|--------|
| `old_py/` | C++ (original) | Legacy |
| `unitree_sdk2_rust/` | Rust (new) | Active |

---

## Rust SDK (`unitree_sdk2_rust/`)

A new idiomatic Rust SDK for Unitree robots (Go2 and compatible).

### Features
- IDL message types (`LowCmd`, `LowState`, `SportModeState`, `WirelessController`, and more)
- Channel abstraction (`ChannelPublisher<T>`, `ChannelSubscriber<T>`, `ChannelFactory`)
- `SportClient` — full sport/locomotion mode API (40+ commands)
- `RobotStateClient` — service management API
- Examples: `go2_sport_client`, `go2_low_level`

### Prerequisites
- Rust 1.75 or higher (`rustup` recommended)

### Build

```bash
cd unitree_sdk2_rust
cargo build
```

### Run examples

```bash
cd unitree_sdk2_rust
cargo run --example go2_sport_client
cargo run --example go2_low_level
```

### Usage

```rust
use unitree_sdk2_rust::channel::ChannelFactory;
use unitree_sdk2_rust::robot::go2::sport::SportClient;

fn main() {
    ChannelFactory::init(0, "eth0");
    let mut client = SportClient::new(false);
    client.init();
    client.stand_up();
    client.move_cmd(0.5, 0.0, 0.0);
    client.stop_move();
}
```

> **Note:** The channel layer is currently a stub. Real DDS connectivity requires a
> Rust DDS binding such as [`dust_dds`](https://crates.io/crates/dust_dds) or
> [`cyclonedds-rs`](https://crates.io/crates/cyclonedds-rs).

---

## Legacy C++ SDK (`old_py/`)

The original C++ SDK. Kept for reference.

### Prebuild environment
* OS: Ubuntu 20.04 LTS
* CPU: aarch64 or x86_64
* Compiler: gcc 9.4.0

### Environment Setup

```bash
apt-get update
apt-get install -y cmake g++ build-essential libyaml-cpp-dev libeigen3-dev libboost-all-dev libspdlog-dev libfmt-dev
```

### Build examples

```bash
cd old_py
mkdir build && cd build
cmake ..
make
```

### Installation

```bash
cd old_py
mkdir build && cd build
cmake ..
sudo make install
```

### Notice
For more reference information, please go to [Unitree Document Center](https://support.unitree.com/home/zh/developer).
