//! DDS channel abstraction for pub/sub communication.
//!
//! This module provides a typed publish/subscribe layer modeled after DDS:
//!
//! - [`ChannelFactory`] — global singleton; call [`ChannelFactory::init`] at startup.
//! - [`ChannelPublisher<T>`] — publishes messages of type `T` to a named topic.
//! - [`ChannelSubscriber<T>`] — receives messages of type `T` via a callback.

pub mod factory;
pub mod publisher;
pub mod subscriber;

pub use factory::ChannelFactory;
pub use publisher::ChannelPublisher;
pub use subscriber::ChannelSubscriber;
