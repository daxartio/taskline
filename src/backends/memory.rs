use std::cell::RefCell;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use crate::backend::{CommitBackend, DequeuBackend, EnqueuBackend};

type Queue<T, S> = Vec<Item<T, S>>;

#[derive(Clone)]
struct Item<T, S>
where
    T: Clone + PartialEq,
    S: Copy + PartialOrd,
{
    score: S,
    val: T,
}

/// Memory backend.
///
/// New in version 0.8.0.
#[derive(Clone, Default)]
pub struct MemoryBackend<T, S>
where
    T: Clone + PartialEq,
    S: Copy + PartialOrd,
{
    queue: Arc<Mutex<RefCell<Queue<T, S>>>>,
}

impl<T, S> MemoryBackend<T, S>
where
    T: Clone + PartialEq,
    S: Copy + PartialOrd,
{
    pub fn new() -> MemoryBackend<T, S> {
        MemoryBackend {
            queue: Arc::new(Mutex::new(RefCell::new(Vec::new()))),
        }
    }

    pub fn read(&self, score: &S) -> Vec<T> {
        self.queue
            .lock()
            .unwrap()
            .borrow()
            .iter()
            .filter(|v| v.score <= *score)
            .map(|v| v.val.clone())
            .collect::<Vec<T>>()
    }

    pub fn write(&self, task: &T, score: &S) {
        let queue = self.queue.lock().unwrap();
        let mut queue = queue.borrow_mut();
        queue.retain(|v| v.val != *task);
        queue.push(Item {
            score: *score,
            val: task.clone(),
        });
    }

    pub fn delete(&self, task: &T) {
        self.queue
            .lock()
            .unwrap()
            .borrow_mut()
            .retain(|v| v.val != *task);
    }
}

#[async_trait]
impl<T, S> DequeuBackend<T, S, ()> for MemoryBackend<T, S>
where
    T: Clone + PartialEq + Send,
    S: Copy + PartialOrd + Sync + Send,
{
    async fn dequeue(&self, score: &S) -> Result<Vec<T>, ()> {
        Ok(self.read(score))
    }
}

#[async_trait]
impl<T, S> EnqueuBackend<T, S, ()> for MemoryBackend<T, S>
where
    T: Clone + PartialEq + Sync + Send,
    S: Copy + PartialOrd + Sync + Send,
{
    async fn enqueue(&self, task: &T, score: &S) -> Result<(), ()> {
        self.write(task, score);
        Ok(())
    }
}

#[async_trait]
impl<T, S> CommitBackend<T, ()> for MemoryBackend<T, S>
where
    T: Clone + PartialEq + Sync + Send,
    S: Copy + PartialOrd + Sync + Send,
{
    async fn commit(&self, task: &T) -> Result<(), ()> {
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
