use std::time::{SystemTime, UNIX_EPOCH};

extern crate redis;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

use taskline::prelude::*;

fn now() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64
}

#[derive(Deserialize, Serialize, Debug)]
struct Data {
    id: u64,
    name: String,
}

#[derive(Clone)]
struct JsonRedisBackend {
    backend: RedisBackend,
}

#[async_trait]
impl EnqueuBackend<Data, f64> for JsonRedisBackend {
    async fn enqueue(&self, data: Data, score: f64) {
        let data = serde_json::to_string(&data).unwrap();
        self.backend.enqueue(data, score).await
    }
}

#[async_trait]
impl DequeuBackend<Data, f64> for JsonRedisBackend {
    async fn dequeue(&self, score: f64) -> Vec<Data> {
        let data = self.backend.dequeue(score).await;
        data.into_iter()
            .map(|d| serde_json::from_str(&d).unwrap())
            .collect()
    }
}

#[tokio::main]
async fn main() {
    let queue_key = String::from("taskline");
    let backend: JsonRedisBackend = JsonRedisBackend {
        backend: RedisBackend::new(
            redis::Client::open("redis://127.0.0.1/").unwrap(),
            queue_key,
            10,
        ),
    };
    let producer = Producer::new(backend.clone());
    let consumer = Consumer::new(backend.clone());

    producer
        .schedule(
            Data {
                id: 1,
                name: "Task".to_string(),
            },
            now() + 1000.,
        )
        .await;

    loop {
        let tasks = consumer.poll(now()).await;
        if tasks.is_empty() {
            sleep(Duration::from_millis(100)).await;
            continue;
        }
        for task in tasks {
            tokio::task::spawn(async move {
                println!("Consumed {:?}", task);
            });
        }
    }
}
