extern crate redis;
use redis::{Commands, IntoConnectionInfo};

use crate::backend::{DequeuBackend, EnqueuBackend};
use crate::tasks::QueuedTask;

pub struct RedisBackend {
    client: redis::Client,
    read_timeout: usize,
    queue_key: &'static str,
}

impl RedisBackend {
    pub fn new<T: IntoConnectionInfo>(params: T) -> RedisBackend {
        RedisBackend {
            client: redis::Client::open(params).unwrap(),
            read_timeout: 1,
            queue_key: "queue",
        }
    }
}

impl Clone for RedisBackend {
    fn clone(&self) -> Self {
        RedisBackend {
            client: self.client.clone(),
            read_timeout: self.read_timeout,
            queue_key: self.queue_key,
        }
    }
}

impl DequeuBackend for RedisBackend {
    fn dequeue(&self) -> Option<QueuedTask> {
        let mut con = self.client.get_connection().unwrap();

        let result: Option<(String, String)> =
            con.brpop(self.queue_key, self.read_timeout).unwrap();
        match result {
            Some((_, value)) => match serde_json::from_str(&value) {
                Ok(task) => Some(task),
                Err(_) => None,
            },
            None => None,
        }
    }
}

impl EnqueuBackend for RedisBackend {
    fn enqueue(&self, task: QueuedTask) {
        let mut con = self.client.get_connection().unwrap();
        let _: () = con
            .lpush(self.queue_key, serde_json::to_string(&task).unwrap())
            .unwrap();
    }
}
