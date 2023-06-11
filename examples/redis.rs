extern crate redis;

use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use tokio;

use taskline::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    text: String,
    dt: f64,
}

async fn handle_task(request: String) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64;
    let data: Data = serde_json::from_str(&request).unwrap();
    println!("Consumed {} ms: '{}'", now - data.dt, data.text)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut tasks = Tasks::new();
    tasks.add_task("task_name", handle_task);

    let backend = RedisBackend::new("redis://127.0.0.1/");
    let producer = Producer::new(backend.clone());
    let mut consumer = Consumer::new(backend.clone());

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64;

    producer
        .schedule(
            QueuedTask {
                name: "task_name".to_string(),
                request: serde_json::to_string(&Data {
                    text: "Hello!".to_string(),
                    dt: now,
                })
                .unwrap(),
            },
            1000.,
        )
        .await;
    consumer.include_tasks(tasks);
    consumer.run().await;
}
