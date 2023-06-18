use async_trait::async_trait;

#[async_trait]
pub trait DequeuBackend<R, S, E> {
    async fn dequeue(&self, score: S) -> Result<Vec<R>, E>;
}

#[async_trait]
pub trait EnqueuBackend<R, S, E> {
    async fn enqueue(&self, task: R, score: S) -> Result<(), E>;
}
