use std::time::{SystemTime, UNIX_EPOCH};

extern crate redis;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

use taskline::prelude::*;

fn now() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Data {
    id: u64,
    name: String,
}

#[tokio::main]
async fn main() {
    let queue_key = String::from("taskline");
    let backend = JsonRedisBackend::<Data>::new(RedisBackend::new(
        redis::Client::open("redis://127.0.0.1/").unwrap(),
        queue_key,
        10,
    ));
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
        .await
        .unwrap();

    loop {
        let tasks = consumer.poll(now()).await.unwrap();
        if tasks.is_empty() {
            sleep(Duration::from_millis(100)).await;
            continue;
        }
        for task in tasks {
            tokio::task::spawn(async move {
                println!("Consumed {:?}", task.unwrap());
            });
        }
    }
}
