use std::marker::PhantomData;

use crate::backend::DequeuBackend;

pub struct Consumer<T, R, S>
where
    T: DequeuBackend<R, S>,
{
    backend: T,
    _phantom_request: PhantomData<R>,
    _phantom_score: PhantomData<S>,
}

impl<T, R, S> Consumer<T, R, S>
where
    T: DequeuBackend<R, S>,
{
    pub fn new(backend: T) -> Consumer<T, R, S> {
        Consumer {
            backend,
            _phantom_request: PhantomData,
            _phantom_score: PhantomData,
        }
    }

    pub async fn next(&self, score: S) -> Vec<R> {
        self.backend.dequeue(score).await
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::sync::{Arc, Mutex};
    use std::vec;

    use super::*;
    use crate::prelude::EnqueuBackend;
    use crate::producer::Producer;

    use async_trait::async_trait;

    struct MemBackend {
        queue: Arc<Mutex<RefCell<Vec<i32>>>>,
    }

    impl MemBackend {
        #[allow(dead_code)]
        fn new() -> MemBackend {
            MemBackend {
                queue: Arc::new(Mutex::new(RefCell::new(Vec::new()))),
            }
        }

        #[allow(dead_code)]
        fn clone(&self) -> MemBackend {
            MemBackend {
                queue: self.queue.clone(),
            }
        }
    }

    #[async_trait]
    impl DequeuBackend<i32, ()> for MemBackend {
        async fn dequeue(&self, _score: ()) -> Vec<i32> {
            vec![self.queue.lock().unwrap().borrow().first().unwrap().clone()]
        }
    }

    #[async_trait]
    impl EnqueuBackend<i32, ()> for MemBackend {
        async fn enqueue(&self, task: i32, _score: ()) {
            self.queue.lock().unwrap().borrow_mut().push(task);
        }
    }

    #[tokio::test]
    async fn test_consumer() {
        let backend = MemBackend::new();
        let client = Producer::new(backend.clone());
        let consumer = Consumer::new(backend);

        client.schedule(1, ()).await;

        loop {
            let tasks = consumer.next(()).await;
            for task in tasks {
                assert_eq!(task, 1);
            }
            break;
        }
    }
}
