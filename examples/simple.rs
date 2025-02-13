use std::time::{SystemTime, UNIX_EPOCH};

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
    let client =
        redis::Client::open("redis://127.0.0.1/").expect("the redis client should be created");

    let queue = Taskline::new(
        TasklineConfig::<&str>::new()
            .queue_key("taskline-queue")
            .build(),
        client,
    );

    queue
        .write("Hello!".to_string(), now() + 3000.)
        .await
        .unwrap();

    println!("the message 'Hello!' will be read in 3s");

    loop {
        let tasks: Vec<String> = queue
            .read(now())
            .await
            .expect("reading from redis should be ok");

        if tasks.is_empty() {
            sleep(Duration::from_millis(500)).await;
            continue;
        }

        for task in tasks {
            tokio::task::spawn(async move {
                println!("Consumed '{}'", task);
            });
        }
    }
}
