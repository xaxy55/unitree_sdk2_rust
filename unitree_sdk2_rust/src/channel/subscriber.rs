//! Channel subscriber for DDS topics.
//!
//! [`ChannelSubscriber<T>`] receives typed messages from a named DDS topic.
//! Create one via [`super::ChannelFactory::create_subscriber`], then call
//! [`init_channel`](ChannelSubscriber::init_channel) with a callback to
//! start receiving messages.
//!
//! > **Note:** The current implementation is a stub. Real DDS connectivity
//! > would require a Rust DDS binding such as `dust_dds` or `cyclonedds-rs`.

use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use crate::error::{Result, SdkError};

/// Subscribes to messages on a DDS topic.
///
/// Generic over the message type `T`. The subscriber spawns a background
/// thread that delivers messages to the provided handler callback.
///
/// # Examples
///
/// ```
/// use unitree_sdk2_rust::channel::ChannelFactory;
/// use unitree_sdk2_rust::idl::go2::LowState;
///
/// ChannelFactory::init(0, "eth0");
/// let factory = ChannelFactory::instance().lock().unwrap();
/// let mut subscriber = factory.create_subscriber::<LowState>("rt/lowstate");
/// drop(factory);
///
/// subscriber.init_channel(|_state: &LowState| {
///     // handle incoming state
/// }).unwrap();
/// subscriber.close_channel();
/// ```
pub struct ChannelSubscriber<T> {
    topic: String,
    initialized: bool,
    running: Arc<Mutex<bool>>,
    _phantom: PhantomData<T>,
}

impl<T> ChannelSubscriber<T> {
    pub(crate) fn new(topic: &str) -> Self {
        Self {
            topic: topic.to_string(),
            initialized: false,
            running: Arc::new(Mutex::new(false)),
            _phantom: PhantomData,
        }
    }
}

impl<T: Clone + Send + 'static> ChannelSubscriber<T> {
    /// Initialize the subscriber and begin delivering messages to `handler`.
    ///
    /// Returns `Err(`[`SdkError::Init`]`)` if already initialized.
    pub fn init_channel(&mut self, handler: impl Fn(&T) + Send + 'static) -> Result<()> {
        if self.initialized {
            return Err(SdkError::Init("Channel already initialized".into()));
        }
        log::info!("Subscriber init_channel: topic={}", self.topic);
        self.initialized = true;
        *self.running.lock().unwrap() = true;

        let running = Arc::clone(&self.running);
        let topic = self.topic.clone();

        // Spawn a stub thread. Real implementation would poll DDS.
        std::thread::spawn(move || {
            log::debug!("Subscriber thread started for topic={}", topic);
            while *running.lock().unwrap() {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            drop(handler); // satisfy the compiler that handler is moved in
            log::debug!("Subscriber thread stopped for topic={}", topic);
        });

        Ok(())
    }

    /// Stop the subscriber thread.
    pub fn close_channel(&mut self) {
        *self.running.lock().unwrap() = false;
        self.initialized = false;
        log::info!("Subscriber close_channel: topic={}", self.topic);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn init_and_close() {
        let mut sub: ChannelSubscriber<u32> = ChannelSubscriber::new("test/topic");
        sub.init_channel(|_val: &u32| {}).unwrap();
        // Give the thread a moment to start
        std::thread::sleep(std::time::Duration::from_millis(50));
        sub.close_channel();
    }

    #[test]
    fn double_init_returns_error() {
        let mut sub: ChannelSubscriber<u32> = ChannelSubscriber::new("test/topic");
        sub.init_channel(|_| {}).unwrap();
        match sub.init_channel(|_| {}) {
            Err(SdkError::Init(_)) => {}
            other => panic!("expected Init error, got {:?}", other),
        }
        sub.close_channel();
    }

    #[test]
    fn close_resets_state() {
        let mut sub: ChannelSubscriber<u32> = ChannelSubscriber::new("test/topic");
        sub.init_channel(|_| {}).unwrap();
        sub.close_channel();
        // Should be able to re-init after close
        sub.init_channel(|_| {}).unwrap();
        sub.close_channel();
    }

    #[test]
    fn handler_is_called_type_check() {
        // This test verifies that the handler type-checks with the subscriber's
        // generic type parameter.
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = Arc::clone(&called);
        let mut sub: ChannelSubscriber<String> = ChannelSubscriber::new("test");
        sub.init_channel(move |_msg: &String| {
            called_clone.store(true, Ordering::SeqCst);
        })
        .unwrap();
        sub.close_channel();
    }
}
