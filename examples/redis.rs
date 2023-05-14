extern crate redis;

use serde::{Deserialize, Serialize};

use taskline::backends::redis::RedisBackend;
use taskline::coders::json::{json_call, json_fn, json_schedule};
use taskline::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    key: String,
}

fn consume(request: Data) {
    println!("Consumed: {:?}", request);
}

fn main() {
    let mut tasks = Tasks::new();
    task!(tasks, consume, json_fn!(consume));

    let backend = RedisBackend::new("redis://127.0.0.1/");
    let client = Client::new(backend.clone());
    let mut consumer = Consumer::new(backend.clone());

    json_call!(
        client,
        consume,
        &Data {
            key: "value".to_string()
        }
    );
    json_schedule!(
        client,
        consume,
        &Data {
            key: "value".to_string()
        },
        1000
    );
    consumer.include_tasks(tasks);
    consumer.run();
}
