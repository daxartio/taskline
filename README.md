# Taskline

[![Crates.io](https://img.shields.io/crates/v/taskline.svg)](https://crates.io/crates/taskline)
[![CI](https://img.shields.io/github/actions/workflow/status/daxartio/taskline/ci.yml?branch=main)](https://github.com/daxartio/taskline/actions)
[![Docs.rs](https://docs.rs/taskline/badge.svg)](https://docs.rs/taskline)
<!-- [![Coverage Status](https://coveralls.io/repos/github/daxartio/taskline/badge.svg?branch=main)](https://coveralls.io/github/daxartio/taskline?branch=main) -->

Taskline is a Rust library for scheduling tasks via Redis.

## Overview

Taskline provides a simple way to schedule and process tasks asynchronously. It follows a producer-consumer model, where a producer schedules tasks to be executed at a specific time, and a consumer retrieves and processes them.

## Use Cases

Taskline is ideal for applications that require deferred execution, such as:

- Scheduling emails to be sent at a later time.
- Sending notifications to users at a specific moment.
- Any background job that needs time-based execution.

## Features

- [x] Send/receive tasks in Redis
- [x] Delayed tasks
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

## Task Auto-Deletion

### Default Behavior

By default, Taskline automatically deletes tasks from storage after they are processed. This is the recommended approach for most use cases, as it ensures tasks are not executed multiple read.

### Disabling Auto-Deletion

If you prefer to manually manage task deletion, you can disable auto-delete by setting `autodelete=false`. However, this should only be used with a single consumer to avoid duplicate processing. If multiple consumers are involved, consider using a distributed lock mechanism like [redlock](https://redis.com/glossary/redlock/). For more details, see [Distributed Locks with Redis](https://redis.io/docs/manual/patterns/distributed-locks/).

To manually remove a processed task, use: `Taskline::delete`.

### Recommendation

If your use case allows, it is recommended to keep `autodelete=true`, as it simplifies task management and reduces configuration overhead. However, be aware that in the event of an application crash, tasks may be lost before they are processed.

## License

* [MIT LICENSE](LICENSE)

## Contribution

[CONTRIBUTING.md](CONTRIBUTING.md)
