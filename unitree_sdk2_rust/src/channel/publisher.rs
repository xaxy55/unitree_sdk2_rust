//! Channel publisher for DDS topics.
//!
//! [`ChannelPublisher<T>`] sends typed messages to a named DDS topic.
//! Create one via [`super::ChannelFactory::create_publisher`], call
//! [`init_channel`](ChannelPublisher::init_channel) once, then
//! [`write`](ChannelPublisher::write) to publish messages.
//!
//! > **Note:** The current implementation is a stub. Real DDS connectivity
//! > would require a Rust DDS binding such as `dust_dds` or `cyclonedds-rs`.

use std::marker::PhantomData;
use crate::error::{Result, SdkError};

/// Publishes messages to a DDS topic.
///
/// Generic over the message type `T`. The publisher must be initialized
/// with [`init_channel`](Self::init_channel) before writing.
///
/// # Examples
///
/// ```
/// use unitree_sdk2_rust::channel::ChannelFactory;
/// use unitree_sdk2_rust::idl::go2::LowCmd;
///
/// ChannelFactory::init(0, "eth0");
/// let factory = ChannelFactory::instance().lock().unwrap();
/// let mut publisher = factory.create_publisher::<LowCmd>("rt/lowcmd");
/// drop(factory);
///
/// publisher.init_channel().unwrap();
/// let cmd = LowCmd::default();
/// assert!(publisher.write(&cmd).unwrap());
/// ```
pub struct ChannelPublisher<T> {
    topic: String,
    initialized: bool,
    _phantom: PhantomData<T>,
}

impl<T> ChannelPublisher<T> {
    pub(crate) fn new(topic: &str) -> Self {
        Self {
            topic: topic.to_string(),
            initialized: false,
            _phantom: PhantomData,
        }
    }

    /// Initialize the publisher channel.
    ///
    /// Must be called before [`write`](Self::write).
    pub fn init_channel(&mut self) -> Result<()> {
        log::info!("Publisher init_channel: topic={}", self.topic);
        self.initialized = true;
        Ok(())
    }

    /// Write a message to the topic.
    ///
    /// Returns `Ok(true)` on success. Returns
    /// `Err(`[`SdkError::NotInitialized`]`)` if [`init_channel`](Self::init_channel)
    /// has not been called.
    pub fn write(&self, _msg: &T) -> Result<bool> {
        if !self.initialized {
            return Err(SdkError::NotInitialized);
        }
        log::debug!("Publisher write: topic={}", self.topic);
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_not_initialized() {
        let pub_: ChannelPublisher<u32> = ChannelPublisher::new("test");
        assert!(pub_.write(&42).is_err());
    }

    #[test]
    fn init_then_write_succeeds() {
        let mut pub_: ChannelPublisher<u32> = ChannelPublisher::new("test");
        pub_.init_channel().unwrap();
        assert_eq!(pub_.write(&42).unwrap(), true);
    }

    #[test]
    fn write_before_init_returns_not_initialized() {
        let pub_: ChannelPublisher<String> = ChannelPublisher::new("test");
        match pub_.write(&"hello".to_string()) {
            Err(SdkError::NotInitialized) => {}
            other => panic!("expected NotInitialized, got {:?}", other),
        }
    }
}
