use std::{
    fmt,
    future::Future,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::time::{sleep, Duration};

use crate::prelude::{Consumer, DequeuBackend};

pub fn now() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64
}

pub async fn poll_tasks<T, R, E, Fut>(
    interval: u64,
    consumer: Consumer<T, R, f64, E>,
    func: impl Fn(Result<Vec<R>, E>) -> Fut,
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
                    sleep(Duration::from_millis(interval)).await;
                    continue;
                }
                if !func(Ok(tasks)).await {
                    break;
                }
            }
            Err(err) => {
                if !func(Err(err)).await {
                    break;
                }
                sleep(Duration::from_millis(interval)).await;
            }
        }
    }
}
