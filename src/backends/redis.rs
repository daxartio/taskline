extern crate redis;
use async_trait::async_trait;
use redis::{AsyncCommands, RedisError};
use std::ops;

use crate::backend::{DequeuBackend, EnqueuBackend};

/// Configuration for Redis backend.
/// You can use it to create `RedisBackend` instance.
pub struct RedisBackendConfig<S: ToString> {
    /// Redis key used to store tasks.
    pub queue_key: S,
    /// Number of tasks to read in one batch.
    pub read_batch_size: usize,
    /// If `true`, tasks will be deleted from queue after reading.
    /// If autodelete is `false`, tasks should be deleted explicitly from queue after reading with `RedisBackend::delete`.
    pub autodelete: bool,
}

impl<S: ToString> RedisBackendConfig<S> {
    /// Create `RedisBackendConfig` instance.
    pub fn with_client(self, client: redis::Client) -> RedisBackend {
        RedisBackend::new(
            client,
            self.queue_key.to_string(),
            self.read_batch_size,
            self.autodelete,
        )
    }
}

impl<S: ToString> ops::Add<redis::Client> for RedisBackendConfig<S> {
    type Output = RedisBackend;

    /// Create `RedisBackend` instance.
    /// It requires `redis::Client` instance.
    fn add(self, client: redis::Client) -> RedisBackend {
        self.with_client(client)
    }
}

/// Redis backend.
/// It implements both `DequeuBackend` and `EnqueuBackend` traits.
/// You can use score to sort tasks in queue. Usually it is unix timestamp.
#[derive(Clone)]
pub struct RedisBackend {
    client: redis::Client,
    queue_key: String,
    pop_schedule_script: redis::Script,
    read_batch_size: usize,
    autodelete: bool,
}

impl RedisBackend {
    /// Create new instance of `RedisBackend`.
    ///
    /// It requires `redis::Client` instance, redis key used to store tasks and number of tasks to read in one batch.
    /// It also creates lua script used to pop tasks from redis.
    /// * `client` - redis client.
    /// * `queue_key` - redis key used to store tasks.
    /// * `read_batch_size` - number of tasks to read in one batch.
    /// * `autodelete` - if `true`, tasks will be deleted from queue after reading. If `false`, tasks should be deleted explicitly from queue after reading with `RedisBackend::delete`.
    pub fn new(
        client: redis::Client,
        queue_key: String,
        read_batch_size: usize,
        autodelete: bool,
    ) -> Self {
        Self {
            client,
            queue_key,
            pop_schedule_script: redis::Script::new(
                r"
                local key = KEYS[1]
                local unix_ts = ARGV[1]
                local limit = ARGV[2]
                local autodelete = ARGV[3] == '1'
                local res = redis.call('zrange', key, '-inf', unix_ts, 'byscore', 'limit', 0, limit)
                if autodelete then
                    for _, raw in ipairs(res) do
                        redis.call('zrem', key, raw)
                    end
                end
                return res",
            ),
            read_batch_size,
            autodelete,
        }
    }
}

impl RedisBackend {
    /// Delete task from queue.
    pub async fn delete(&self, task: String) -> Result<(), RedisError> {
        let mut con = match self.client.get_async_connection().await {
            Ok(con) => con,
            Err(e) => return Err(e),
        };

        con.zrem(self.queue_key.as_str(), task).await
    }
}

#[async_trait]
impl DequeuBackend<String, f64, RedisError> for RedisBackend {
    /// Calls lua script to pop tasks from redis.
    /// If there are no tasks in queue it returns empty vector.
    /// If there are no tasks with score less than `score`, returns empty vector.
    async fn dequeue(&self, score: f64) -> Result<Vec<String>, RedisError> {
        let mut con = match self.client.get_async_connection().await {
            Ok(con) => con,
            Err(e) => return Err(e),
        };

        let result: Vec<String> = match self
            .pop_schedule_script
            .key(self.queue_key.as_str())
            .arg(score)
            .arg(self.read_batch_size)
            .arg(self.autodelete as u8)
            .invoke_async(&mut con)
            .await
        {
            Ok(result) => result,
            Err(e) => return Err(e),
        };

        Ok(result)
    }
}

#[async_trait]
impl EnqueuBackend<String, f64, RedisError> for RedisBackend {
    /// Adds a task to redis.
    /// It uses score to sort tasks in queue. Usually it is unix timestamp.
    async fn enqueue(&self, task: String, score: f64) -> Result<(), RedisError> {
        let mut con = match self.client.get_async_connection().await {
            Ok(con) => con,
            Err(e) => return Err(e),
        };
        con.zadd(self.queue_key.as_str(), task, score).await
    }
}
