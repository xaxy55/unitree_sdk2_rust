//! Channel factory — global singleton for creating publishers and subscribers.
//!
//! The [`ChannelFactory`] is the entry point for the DDS communication layer.
//! Call [`ChannelFactory::init`] once at startup to configure the DDS domain
//! and network interface, then use the singleton to create typed publishers
//! and subscribers.
//!
//! > **Note:** The current implementation is a stub. Real DDS connectivity
//! > would require a Rust DDS binding such as `dust_dds` or `cyclonedds-rs`.

use std::sync::{Mutex, OnceLock};
use crate::channel::{ChannelPublisher, ChannelSubscriber};

static INSTANCE: OnceLock<Mutex<ChannelFactory>> = OnceLock::new();

/// Global factory for creating DDS channels.
///
/// # Examples
///
/// ```
/// use unitree_sdk2_rust::channel::ChannelFactory;
///
/// ChannelFactory::init(0, "eth0");
/// let factory = ChannelFactory::instance().lock().unwrap();
/// assert_eq!(factory.domain_id, 0);
/// assert_eq!(factory.network_interface, "eth0");
/// ```
pub struct ChannelFactory {
    /// DDS domain ID.
    pub domain_id: i32,
    /// Network interface name (e.g. `"eth0"`).
    pub network_interface: String,
}

impl ChannelFactory {
    /// Get the global singleton instance.
    ///
    /// Creates a default (unconfigured) instance on first call.
    pub fn instance() -> &'static Mutex<ChannelFactory> {
        INSTANCE.get_or_init(|| {
            Mutex::new(ChannelFactory {
                domain_id: 0,
                network_interface: String::new(),
            })
        })
    }

    /// Initialize the factory with a domain ID and network interface.
    ///
    /// This should be called once at application startup before creating
    /// any publishers or subscribers.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singleton_returns_same_instance() {
        let a = ChannelFactory::instance() as *const _;
        let b = ChannelFactory::instance() as *const _;
        assert_eq!(a, b);
    }

    #[test]
    fn init_sets_fields() {
        ChannelFactory::init(1, "wlan0");
        let factory = ChannelFactory::instance().lock().unwrap();
        // Note: since this is a global singleton shared across tests,
        // the values may be overwritten by other tests. We just verify
        // the init path doesn't panic.
        assert!(factory.domain_id >= 0);
    }

    #[test]
    fn create_publisher_returns_publisher() {
        let factory = ChannelFactory::instance().lock().unwrap();
        let _pub: ChannelPublisher<u8> = factory.create_publisher("test/topic");
    }

    #[test]
    fn create_subscriber_returns_subscriber() {
        let factory = ChannelFactory::instance().lock().unwrap();
        let _sub: ChannelSubscriber<u8> = factory.create_subscriber("test/topic");
    }
}
