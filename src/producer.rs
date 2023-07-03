//! Producer of tasks.
//! Main struct for producing tasks to queue.
use std::marker::PhantomData;

use crate::backend::EnqueuBackend;

/// Abstract producer of tasks.
/// It is generic over the backend used to enqueue tasks.
/// R, S, E - types of request, score and error.
pub struct Producer<T, R, S, E>
where
    T: EnqueuBackend<R, S, E>,
{
    backend: T,
    _phantom_request: PhantomData<R>,
    _phantom_score: PhantomData<S>,
    _phantom_error: PhantomData<E>,
}

impl<T, R, S, E> Producer<T, R, S, E>
where
    T: EnqueuBackend<R, S, E>,
{
    /// Creates new producer.
    pub fn new(backend: T) -> Producer<T, R, S, E> {
        Producer {
            backend,
            _phantom_request: PhantomData,
            _phantom_score: PhantomData,
            _phantom_error: PhantomData,
        }
    }

    /// Schedules a task to queue.
    /// Returns `Ok(())` if task was successfully scheduled.
    pub async fn schedule(&self, task: R, score: S) -> Result<(), E> {
        self.backend.enqueue(task, score).await
    }
}
