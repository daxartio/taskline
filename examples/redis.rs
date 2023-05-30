extern crate redis;

use serde::{Deserialize, Serialize};

use taskline::backends::redis::RedisBackend;
use taskline::coders::json::{json_fn, schedule_json};
use taskline::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    key: String,
}

fn handle_task(request: Data) {
    println!("Consumed: '{:?}'", request,);
}

fn main() {
    let mut tasks = Tasks::new();
    task!(tasks, task_name, json_fn!(handle_task));

    let backend = RedisBackend::new("redis://127.0.0.1/");
    let producer = Producer::new(backend.clone());
    let mut consumer = Consumer::new(backend.clone());

    schedule_json!(
        producer,
        task_name,
        &Data {
            key: "Hello!".to_string()
        },
        5000.
    );
    consumer.include_tasks(tasks);
    consumer.run();
}
