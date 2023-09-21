//! Backend traits for queue.
//! You can implement your own backend by implementing this traits.
use async_trait::async_trait;

/// The trait for backend implementations which can be used to dequeue tasks.
///
/// R - type of task.
/// S - type of score used to sort tasks in queue.
/// E - type of error.
#[async_trait]
pub trait DequeuBackend<R, S, E> {
    async fn dequeue(&self, score: &S) -> Result<Vec<R>, E>;
}

/// The trait for backend implementations which can be used to enqueue tasks.
///
/// R - type of task.
/// S - type of score used to sort tasks in queue.
/// E - type of error.
#[async_trait]
pub trait EnqueuBackend<R, S, E> {
    async fn enqueue(&self, task: &R, score: &S) -> Result<(), E>;
}

/// The trait for backend implementations which can be used to commit tasks.
///
/// R - type of task.
/// E - type of error.
#[async_trait]
pub trait CommitBackend<R, E> {
    async fn commit(&self, task: &R) -> Result<(), E>;
}
