use std::cell::RefCell;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use crate::backend::{CommitBackend, DequeuBackend, EnqueuBackend};

#[derive(Clone)]
struct Item {
    score: f64,
    val: String,
}

/// Memory backend.
///
/// New in version 0.8.0.
#[derive(Clone, Default)]
pub struct MemoryBackend {
    queue: Arc<Mutex<RefCell<Vec<Item>>>>,
}

impl MemoryBackend {
    pub fn new() -> MemoryBackend {
        MemoryBackend {
            queue: Arc::new(Mutex::new(RefCell::new(Vec::new()))),
        }
    }

    pub fn read(&self, score: &f64) -> Vec<String> {
        self.queue
            .lock()
            .unwrap()
            .borrow()
            .iter()
            .filter(|v| v.score <= *score)
            .map(|v| v.val.clone())
            .collect::<Vec<String>>()
    }

    pub fn write(&self, task: &String, score: &f64) {
        let queue = self.queue.lock().unwrap();
        let mut queue = queue.borrow_mut();
        queue.retain(|v| *v.val != *task);
        queue.push(Item {
            score: *score,
            val: task.clone(),
        });
    }

    pub fn delete(&self, task: &String) {
        self.queue
            .lock()
            .unwrap()
            .borrow_mut()
            .retain(|v| *v.val != *task);
    }
}

#[async_trait]
impl<'a> DequeuBackend<'a, String, f64, ()> for MemoryBackend {
    async fn dequeue(&self, score: &'a f64) -> Result<Vec<String>, ()> {
        Ok(self.read(score))
    }
}

#[async_trait]
impl<'a> EnqueuBackend<'a, String, f64, ()> for MemoryBackend {
    async fn enqueue(&self, task: &'a String, score: &'a f64) -> Result<(), ()> {
        self.write(task, score);
        Ok(())
    }
}

#[async_trait]
impl<'a> CommitBackend<'a, String, ()> for MemoryBackend {
    async fn commit(&self, task: &'a String) -> Result<(), ()> {
        self.delete(task);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_consumer() {
        let backend = MemoryBackend::new();

        backend.enqueue(&"task".to_string(), &1.).await.unwrap();
        backend.enqueue(&"task".to_string(), &1.).await.unwrap();

        let tasks = backend.dequeue(&1.).await.unwrap();
        for task in tasks {
            assert_eq!(task, "task".to_string());
            backend.commit(&task).await.unwrap();
        }
        assert!(backend.queue.lock().unwrap().borrow().is_empty());
    }
}
