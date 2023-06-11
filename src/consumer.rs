use std::cell::RefCell;
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use std::vec;

use async_trait::async_trait;

use crate::backend::{DequeuBackend, EnqueuBackend};

pub struct Consumer<T, F, Fut>
where
    T: DequeuBackend,
    F: Fn(String) -> Fut,
    Fut: Future<Output = ()>,
{
    backend: T,
    running: bool,
    on_task: F,
}

impl<T, F, Fut> Consumer<T, F, Fut>
where
    T: DequeuBackend,
    F: Fn(String) -> Fut,
    Fut: Future<Output = ()>,
{
    pub fn new(backend: T, on_task: F) -> Consumer<T, F, Fut> {
        Consumer {
            backend,
            running: true,
            on_task,
        }
    }

    pub async fn run(&self) {
        while self.running {
            self.iter().await;
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    async fn iter(&self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as f64;
        let tasks = self.backend.dequeue(now).await;
        // spawn a task for each queued task
        for task in tasks {
            (self.on_task)(task).await;
        }
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
        let consumer = Consumer::new(backend, func);

        client.schedule("task".to_string(), 0.).await;

        consumer.iter().await;
    }
}
