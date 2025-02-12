# Taskline

[![Crates.io](https://img.shields.io/crates/v/taskline.svg)](https://crates.io/crates/taskline)
[![CI](https://img.shields.io/github/actions/workflow/status/daxartio/taskline/ci.yml?branch=main)](https://github.com/daxartio/taskline/actions)
[![Docs.rs](https://docs.rs/taskline/badge.svg)](https://docs.rs/taskline)
<!-- [![Coverage Status](https://coveralls.io/repos/github/daxartio/taskline/badge.svg?branch=main)](https://coveralls.io/github/daxartio/taskline?branch=main) -->

The library allows for creating scheduled tasks via Redis for Rust.

You can customize a format of an event for redis. Write your wrapper over [RedisBackend](src/backends/redis.rs). See [redis_json backend](src/backends/redis_json.rs).

## Features

- [x] Send/receive tasks in Redis
- [x] Delayed tasks
- [x] Support json
- [x] Deleting from a storage after handling
- [ ] Support Redis Cluster
- [ ] Metrics

## Requirements

- Redis 6.2.0 or higher

## Installation

### Cargo

```
cargo add taskline
```

## License

* [MIT LICENSE](LICENSE)

## Contribution

[CONTRIBUTING.md](CONTRIBUTING.md)
