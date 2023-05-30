use crate::tasks::QueuedTask;

pub trait DequeuBackend {
    fn dequeue(&self, time: f64) -> Vec<QueuedTask>;
}

pub trait EnqueuBackend {
    fn enqueue(&self, task: QueuedTask, time: f64);
}
