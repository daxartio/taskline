use std::{
    fmt,
    future::Future,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::time::{sleep, Duration};

use crate::prelude::{Consumer, DequeuBackend};

/// Returns current time.
pub fn now() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64
}

/// Polls tasks from a backend and calls the function in the loop.
pub async fn poll_tasks<T, R, E, Fut>(
    interval: u64,
    consumer: Consumer<T, R, f64, E>,
    callback: impl Fn(Result<Vec<R>, E>) -> Fut,
) where
    T: DequeuBackend<R, f64, E>,
    Fut: Future<Output = bool>,
    E: fmt::Debug,
{
    loop {
        let score = now();
        match consumer.poll(&score).await {
            Ok(tasks) => {
                if tasks.is_empty() {
                    if interval != 0 {
                        sleep(Duration::from_millis(interval)).await;
                    }
                    continue;
                }
                if !callback(Ok(tasks)).await {
                    break;
                }
            }
            Err(err) => {
                if !callback(Err(err)).await {
                    break;
                }
                if interval != 0 {
                    sleep(Duration::from_millis(interval)).await;
                }
            }
        }
    }
}
