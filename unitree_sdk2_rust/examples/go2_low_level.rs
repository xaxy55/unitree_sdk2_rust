//! Example: Low-level channel publisher and subscriber for Go2.
//!
//! Demonstrates how to use ChannelPublisher<LowCmd> and
//! ChannelSubscriber<LowState> to send motor commands and receive robot state.

use std::time::Duration;
use unitree_sdk2_rust::channel::{ChannelFactory, ChannelPublisher, ChannelSubscriber};
use unitree_sdk2_rust::idl::go2::{LowCmd, LowState};

fn main() {
    env_logger::init();

    // Initialize the channel factory.
    ChannelFactory::init(0, "eth0");

    let factory = ChannelFactory::instance().lock().unwrap();

    // Create publisher and subscriber.
    let mut publisher: ChannelPublisher<LowCmd> =
        factory.create_publisher("rt/lowcmd");
    let mut subscriber: ChannelSubscriber<LowState> =
        factory.create_subscriber("rt/lowstate");
    drop(factory);

    // Initialize channels.
    publisher.init_channel().expect("Publisher init failed");

    subscriber
        .init_channel(|state: &LowState| {
            println!(
                "Received LowState: imu_temp={} tick={}",
                state.imu_state.temperature, state.tick
            );
        })
        .expect("Subscriber init failed");

    // Publish a default LowCmd.
    let cmd = LowCmd::default();
    match publisher.write(&cmd) {
        Ok(true) => println!("LowCmd published successfully"),
        Ok(false) => println!("LowCmd publish returned false"),
        Err(e) => println!("LowCmd publish error: {e}"),
    }

    // Give the subscriber thread a moment to run.
    std::thread::sleep(Duration::from_millis(300));

    subscriber.close_channel();
    println!("Example complete.");
}
