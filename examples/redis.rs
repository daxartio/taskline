use tokio::time::{sleep, Duration};

use taskline::prelude::*;

#[tokio::main]
async fn main() {
    let queue = RedisBackendConfig {
        queue_key: "taskline".to_string(),
        read_batch_size: 10,
        autodelete: true,
    }
    .with_client(redis::Client::open("redis://127.0.0.1/").unwrap());

    if !queue.check_version().await.unwrap() {
        return;
    }

    queue
        .write(&"Hello!".to_string(), &(now() + 1000.))
        .await
        .unwrap();

    loop {
        let tasks = queue.read(&now()).await;
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
