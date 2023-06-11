extern crate redis;

use tokio::time::{sleep, Duration};

use taskline::prelude::*;

async fn handle_task(request: String) {
    println!("Consumed '{}'", request)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let backend = RedisBackend::new("redis://127.0.0.1/");
    let producer = Producer::new(backend.clone());
    let consumer = Consumer::new(backend.clone());

    producer.schedule("Hello!".to_string(), 1000.).await;

    loop {
        let tasks = consumer.next().await;
        if tasks.is_empty() {
            sleep(Duration::from_millis(100)).await;
            continue;
        }
        for task in tasks {
            handle_task(task).await;
        }
    }
}
