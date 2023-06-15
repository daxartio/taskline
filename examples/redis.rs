use std::time::{SystemTime, UNIX_EPOCH};

extern crate redis;
use tokio::time::{sleep, Duration};

use taskline::prelude::*;

fn now() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64
}

#[tokio::main]
async fn main() {
    let backend = RedisBackendConfig {
        queue_key: "taskline",
        read_batch_size: 10,
    } + redis::Client::open("redis://127.0.0.1/").unwrap();
    let producer = Producer::new(backend.clone());
    let consumer = Consumer::new(backend.clone());

    producer.schedule("Hello!".to_string(), now() + 1000.).await;

    loop {
        let tasks = consumer.poll(now()).await;
        if tasks.is_empty() {
            sleep(Duration::from_millis(100)).await;
            continue;
        }
        for task in tasks {
            tokio::task::spawn(async move {
                println!("Consumed '{}'", task);
            });
        }
    }
}
