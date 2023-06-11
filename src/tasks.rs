use std::future::Future;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct QueuedTask {
    pub name: String,
    pub request: String,
}

pub struct Tasks<T, Fut>
where
    T: Fn(String) -> Fut,
    Fut: Future<Output = ()>,
{
    pub tasks: Vec<(&'static str, T)>,
}

impl<T, Fut> Tasks<T, Fut>
where
    T: Fn(String) -> Fut,
    Fut: Future<Output = ()>,
{
    pub fn new() -> Tasks<T, Fut> {
        Tasks { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, name: &'static str, func: T) {
        if self.tasks.iter().any(|(n, _)| n == &name) {
            panic!("Task name '{}' already exists", name);
        }
        self.tasks.push((name, func));
    }

    pub fn get_task(&self, name: &str) -> Option<&T> {
        self.tasks.iter().find_map(|(n, f)| if n == &name { Some(f) } else { None })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut tasks = Tasks::new();
        async fn func(_s: String) {}
        tasks.add_task("test", func);
    }

    #[test]
    #[should_panic]
    fn test_register_twice() {
        let mut tasks = Tasks::new();
        async fn func(_s: String) {}
        tasks.add_task("test", func);
        tasks.add_task("test", func);
    }
}
