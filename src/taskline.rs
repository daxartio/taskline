use redis::{AsyncCommands, FromRedisValue, RedisError, ToRedisArgs};

static POP_SCHEDULE_SCRIPT: &str = r#"
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
return res
"#;

/// Configuration for Taskline.
#[derive(Debug, Clone)]
pub struct TasklineConfig<K: ToRedisArgs + Clone> {
    queue_key: K,
    read_batch_size: usize,
    autodelete: bool,
}

impl<K: ToRedisArgs + Default + Clone> Default for TasklineConfig<K> {
    fn default() -> Self {
        Self {
            queue_key: K::default(),
            read_batch_size: 50,
            autodelete: true,
        }
    }
}

impl<K: ToRedisArgs + Default + Clone> TasklineConfig<K> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Redis key is used to store tasks.
    pub fn queue_key(mut self, queue_key: impl Into<K>) -> Self {
        self.queue_key = queue_key.into();
        self
    }

    /// Number of tasks to read in one batch.
    pub fn read_batch_size(mut self, read_batch_size: usize) -> Self {
        self.read_batch_size = read_batch_size;
        self
    }

    /// If `true`, tasks will be deleted from queue after reading.
    /// If autodelete is `false`, tasks should be deleted explicitly from queue after reading with `Taskline::delete`.
    pub fn autodelete(mut self, autodelete: bool) -> Self {
        self.autodelete = autodelete;
        self
    }

    pub fn build(self) -> Self {
        Self {
            queue_key: self.queue_key,
            read_batch_size: self.read_batch_size,
            autodelete: self.autodelete,
        }
    }
}

/// # Taskline.
/// You can use score to sort tasks in queue. Usually it is unix timestamp.
#[derive(Debug, Clone)]
pub struct Taskline<K: ToRedisArgs + Clone> {
    config: TasklineConfig<K>,
    client: redis::Client,
    pop_schedule_script: redis::Script,
}

impl<K: ToRedisArgs + Clone> Taskline<K> {
    /// Creates new instance of `Taskline`.
    ///
    /// It requires `redis::Client` instance, redis key used to store tasks and number of tasks to read in one batch.
    pub fn new(config: TasklineConfig<K>, client: redis::Client) -> Self {
        Self {
            config,
            client,
            pop_schedule_script: redis::Script::new(POP_SCHEDULE_SCRIPT),
        }
    }

    /// Calls lua script to pop tasks from redis.
    /// If there are no tasks in queue it returns empty vector.
    /// If there are no tasks with score less than `score`, returns empty vector.
    pub async fn read<S: ToRedisArgs, R: FromRedisValue>(&self, score: S) -> Result<R, RedisError> {
        let mut con = self.client.get_multiplexed_async_connection().await?;

        let result: R = self
            .pop_schedule_script
            .key(self.config.queue_key.clone())
            .arg(score)
            .arg(self.config.read_batch_size)
            .arg(self.config.autodelete as u8)
            .invoke_async(&mut con)
            .await?;

        Ok(result)
    }
}

impl<K: ToRedisArgs + Clone + Send + Sync> Taskline<K> {
    /// Adds a task to redis.
    /// It uses score to sort tasks in queue. Usually it is unix timestamp.
    pub async fn write<M: ToRedisArgs + Send + Sync, S: ToRedisArgs + Send + Sync>(
        &self,
        task: M,
        score: S,
    ) -> Result<(), RedisError> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        con.zadd(self.config.queue_key.clone(), task, score).await
    }

    /// Delete a task from queue.
    pub async fn delete<M: ToRedisArgs + Send + Sync>(&self, task: M) -> Result<(), RedisError> {
        if self.config.autodelete {
            return Ok(());
        }
        let mut con = self.client.get_multiplexed_async_connection().await?;

        con.zrem(self.config.queue_key.clone(), task).await
    }
}
