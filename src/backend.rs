use async_trait::async_trait;

use crate::tasks::QueuedTask;

#[async_trait]
pub trait DequeuBackend {
    async fn dequeue(&self, time: f64) -> Vec<QueuedTask>;
}

#[async_trait]
pub trait EnqueuBackend {
    async fn enqueue(&self, task: QueuedTask, time: f64);
}
