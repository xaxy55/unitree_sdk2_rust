//! Channel factory - global singleton for creating publishers and subscribers.
//!
//! TODO: Real DDS connectivity would require a DDS Rust binding such as
//! `dust_dds` or `cyclonedds-rs`. This is a stub simulation.

use std::sync::{Mutex, OnceLock};
use crate::channel::{ChannelPublisher, ChannelSubscriber};

static INSTANCE: OnceLock<Mutex<ChannelFactory>> = OnceLock::new();

/// Global factory for creating DDS channels.
pub struct ChannelFactory {
    pub domain_id: i32,
    pub network_interface: String,
}

impl ChannelFactory {
    /// Get the global singleton instance.
    pub fn instance() -> &'static Mutex<ChannelFactory> {
        INSTANCE.get_or_init(|| {
            Mutex::new(ChannelFactory {
                domain_id: 0,
                network_interface: String::new(),
            })
        })
    }

    /// Initialize the factory with a domain ID and network interface.
    pub fn init(domain_id: i32, network_interface: &str) {
        let mut factory = Self::instance().lock().unwrap();
        factory.domain_id = domain_id;
        factory.network_interface = network_interface.to_string();
        log::info!(
            "ChannelFactory initialized: domain_id={}, interface={}",
            domain_id,
            network_interface
        );
    }

    /// Create a publisher for the given topic.
    pub fn create_publisher<T>(&self, topic: &str) -> ChannelPublisher<T> {
        ChannelPublisher::new(topic)
    }

    /// Create a subscriber for the given topic.
    pub fn create_subscriber<T>(&self, topic: &str) -> ChannelSubscriber<T> {
        ChannelSubscriber::new(topic)
    }
}
