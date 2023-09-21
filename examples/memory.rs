use std::time::{SystemTime, UNIX_EPOCH};

use tokio::time::{sleep, Duration};

use taskline::prelude::*;

#[derive(Debug, PartialEq, Clone)]
struct Task {
    name: String,
}

fn now() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64
}

#[tokio::main]
async fn main() {
    let backend = MemoryBackend::new();
    let producer = Producer::new(backend.clone());
    let consumer = Consumer::new(backend.clone());
    let committer = Committer::new(backend.clone());

    producer
        .schedule(
            &Task {
                name: "task".to_string(),
            },
            &now(),
        )
        .await
        .unwrap();
    producer
        .schedule(
            &Task {
                name: "task2".to_string(),
            },
            &(now() + 3000.),
        )
        .await
        .unwrap();

    loop {
        let tasks = consumer.poll(&now()).await.unwrap();
        if tasks.is_empty() {
            sleep(Duration::from_millis(100)).await;
            continue;
        }
        for task in tasks {
            println!("Consumed {:?}", task);
            committer.commit(&task).await.unwrap();
        }
    }
}
