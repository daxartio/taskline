use std::{cell::RefCell, rc::Rc};

use crate::{
    backend::{DequeuBackend, EnqueuBackend},
    tasks::{QueuedTask, Tasks},
};

pub struct Consumer<T, F>
where
    T: DequeuBackend,
    F: Fn(String) -> (),
{
    tasks: Vec<Tasks<F>>,
    backend: T,
    running: bool,
}

impl<T, F> Consumer<T, F>
where
    T: DequeuBackend,
    F: Fn(String) -> (),
{
    pub fn new(backend: T) -> Consumer<T, F> {
        Consumer {
            tasks: Vec::new(),
            backend: backend,
            running: true,
        }
    }

    pub fn include_tasks(&mut self, tasks: Tasks<F>) {
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

    pub fn run(&self) {
        while self.running {
            self.iter();
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    fn iter(&self) {
        let Some(queued_task) = self.backend.dequeue() else {
            return;
        };
        for task in &self.tasks {
            for (name, func) in &task.tasks {
                if name != &queued_task.name {
                    continue;
                }
                func(queued_task.request.clone());
                break;
            }
        }
    }
}

struct MemBackend {
    queue: Rc<RefCell<Vec<QueuedTask>>>,
}

impl MemBackend {
    fn _new() -> MemBackend {
        MemBackend {
            queue: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn _clone(&self) -> MemBackend {
        MemBackend {
            queue: self.queue.clone(),
        }
    }
}

impl DequeuBackend for MemBackend {
    fn dequeue(&self) -> Option<QueuedTask> {
        Some(self.queue.borrow().first().unwrap().clone())
    }
}

impl EnqueuBackend for MemBackend {
    fn enqueue(&self, task: QueuedTask) {
        self.queue.borrow_mut().push(task);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::Client;
    use crate::consumer;
    use crate::tasks::QueuedTask;

    #[test]
    fn test() {
        let mut tasks = Tasks::new();
        fn func(s: String) {
            println!("Called with {:?}", s)
        }

        let backend = MemBackend::_new();
        let client = Client::new(backend._clone());
        let mut consumer = Consumer::new(backend);

        client.call(QueuedTask {
            name: "func".to_string(),
            request: "task".to_string(),
        });

        tasks.add_task("func", func);

        consumer.include_tasks(tasks);
        consumer.iter();
    }

    #[test]
    #[should_panic]
    fn test_register_twice() {
        let mut consumer = consumer::Consumer::new(MemBackend::_new());
        let mut tasks = Tasks::new();
        fn func(_s: String) {}
        tasks.add_task("test", func);
        consumer.include_tasks(tasks);

        let mut tasks = Tasks::new();
        tasks.add_task("test", func);
        consumer.include_tasks(tasks);
    }
}
