use std::time::{SystemTime, UNIX_EPOCH};

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
    let client =
        redis::Client::open("redis://127.0.0.1/").expect("the redis client should be created");

    let queue = Taskline::new(TasklineConfig::<i32>::new().queue_key(123).build(), client);

    queue
        .write(
            serde_json::to_string(&Data {
                id: 1,
                name: "Task".to_string(),
            })
            .ok(),
            now() + 3000.,
        )
        .await
        .unwrap();

    println!("the data '1:Task!' will be read in 3s");

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
                println!(
                    "Consumed '{:?}'",
                    serde_json::from_str::<Data>(&task).expect("the message should be parsed")
                );
            });
        }
    }
}
