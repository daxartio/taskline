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
    let backend = JsonRedisBackend::<Data>::new(RedisBackend::new(
        redis::Client::open("redis://127.0.0.1/").unwrap(),
        String::from("taskline"),
        10,
        false,
    ));
    let producer = Producer::new(backend.clone());
    let consumer = Consumer::new(backend.clone());
    let committer = Committer::new(backend.clone());

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
            let task = task.unwrap();
            println!("Consumed {:?}", task);
            committer.commit(&task).await.unwrap();
        }
        true
    })
    .await;
}
