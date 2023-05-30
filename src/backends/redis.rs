extern crate redis;
use redis::{Commands, IntoConnectionInfo};

use crate::backend::{DequeuBackend, EnqueuBackend};
use crate::tasks::QueuedTask;

pub struct RedisBackend {
    client: redis::Client,
    queue_key: &'static str,
    pop_schedule_script: redis::Script,
}

impl RedisBackend {
    pub fn new<T: IntoConnectionInfo>(params: T) -> RedisBackend {
        let pop_schedule_script = redis::Script::new(
            r"
            local key = KEYS[1]
            local unix_ts = ARGV[1]
            local res = redis.call('zrange', key, '-inf', unix_ts, 'byscore')
            if #res and redis.call('zremrangebyscore', key, '-inf', unix_ts) == #res then
                return res
            end
        ",
        );
        RedisBackend {
            client: redis::Client::open(params).unwrap(),
            queue_key: "queue",
            pop_schedule_script: pop_schedule_script,
        }
    }
}

impl Clone for RedisBackend {
    fn clone(&self) -> Self {
        RedisBackend {
            client: self.client.clone(),
            queue_key: self.queue_key,
            pop_schedule_script: self.pop_schedule_script.clone(),
        }
    }
}

impl DequeuBackend for RedisBackend {
    fn dequeue(&self, time: f64) -> Vec<QueuedTask> {
        let mut con = self.client.get_connection().unwrap();
        let result: Vec<String> = self
            .pop_schedule_script
            .key(self.queue_key)
            .arg(time)
            .invoke(&mut con)
            .unwrap();

        return result
            .iter()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
    }
}

impl EnqueuBackend for RedisBackend {
    fn enqueue(&self, task: QueuedTask, time: f64) {
        let mut con = self.client.get_connection().unwrap();
        let _: () = con
            .zadd(self.queue_key, serde_json::to_string(&task).unwrap(), time)
            .unwrap();
    }
}
