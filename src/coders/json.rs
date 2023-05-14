#[macro_export]
macro_rules! json_fn {
    ($f:expr) => {{
        fn inner(s: String) {
            match serde_json::from_str(&s) {
                Ok(v) => $f(v),
                _ => (),
            }
        }
        inner
    }};
}

pub use json_fn;

#[macro_export]
macro_rules! json_call {
    ($client:ident, $task_name:ident, $queued_task:expr) => {{
        call!(
            $client,
            $task_name,
            serde_json::to_string(&$queued_task).unwrap()
        );
    }};
}

pub use json_call;

#[macro_export]
macro_rules! json_schedule {
    ($client:ident, $task_name:ident, $queued_task:expr, $time:expr) => {{
        schedule!(
            $client,
            $task_name,
            serde_json::to_string(&$queued_task).unwrap(),
            $time
        );
    }};
}

pub use json_schedule;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[test]
    fn test_json() {
        #[derive(Deserialize)]
        struct TestJson {
            _a: String,
        }
        fn func(_s: TestJson) {}
        let wrapped = json_fn!(func);
        wrapped("{\"_a\": \"test\"}".to_string());
    }
}
