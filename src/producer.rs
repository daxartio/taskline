use std::time::{SystemTime, UNIX_EPOCH};

use crate::backend::EnqueuBackend;

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

    pub async fn schedule(&self, task: String, time: f64) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as f64;
        self.backend.enqueue(task, now + time).await;
    }
}
