use taskline::prelude::*;

#[derive(Debug, PartialEq, Clone)]
struct Task {
    name: String,
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

    poll_tasks(100, consumer, |tasks| async {
        for task in tasks.unwrap() {
            println!("Consumed {:?}", task);
            committer.commit(&task).await.unwrap();
        }
        true
    })
    .await;
}
