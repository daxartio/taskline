//! Committer of tasks.
//! Main struct for committing tasks from queue.
use std::marker::PhantomData;

use crate::backend::CommitBackend;

/// Abstract committer of tasks.
/// It is generic over the backend used to commit tasks.
/// R, S, E - types of request, score and error.
pub struct Committer<T, R, E>
where
    T: CommitBackend<R, E>,
{
    backend: T,
    _phantom_request: PhantomData<R>,
    _phantom_error: PhantomData<E>,
}

impl<T, R, E> Committer<T, R, E>
where
    T: CommitBackend<R, E>,
{
    /// Creates new committer.
    pub fn new(backend: T) -> Committer<T, R, E> {
        Committer {
            backend,
            _phantom_request: PhantomData,
            _phantom_error: PhantomData,
        }
    }

    pub async fn commit(&self, task: &R) -> Result<(), E> {
        self.backend.commit(task).await
    }
}
