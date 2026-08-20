use std::time::{SystemTime, UNIX_EPOCH};

use taskline::prelude::*;
use tokio::time::{Duration, sleep};

fn now() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("the system clock should be after the Unix epoch")
        .as_millis() as f64
}

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    let client = redis::cluster::ClusterClient::new([
        "redis://127.0.0.1:7100",
        "redis://127.0.0.1:7101",
        "redis://127.0.0.1:7102",
    ])?;
    let queue = Taskline::new(
        TasklineConfig::<&str>::new()
            .queue_key("{taskline}:queue")
            .build(),
        client,
    );

    queue.write("Hello from Redis Cluster!", now()).await?;

    loop {
        let tasks: Vec<String> = queue.read(now()).await?;
        for task in tasks {
            println!("Consumed '{task}'");
        }
        sleep(Duration::from_millis(500)).await;
    }
}
