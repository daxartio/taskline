//! Backends which can be used to store data.
//! The backends must implement `DequeuBackend` and `EnqueuBackend` traits.
//! They can be used in Consumer and Producer.
pub mod redis;
pub mod redis_json;
