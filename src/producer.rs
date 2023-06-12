use std::marker::PhantomData;

use crate::backend::EnqueuBackend;

pub struct Producer<T, R, S>
where
    T: EnqueuBackend<R, S>,
{
    backend: T,
    _phantom_request: PhantomData<R>,
    _phantom_score: PhantomData<S>,
}

impl<T, R, S> Producer<T, R, S>
where
    T: EnqueuBackend<R, S>,
{
    pub fn new(backend: T) -> Producer<T, R, S> {
        Producer {
            backend,
            _phantom_request: PhantomData,
            _phantom_score: PhantomData,
        }
    }

    pub async fn schedule(&self, task: R, score: S) {
        self.backend.enqueue(task, score).await;
    }
}
