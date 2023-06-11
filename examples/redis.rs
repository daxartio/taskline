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
    let backend = RedisBackend::new("redis://127.0.0.1/");
    let producer = Producer::new(backend.clone());
    let consumer = Consumer::new(backend.clone(), handle_task);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64;

    producer
        .schedule(
            serde_json::to_string(&Data {
                text: "Hello!".to_string(),
                dt: now,
            })
            .unwrap(),
            1000.,
        )
        .await;
    consumer.run().await;
}
