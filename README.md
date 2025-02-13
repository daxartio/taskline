# Taskline

[![Crates.io](https://img.shields.io/crates/v/taskline.svg)](https://crates.io/crates/taskline)
[![CI](https://img.shields.io/github/actions/workflow/status/daxartio/taskline/ci.yml?branch=main)](https://github.com/daxartio/taskline/actions)
[![Docs.rs](https://docs.rs/taskline/badge.svg)](https://docs.rs/taskline)
<!-- [![Coverage Status](https://coveralls.io/repos/github/daxartio/taskline/badge.svg?branch=main)](https://coveralls.io/github/daxartio/taskline?branch=main) -->

The library allows for creating scheduled tasks via Redis for Rust.

You can customize a format of an event for redis. Write your wrapper over [RedisBackend](src/backends/redis.rs). See [redis_json backend](src/backends/redis_json.rs).

## How does it work?

Taskline revolves around the concept of a task. A task is a unit of work that is requested by a producer to be completed by a consumer / worker.

A producer can schedule a task to be completed at a specific time in the future. A consumer can then fetch the task and complete it.

There are backends for consumers and producers, which must implement the `DequeuBackend` and `EnqueuBackend` traits. Right now, there is only one backend, which is Redis.

## When should I use Taskline?

Taskline is a good fit for applications that need to schedule work to be done in the future. For example, Taskline is a good fit for:

- Scheduling emails to be sent in the future
- Scheduling a notification to be sent to a user in the future


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

# Autodelete

## Deleting from a storage after handling

If you want to delete a task from storage after handling, you can use `RedisJsonBackend` or `RedisBackend` with `autodelete=false` parameter. It's safe to use it only with one consumer. If you have more than one consumer, you can use distributed lock by redis. It's also named as [redlock](https://redis.com/glossary/redlock/). See [Distributed Locks with Redis](https://redis.io/docs/manual/patterns/distributed-locks/).

Don't forget to delete a task explicitly from storage after handling. See `Committer::commit`.

It's experimental implementation. In the future, it will be implemented more comfortable way.

## Recommendation

I recommend to use `autodelete=True`, if it fits to you. This way is simple to understanding and it do not require extra configurations.
But you need to know that your tasks will not be handling again if your application has an error.

# Formats of tasks

## A format of a task for sending and receiving via Redis

Actually, Taskline uses a format of a backend. You can use any format which you want.

There are two formats of a task for sending and receiving via Redis which are implemented in the library:

- JSON
- String

## License

* [MIT LICENSE](LICENSE)

## Contribution

[CONTRIBUTING.md](CONTRIBUTING.md)
