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
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let backend1 = JsonRedisBackend::<Data>::new(
        RedisBackendConfig {
            queue_key: "step1",
            read_batch_size: 10,
            autodelete: true,
        }
        .with_client(client.clone()),
    );
    let producer1 = Producer::new(backend1.clone());
    let consumer1 = Consumer::new(backend1.clone());

    let backend2 = JsonRedisBackend::<Data>::new(
        RedisBackendConfig {
            queue_key: "step2",
            read_batch_size: 10,
            autodelete: true,
        }
        .with_client(client.clone()),
    );
    let producer2 = Producer::new(backend2.clone());
    let consumer2 = Consumer::new(backend2.clone());

    producer1
        .schedule(
            Data {
                id: 1,
                name: "Step 1".to_string(),
            },
            now() + 1000.,
        )
        .await
        .unwrap();

    let t1 = tokio::spawn(async move {
        loop {
            let tasks = consumer1.poll(now()).await.unwrap();
            if tasks.is_empty() {
                sleep(Duration::from_millis(100)).await;
                continue;
            }
            for task in tasks {
                println!("Consumed {:?}", task.unwrap());
                producer2
                    .schedule(
                        Data {
                            id: 2,
                            name: "Step 2".to_string(),
                        },
                        now() + 1000.,
                    )
                    .await
                    .unwrap();
            }
        }
    });
    let t2 = tokio::spawn(async move {
        loop {
            let tasks = consumer2.poll(now()).await.unwrap();
            if tasks.is_empty() {
                sleep(Duration::from_millis(100)).await;
                continue;
            }
            for task in tasks {
                println!("Consumed {:?}", task.unwrap());
            }
        }
    });
    let _ = tokio::join!(t1, t2);
}
