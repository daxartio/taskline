use redis::{AsyncCommands, RedisError};

/// Configuration for Redis backend.
/// You can use it to create `RedisBackend` instance.
#[derive(Debug, Clone)]
pub struct RedisBackendConfig {
    /// Redis key is used to store tasks.
    pub queue_key: String,
    /// Number of tasks to read in one batch.
    pub read_batch_size: usize,
    /// If `true`, tasks will be deleted from queue after reading.
    /// If autodelete is `false`, tasks should be deleted explicitly from queue after reading with `RedisBackend::delete`.
    pub autodelete: bool,
}

impl RedisBackendConfig {
    /// Create `RedisBackend` instance.
    /// It requires `redis::Client` instance.
    pub fn with_client(self, client: redis::Client) -> RedisBackend {
        RedisBackend::new(
            client,
            self.queue_key.to_string(),
            self.read_batch_size,
            self.autodelete,
        )
    }
}

/// Redis backend.
/// It implements both `DequeuBackend` and `EnqueuBackend` traits.
/// You can use score to sort tasks in queue. Usually it is unix timestamp.
#[derive(Debug, Clone)]
pub struct RedisBackend {
    client: redis::Client,
    queue_key: String,
    pop_schedule_script: redis::Script,
    read_batch_size: usize,
    pub(crate) autodelete: bool,
}

impl RedisBackend {
    /// Create new instance of `RedisBackend`.
    ///
    /// It requires `redis::Client` instance, redis key used to store tasks and number of tasks to read in one batch.
    /// It also creates lua script used to pop tasks from redis.
    /// * `client` - redis client.
    /// * `queue_key` - redis key is used to store tasks.
    /// * `read_batch_size` - number of tasks to read in one batch.
    /// * `autodelete` - if `true`, tasks will be deleted from queue after reading. If `false`, tasks should be deleted explicitly from queue after reading with `RedisBackend::delete`. New in version 0.5.0.
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

    /// Calls lua script to pop tasks from redis.
    /// If there are no tasks in queue it returns empty vector.
    /// If there are no tasks with score less than `score`, returns empty vector.
    pub async fn read(&self, score: &f64) -> Result<Vec<String>, RedisError> {
        let mut con = self.client.get_multiplexed_async_connection().await?;

        let result: Vec<String> = self
            .pop_schedule_script
            .key(self.queue_key.as_str())
            .arg(score)
            .arg(self.read_batch_size)
            .arg(self.autodelete as u8)
            .invoke_async(&mut con)
            .await?;

        Ok(result)
    }

    /// Adds a task to redis.
    /// It uses score to sort tasks in queue. Usually it is unix timestamp.
    pub async fn write(&self, task: &String, score: &f64) -> Result<(), RedisError> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        con.zadd(self.queue_key.as_str(), task, score).await
    }

    /// Delete a task from queue.
    pub async fn delete(&self, task: &String) -> Result<(), RedisError> {
        if self.autodelete {
            return Ok(());
        }
        let mut con = self.client.get_multiplexed_async_connection().await?;

        con.zrem(self.queue_key.as_str(), task).await
    }

    /// Check redis version.
    pub async fn check_version(&self) -> Result<bool, RedisError> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        let res: String = redis::cmd("INFO").query_async(&mut con).await?;
        let mut ver = res
            .lines()
            .find(|s| s.contains("redis_version:"))
            .unwrap()
            .split(':')
            .last()
            .unwrap()
            .split('.')
            .take(2);

        let major: u8 = ver.next().unwrap().parse().unwrap();
        let minor: u8 = ver.next().unwrap().parse().unwrap();
        Ok((major, minor) >= (6, 2))
    }
}
