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
    let queue = RedisBackendConfig {
        queue_key: "taskline",
        read_batch_size: 10,
        autodelete: true,
    }
    .with_client(redis::Client::open("redis://127.0.0.1/").unwrap());

    if !queue.is_redis_version_ok().await.unwrap() {
        return;
    }

    queue
        .write("Hello!".to_string(), now() + 1000.)
        .await
        .unwrap();

    loop {
        let tasks = queue.read(now()).await;
        match tasks {
            Ok(tasks) => {
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
            Err(e) => {
                sleep(Duration::from_millis(1000)).await;
                println!("Error: {:?}", e);
                continue;
            }
        }
    }
}
