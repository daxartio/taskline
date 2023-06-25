//! Consumer of tasks.
//! Main struct for consuming tasks from queue.
use std::marker::PhantomData;

use crate::backend::DequeuBackend;

/// Abstract consumer of tasks.
/// It is generic over the backend used to dequeue tasks.
/// R, S, E - types of request, score and error.
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
    /// Creates new consumer.
    pub fn new(backend: T) -> Consumer<T, R, S, E> {
        Consumer {
            backend,
            _phantom_request: PhantomData,
            _phantom_score: PhantomData,
            _phantom_error: PhantomData,
        }
    }

    /// Polls tasks from queue.
    /// Returns vector of tasks.
    /// If there are no tasks in queue, returns empty vector.
    /// If there are no tasks with score less than `score`, returns empty vector.
    pub async fn poll(&self, score: S) -> Result<Vec<R>, E> {
        self.backend.dequeue(score).await
    }
}
