extern crate redis;
use async_trait::async_trait;
use redis::{AsyncCommands, IntoConnectionInfo};
use tokio::time::{sleep, Duration};

use crate::backend::{DequeuBackend, EnqueuBackend};
use crate::tasks::QueuedTask;

pub struct RedisBackend {
    client: redis::Client,
    queue_key: &'static str,
    pop_schedule_script: redis::Script,
    read_batch_size: usize,
    read_interval: Duration,
}

impl RedisBackend {
    pub fn new<T: IntoConnectionInfo>(params: T) -> RedisBackend {
        let pop_schedule_script = redis::Script::new(
            r"
            local key = KEYS[1]
            local unix_ts = ARGV[1]
            local limit = ARGV[2]
            local res = redis.call('zrange', key, '-inf', unix_ts, 'byscore', 'limit', 0, limit)
            for _, raw in ipairs(res) do
                redis.call('zrem', key, raw)
            end
            return res
        ",
        );
        RedisBackend {
            client: redis::Client::open(params).unwrap(),
            queue_key: "queue",
            pop_schedule_script: pop_schedule_script,
            read_batch_size: 50,
            read_interval: Duration::from_millis(1000),
        }
    }
}

impl Clone for RedisBackend {
    fn clone(&self) -> Self {
        RedisBackend {
            client: self.client.clone(),
            queue_key: self.queue_key,
            pop_schedule_script: self.pop_schedule_script.clone(),
            read_batch_size: self.read_batch_size,
            read_interval: self.read_interval,
        }
    }
}

#[async_trait]
impl DequeuBackend for RedisBackend {
    async fn dequeue(&self, time: f64) -> Vec<QueuedTask> {
        let mut con = self.client.get_async_connection().await.unwrap();
        let result: Vec<String> = self
            .pop_schedule_script
            .key(self.queue_key)
            .arg(time)
            .arg(self.read_batch_size)
            .invoke_async(&mut con)
            .await
            .unwrap();

        if result.len() == 0 {
            sleep(self.read_interval).await;
            return vec![];
        }

        return result
            .iter()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
    }
}

#[async_trait]
impl EnqueuBackend for RedisBackend {
    async fn enqueue(&self, task: QueuedTask, time: f64) {
        let mut con = self.client.get_async_connection().await.unwrap();
        let _: () = con
            .zadd(self.queue_key, serde_json::to_string(&task).unwrap(), time)
            .await
            .unwrap();
    }
}
