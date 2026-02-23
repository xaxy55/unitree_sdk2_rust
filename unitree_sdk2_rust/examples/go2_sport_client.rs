//! Example: Control the Go2 robot using SportClient.
//!
//! This example demonstrates how to initialize the channel factory and
//! use SportClient to send sport commands to the robot.

use unitree_sdk2_rust::channel::ChannelFactory;
use unitree_sdk2_rust::robot::go2::sport::SportClient;

fn main() {
    env_logger::init();

    // Initialize the channel factory with domain 0 and a network interface.
    ChannelFactory::init(0, "eth0");

    // Create and initialize the sport client.
    let mut client = SportClient::new(false);
    client.init();

    println!("Sending stand_up command: ret={}", client.stand_up());
    println!("Sending balance_stand command: ret={}", client.balance_stand());
    println!("Sending move_cmd (0.5, 0.0, 0.0): ret={}", client.move_cmd(0.5, 0.0, 0.0));
    println!("Sending stop_move command: ret={}", client.stop_move());
    println!("Sending stand_down command: ret={}", client.stand_down());
    println!("Sending damp command: ret={}", client.damp());

    match client.auto_recover_get() {
        Ok(flag) => println!("auto_recover_get: {flag}"),
        Err(code) => println!("auto_recover_get error: {code}"),
    }
}
