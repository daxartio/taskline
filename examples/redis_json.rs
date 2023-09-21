extern crate redis;
use serde::{Deserialize, Serialize};

use taskline::prelude::*;

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
        true,
    ));
    let producer = Producer::new(backend.clone());
    let consumer = Consumer::new(backend.clone());

    if !backend.is_redis_version_ok().await.unwrap() {
        return;
    }

    producer
        .schedule(
            &Data {
                id: 1,
                name: "Task".to_string(),
            },
            &(now() + 1000.),
        )
        .await
        .unwrap();

    poll_tasks(100, consumer, |tasks| async {
        for task in tasks.unwrap() {
            tokio::task::spawn(async move {
                println!("Consumed {:?}", task.unwrap());
            });
        }
        true
    })
    .await;
}
