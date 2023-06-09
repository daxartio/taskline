extern crate redis;

use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use taskline::backends::redis::RedisBackend;
use taskline::coders::json::{json_fn, schedule_json};
use taskline::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    text: String,
    dt: f64,
}

fn handle_task(request: Data) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64;
    println!("Consumed {} ms: '{}'", now - request.dt, request.text);
}

fn main() {
    let mut tasks = Tasks::new();
    task!(tasks, task_name, json_fn!(handle_task));

    let backend = RedisBackend::new("redis://127.0.0.1/");
    let producer = Producer::new(backend.clone());
    let mut consumer = Consumer::new(backend.clone());

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64;

    schedule_json!(
        producer,
        task_name,
        &Data {
            text: "Hello!".to_string(),
            dt: now,
        },
        5000.
    );
    consumer.include_tasks(tasks);
    consumer.run();
}
