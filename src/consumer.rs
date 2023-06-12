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
