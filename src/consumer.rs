use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use std::vec;

use async_trait::async_trait;

use crate::backend::{DequeuBackend, EnqueuBackend};

pub struct Consumer<T>
where
    T: DequeuBackend,
{
    backend: T,
}

impl<T> Consumer<T>
where
    T: DequeuBackend,
{
    pub fn new(backend: T) -> Consumer<T> {
        Consumer { backend }
    }

    pub async fn next(&self) -> Vec<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as f64;
        self.backend.dequeue(now).await
    }
}

struct MemBackend {
    queue: Arc<Mutex<RefCell<Vec<String>>>>,
}

impl MemBackend {
    #[allow(dead_code)]
    fn new() -> MemBackend {
        MemBackend {
            queue: Arc::new(Mutex::new(RefCell::new(Vec::new()))),
        }
    }

    #[allow(dead_code)]
    fn clone(&self) -> MemBackend {
        MemBackend {
            queue: self.queue.clone(),
        }
    }
}

#[async_trait]
impl DequeuBackend for MemBackend {
    async fn dequeue(&self, _time: f64) -> Vec<String> {
        vec![self.queue.lock().unwrap().borrow().first().unwrap().clone()]
    }
}

#[async_trait]
impl EnqueuBackend for MemBackend {
    async fn enqueue(&self, task: String, _time: f64) {
        self.queue.lock().unwrap().borrow_mut().push(task);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::producer::Producer;
    use tokio;

    #[tokio::test]
    async fn test_consumer() {
        async fn func(s: String) {
            println!("Called with {:?}", s)
        }

        let backend = MemBackend::new();
        let client = Producer::new(backend.clone());
        let consumer = Consumer::new(backend);

        client.schedule("task".to_string(), 0.).await;

        loop {
            let tasks = consumer.next().await;
            for task in tasks {
                func(task).await;
            }
            break;
        }
    }
}
