extern crate redis;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::backend::{DequeuBackend, EnqueuBackend};
use crate::backends::redis::RedisBackend;

#[derive(Debug)]
pub enum JsonRedisError {
    Redis(redis::RedisError),
    Serde(serde_json::Error),
}

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

#[async_trait]
impl<T> EnqueuBackend<T, f64, JsonRedisError> for JsonRedisBackend<T>
where
    T: Serialize + Send + Sync,
{
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
    async fn dequeue(&self, score: f64) -> Result<Vec<serde_json::Result<T>>, redis::RedisError> {
        let data = self.backend.dequeue(score).await?;
        Ok(data.into_iter().map(|d| serde_json::from_str(&d)).collect())
    }
}
