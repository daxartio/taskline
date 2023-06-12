# Taskline

[![Crates.io](https://img.shields.io/crates/v/taskline.svg)](https://crates.io/crates/taskline)
[![Docs.rs](https://docs.rs/taskline/badge.svg)](https://docs.rs/taskline)
[![CI](https://github.com/daxartio/taskline/workflows/CI/badge.svg)](https://github.com/daxartio/taskline/actions)
[![Coverage Status](https://coveralls.io/repos/github/daxartio/taskline/badge.svg?branch=main)](https://coveralls.io/github/daxartio/taskline?branch=main)

WIP

The library allows to create scheduled tasks via Redis for Rust.

```rust
async fn handle(request: String) {}

producer.schedule("Hello!".to_string(), now() + 1000.).await;
```

That means the handle will be run in 1 second.

You can customize a format of an event for redis. Write your wrapper over [RedisBackend](src/backends/redis.rs).

## Installation

### Cargo

```
cargo add taskline
```

## License

* [MIT LICENSE](LICENSE)

## Contribution

[CONTRIBUTING.md](CONTRIBUTING.md)
