use async_trait::async_trait;

#[async_trait]
pub trait DequeuBackend<R, S> {
    async fn dequeue(&self, score: S) -> Vec<R>;
}

#[async_trait]
pub trait EnqueuBackend<R, S> {
    async fn enqueue(&self, task: R, score: S);
}
