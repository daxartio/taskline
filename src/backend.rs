use async_trait::async_trait;

#[async_trait]
pub trait DequeuBackend {
    async fn dequeue(&self, time: f64) -> Vec<String>;
}

#[async_trait]
pub trait EnqueuBackend {
    async fn enqueue(&self, task: String, time: f64);
}
