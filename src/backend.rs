use crate::tasks::QueuedTask;

pub trait DequeuBackend {
    fn dequeue(&self) -> Option<QueuedTask>;
}

pub trait EnqueuBackend {
    fn enqueue(&self, task: QueuedTask);
}
