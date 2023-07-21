extern crate redis;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::backend::{CommitBackend, DequeuBackend, EnqueuBackend};
use crate::backends::redis::RedisBackend;

/// Error type for `JsonRedisBackend`.
/// It wraps `redis::RedisError` and `serde_json::Error`.
#[derive(Debug)]
pub enum JsonRedisError {
    Redis(redis::RedisError),
    Serde(serde_json::Error),
}

/// Redis backend with JSON serialization.
/// It implements both `DequeuBackend` and `EnqueuBackend` traits.
/// It requires `serde::Serialize` and `serde::DeserializeOwned` traits for task type.
/// It overrides `RedisBackend` with `serde_json::to_string` and `serde_json::from_str` calls.
/// The logic of `RedisBackend` is not changed.
#[derive(Clone)]
pub struct JsonRedisBackend<T> {
    backend: RedisBackend,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> JsonRedisBackend<T> {
    pub fn new(backend: RedisBackend) -> Self {
        Self {
            backend,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> JsonRedisBackend<T>
where
    T: Serialize + Send + Sync,
{
    /// Delete task from queue.
    ///
    /// New in version 0.5.0.
    #[deprecated(since = "0.6.0", note = "please use `Committer::commit` instead")]
    pub async fn delete(&self, data: T) -> Result<(), JsonRedisError> {
        self.commit(data).await
    }

    /// Check redis version.
    ///
    /// New in version 0.6.0.
    pub async fn is_redis_version_ok(&self) -> Result<bool, redis::RedisError> {
        self.backend.is_redis_version_ok().await
    }
}

#[async_trait]
impl<T> CommitBackend<T, JsonRedisError> for JsonRedisBackend<T>
where
    T: Serialize + Send + Sync,
{
    /// Delete task from queue.
    ///
    /// New in version 0.5.1.
    async fn commit(&self, data: T) -> Result<(), JsonRedisError> {
        if self.backend.autodelete {
            return Ok(());
        }
        let data = match serde_json::to_string(&data) {
            Ok(data) => data,
            Err(e) => return Err(JsonRedisError::Serde(e)),
        };
        match self.backend.commit(data).await {
            Ok(_) => Ok(()),
            Err(e) => Err(JsonRedisError::Redis(e)),
        }
    }
}

#[async_trait]
impl<T> EnqueuBackend<T, f64, JsonRedisError> for JsonRedisBackend<T>
where
    T: Serialize + Send + Sync,
{
    /// Serializes data to JSON and calls `RedisBackend::enqueue`.
    async fn enqueue(&self, data: T, score: f64) -> Result<(), JsonRedisError> {
        let data = match serde_json::to_string(&data) {
            Ok(data) => data,
            Err(e) => return Err(JsonRedisError::Serde(e)),
        };
        match self.backend.enqueue(data, score).await {
            Ok(_) => Ok(()),
            Err(e) => Err(JsonRedisError::Redis(e)),
        }
    }
}

#[async_trait]
impl<T> DequeuBackend<serde_json::Result<T>, f64, redis::RedisError> for JsonRedisBackend<T>
where
    T: DeserializeOwned + Send + Sync,
{
    /// Returns `Vec<serde_json::Result<T>>` because it is possible that some tasks will be corrupted.
    async fn dequeue(&self, score: f64) -> Result<Vec<serde_json::Result<T>>, redis::RedisError> {
        let data = self.backend.dequeue(score).await?;
        Ok(data.into_iter().map(|d| serde_json::from_str(&d)).collect())
    }
}
