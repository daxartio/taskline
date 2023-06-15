use taskline::consumer::Consumer;
use taskline::producer::Producer;

mod backend {
    use std::cell::RefCell;
    use std::sync::{Arc, Mutex};
    use std::vec;

    use async_trait::async_trait;

    use taskline::prelude::{DequeuBackend, EnqueuBackend};

    #[derive(Clone)]
    pub(crate) struct MemBackend {
        queue: Arc<Mutex<RefCell<Vec<i32>>>>,
    }

    impl MemBackend {
        pub(crate) fn new() -> MemBackend {
            MemBackend {
                queue: Arc::new(Mutex::new(RefCell::new(Vec::new()))),
            }
        }
    }

    #[async_trait]
    impl DequeuBackend<i32, ()> for MemBackend {
        async fn dequeue(&self, _score: ()) -> Vec<i32> {
            vec![*self.queue.lock().unwrap().borrow().first().unwrap()]
        }
    }

    #[async_trait]
    impl EnqueuBackend<i32, ()> for MemBackend {
        async fn enqueue(&self, task: i32, _score: ()) {
            self.queue.lock().unwrap().borrow_mut().push(task);
        }
    }
}

#[tokio::test]
async fn test_consumer() {
    let backend = backend::MemBackend::new();
    let client = Producer::new(backend.clone());
    let consumer = Consumer::new(backend);

    client.schedule(1, ()).await;

    let tasks = consumer.poll(()).await;
    for task in tasks {
        assert_eq!(task, 1);
    }
}
