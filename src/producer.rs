use std::time::{SystemTime, UNIX_EPOCH};

use crate::backend::EnqueuBackend;
use crate::tasks::QueuedTask;

pub struct Producer<T>
where
    T: EnqueuBackend,
{
    backend: T,
}

impl<T> Producer<T>
where
    T: EnqueuBackend,
{
    pub fn new(backend: T) -> Producer<T> {
        Producer { backend: backend }
    }

    pub fn schedule(&self, task: QueuedTask, time: f64) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as f64;
        self.backend.enqueue(task, now + time);
    }
}

#[macro_export]
macro_rules! schedule {
    ($producer:ident, $task_name:ident, $queued_task:expr, $time:expr) => {{
        $producer.schedule(
            QueuedTask {
                name: stringify!($task_name).to_string(),
                request: $queued_task,
            },
            $time,
        );
    }};
}

pub use schedule;
