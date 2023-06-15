extern crate redis;
use async_trait::async_trait;
use redis::AsyncCommands;
use std::ops;

use crate::backend::{DequeuBackend, EnqueuBackend};

pub struct RedisBackendConfig<S: ToString> {
    pub queue_key: S,
    pub read_batch_size: usize,
}

impl<S: ToString> ops::Add<redis::Client> for RedisBackendConfig<S> {
    type Output = RedisBackend;

    fn add(self, client: redis::Client) -> RedisBackend {
        RedisBackend::new(client, self.queue_key.to_string(), self.read_batch_size)
    }
}

#[derive(Clone)]
pub struct RedisBackend {
    client: redis::Client,
    queue_key: String,
    pop_schedule_script: redis::Script,
    read_batch_size: usize,
}

impl RedisBackend {
    pub fn new(client: redis::Client, queue_key: String, read_batch_size: usize) -> Self {
        Self {
            client,
            queue_key,
            pop_schedule_script: redis::Script::new(
                r"
                local key = KEYS[1]
                local unix_ts = ARGV[1]
                local limit = ARGV[2]
                local res = redis.call('zrange', key, '-inf', unix_ts, 'byscore', 'limit', 0, limit)
                for _, raw in ipairs(res) do
                    redis.call('zrem', key, raw)
                end
                return res",
            ),
            read_batch_size,
        }
    }
}

#[async_trait]
impl DequeuBackend<String, f64> for RedisBackend {
    async fn dequeue(&self, time: f64) -> Vec<String> {
        let mut con = self.client.get_async_connection().await.unwrap();
        let result: Vec<String> = self
            .pop_schedule_script
            .key(self.queue_key.as_str())
            .arg(time)
            .arg(self.read_batch_size)
            .invoke_async(&mut con)
            .await
            .unwrap();

        return result;
    }
}

#[async_trait]
impl EnqueuBackend<String, f64> for RedisBackend {
    async fn enqueue(&self, task: String, time: f64) {
        let mut con = self.client.get_async_connection().await.unwrap();
        let _: () = con.zadd(self.queue_key.as_str(), task, time).await.unwrap();
    }
}
