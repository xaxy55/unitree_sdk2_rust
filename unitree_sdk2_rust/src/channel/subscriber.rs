//! Channel subscriber for DDS topics.
//!
//! TODO: Real DDS connectivity would require a DDS Rust binding such as
//! `dust_dds` or `cyclonedds-rs`. This is a stub simulation.

use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use crate::error::{Result, SdkError};

/// Subscribes to messages on a DDS topic.
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
