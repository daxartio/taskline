use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct QueuedTask {
    pub name: String,
    pub request: String,
}

pub struct Tasks<T>
where
    T: Fn(String) -> (),
{
    pub tasks: Vec<(&'static str, T)>,
}

impl<T> Tasks<T>
where
    T: Fn(String) -> (),
{
    pub fn new() -> Tasks<T> {
        Tasks { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, name: &'static str, func: T) {
        if self.tasks.iter().any(|(n, _)| n == &name) {
            panic!("Task name '{}' already exists", name);
        }
        self.tasks.push((name, func));
    }
}

#[macro_export]
macro_rules! task {
    ($tasks:ident, $task_name:ident, $f:expr) => {{
        $tasks.add_task(stringify!($task_name), $f);
    }};
}

pub use task;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut tasks = Tasks::new();
        fn func(_s: String) {}
        task!(tasks, test, func);
    }

    #[test]
    #[should_panic]
    fn test_register_twice() {
        let mut tasks = Tasks::new();
        fn func(_s: String) {}
        task!(tasks, test, func);
        task!(tasks, test, func);
    }
}
