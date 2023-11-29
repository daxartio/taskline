use taskline::committer::Committer;
use taskline::consumer::Consumer;
use taskline::producer::Producer;

mod backend {
    use std::cell::RefCell;
    use std::sync::{Arc, Mutex};
    use std::vec;

    use async_trait::async_trait;

    use taskline::prelude::{CommitBackend, DequeuBackend, EnqueuBackend};

    #[derive(Clone)]
    pub(crate) struct MemBackend {
        pub queue: Arc<Mutex<RefCell<Vec<i32>>>>,
    }

    impl MemBackend {
        pub(crate) fn new() -> MemBackend {
            MemBackend {
                queue: Arc::new(Mutex::new(RefCell::new(Vec::new()))),
            }
        }
    }

    #[async_trait]
    impl DequeuBackend<i32, (), ()> for MemBackend {
        async fn dequeue(&self, _score: &()) -> Result<Vec<i32>, ()> {
            Ok(vec![*self.queue.lock().unwrap().borrow().first().unwrap()])
        }
    }

    #[async_trait]
    impl EnqueuBackend<i32, (), ()> for MemBackend {
        async fn enqueue(&self, task: &i32, _score: &()) -> Result<(), ()> {
            self.queue.lock().unwrap().borrow_mut().push(*task);
            Ok(())
        }
    }

    #[async_trait]
    impl CommitBackend<i32, ()> for MemBackend {
        async fn commit(&self, task: &i32) -> Result<(), ()> {
            self.queue
                .lock()
                .unwrap()
                .borrow_mut()
                .retain(|x| *x != *task);
            Ok(())
        }
    }
}

#[tokio::test]
async fn test_consumer() {
    let backend = backend::MemBackend::new();
    let client = Producer::new(backend.clone());
    let consumer = Consumer::new(backend.clone());
    let committer = Committer::new(backend.clone());

    client.schedule(&1, &()).await.unwrap();

    let tasks = consumer.poll(&()).await.unwrap();
    for task in tasks {
        assert_eq!(task, 1);
        committer.commit(&task).await.unwrap();
    }
    assert!(backend.queue.lock().unwrap().borrow().is_empty());
}
