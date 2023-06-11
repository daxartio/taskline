use std::cell::RefCell;
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use std::vec;

use async_trait::async_trait;

use crate::{
    backend::{DequeuBackend, EnqueuBackend},
    tasks::{QueuedTask, Tasks},
};

pub struct Consumer<T, F, Fut>
where
    T: DequeuBackend,
    F: Fn(String) -> Fut,
    Fut: Future<Output = ()>,
{
    tasks: Vec<Tasks<F, Fut>>,
    backend: T,
    running: bool,
}

impl<T, F, Fut> Consumer<T, F, Fut>
where
    T: DequeuBackend,
    F: Fn(String) -> Fut,
    Fut: Future<Output = ()>,
{
    pub fn new(backend: T) -> Consumer<T, F, Fut> {
        Consumer {
            tasks: Vec::new(),
            backend,
            running: true,
        }
    }

    pub fn include_tasks(&mut self, tasks: Tasks<F, Fut>) {
        for (name, _) in &tasks.tasks {
            if self
                .tasks
                .iter()
                .any(|t| t.tasks.iter().any(|(n, _)| n == name))
            {
                panic!("Task name '{}' already exists", name);
            }
        }
        self.tasks.push(tasks);
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
        let queued_tasks = self.backend.dequeue(now).await;
        // spawn a task for each queued task
        for queued_task in queued_tasks {
            for task in &self.tasks {
                let Some(func) = task.get_task(&queued_task.name) else {
                    continue;
                };
                func(queued_task.request.clone()).await;
            }
        }
    }
}

struct MemBackend {
    queue: Arc<Mutex<RefCell<Vec<QueuedTask>>>>,
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
    async fn dequeue(&self, _time: f64) -> Vec<QueuedTask> {
        vec![self.queue.lock().unwrap().borrow().first().unwrap().clone()]
    }
}

#[async_trait]
impl EnqueuBackend for MemBackend {
    async fn enqueue(&self, task: QueuedTask, _time: f64) {
        self.queue.lock().unwrap().borrow_mut().push(task);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consumer;
    use crate::producer::Producer;
    use crate::tasks::QueuedTask;
    use tokio;

    #[tokio::test]
    async fn test_consumer() {
        let mut tasks = Tasks::new();
        async fn func(s: String) {
            println!("Called with {:?}", s)
        }

        let backend = MemBackend::new();
        let client = Producer::new(backend.clone());
        let mut consumer = Consumer::new(backend);

        client
            .schedule(
                QueuedTask {
                    name: "func".to_string(),
                    request: "task".to_string(),
                },
                0.,
            )
            .await;

        tasks.add_task("func", func);

        consumer.include_tasks(tasks);
        consumer.iter().await;
    }

    #[test]
    #[should_panic]
    fn test_register_twice() {
        let mut consumer = consumer::Consumer::new(MemBackend::new());
        let mut tasks = Tasks::new();
        async fn func(_s: String) {}
        tasks.add_task("test", func);
        consumer.include_tasks(tasks);

        let mut tasks = Tasks::new();
        tasks.add_task("test", func);
        consumer.include_tasks(tasks);
    }
}
