# Introduction

The library allows to create scheduled tasks via Redis for Rust.

## How does it work?

Taskline revolves around the concept of a task. A task is a unit of work that is requested by a producer to be completed by a consumer / worker.

A producer can schedule a task to be completed at a specific time in the future. A consumer can then fetch the task and complete it.

There are backends for consumers and producers, which must implement the `DequeuBackend` and `EnqueuBackend` traits. Right now, there is only one backend, which is Redis.

## When should I use Taskline?

Taskline is a good fit for applications that need to schedule work to be done in the future. For example, Taskline is a good fit for:

- Scheduling emails to be sent in the future
- Scheduling a notification to be sent to a user in the future
