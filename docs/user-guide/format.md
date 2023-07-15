# Formats of tasks

## A format of a task for sending and receiving via Redis

Actualy, Taskline uses a format of a backend. You can use any format which you want.

There are two formats of a task for sending and receiving via Redis which are implemented in the library:

- JSON
- String

## Example

```rust,no_run,noplayground
{{#include ../../examples/redis_json.rs}}
```
