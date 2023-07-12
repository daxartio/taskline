# Quick Start

Taskline is provided as the a library on [crates.io](https://crates.io/crates/taskline). To get started, add taskline as a dependency to your project.

```
cargo add taskline
```

## Example

The library provides a asynchronous code for interacting with Redis. You can use `tokio` or `async-std` as the runtime.

First of all, you need to create a `RedisBackend` instance. You can do this by using the `RedisBackend::new` method or `RedisBackendConfig` struct.

After that, you need to create a consumer and a producer. These are simple structs for more comfortable interaction with the library. You can create them using the `Consumer::new` and `Producer::new` methods.

You can look at the example below.

```rust,no_run,noplayground
{{#include ../examples/redis.rs}}
```

More examples can be found [here](https://github.com/daxartio/taskline/tree/main/examples).
