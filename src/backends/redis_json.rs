use serde::{de::DeserializeOwned, Serialize};

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

    /// Check redis version.
    pub async fn check_version(&self) -> Result<bool, redis::RedisError> {
        self.backend.check_version().await
    }
}

impl<T> JsonRedisBackend<T>
where
    T: Serialize + Send + Sync,
{
    /// Serializes data to JSON and calls `RedisBackend::write`.
    pub async fn write(&self, data: &T, score: &f64) -> Result<(), JsonRedisError> {
        let data = match serde_json::to_string(&data) {
            Ok(data) => data,
            Err(e) => return Err(JsonRedisError::Serde(e)),
        };
        match self.backend.write(&data, score).await {
            Ok(_) => Ok(()),
            Err(e) => Err(JsonRedisError::Redis(e)),
        }
    }

    /// Delete task from queue.
    pub async fn delete(&self, data: &T) -> Result<(), JsonRedisError> {
        if self.backend.autodelete {
            return Ok(());
        }
        let data = match serde_json::to_string(&data) {
            Ok(data) => data,
            Err(e) => return Err(JsonRedisError::Serde(e)),
        };
        match self.backend.delete(&data).await {
            Ok(_) => Ok(()),
            Err(e) => Err(JsonRedisError::Redis(e)),
        }
    }
}

impl<T> JsonRedisBackend<T>
where
    T: DeserializeOwned + Send + Sync,
{
    /// Returns `Vec<serde_json::Result<T>>` because it is possible that some tasks will be corrupted.
    pub async fn read(&self, score: &f64) -> Result<Vec<serde_json::Result<T>>, redis::RedisError> {
        let data = self.backend.read(score).await?;
        Ok(data.into_iter().map(|d| serde_json::from_str(&d)).collect())
    }
}
