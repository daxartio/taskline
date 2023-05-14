use crate::backend::EnqueuBackend;
use crate::tasks::QueuedTask;

pub struct Client<T>
where
    T: EnqueuBackend,
{
    backend: T,
}

impl<T> Client<T>
where
    T: EnqueuBackend,
{
    pub fn new(backend: T) -> Client<T> {
        Client { backend: backend }
    }

    pub fn call(&self, task: QueuedTask) {
        self.backend.enqueue(task);
    }

    pub fn schedule(&self, task: QueuedTask, _time: u64) {
        self.backend.enqueue(task);
    }
}

#[macro_export]
macro_rules! call {
    ($client:ident, $task_name:ident, $queued_task:expr) => {{
        $client.call(QueuedTask {
            name: stringify!($task_name).to_string(),
            request: $queued_task,
        });
    }};
}

pub use call;

#[macro_export]
macro_rules! schedule {
    ($client:ident, $task_name:ident, $queued_task:expr, $time:expr) => {{
        $client.schedule(
            QueuedTask {
                name: stringify!($task_name).to_string(),
                request: $queued_task,
            },
            $time,
        );
    }};
}

pub use schedule;
