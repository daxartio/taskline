use std::marker::PhantomData;

use crate::backend::DequeuBackend;

pub struct Consumer<T, R, S, E>
where
    T: DequeuBackend<R, S, E>,
{
    backend: T,
    _phantom_request: PhantomData<R>,
    _phantom_score: PhantomData<S>,
    _phantom_error: PhantomData<E>,
}

impl<T, R, S, E> Consumer<T, R, S, E>
where
    T: DequeuBackend<R, S, E>,
{
    pub fn new(backend: T) -> Consumer<T, R, S, E> {
        Consumer {
            backend,
            _phantom_request: PhantomData,
            _phantom_score: PhantomData,
            _phantom_error: PhantomData,
        }
    }

    pub async fn poll(&self, score: S) -> Result<Vec<R>, E> {
        self.backend.dequeue(score).await
    }
}
