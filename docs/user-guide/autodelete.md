# Autodelete

## Deleting from a storage after handling

If you want to delete a task from storage after handling, you can use `RedisJsonBackend` or `RedisBackend` with `autodelete=false` parameter. It's safe to use it only with one consumer. If you have more than one consumer, you can use distributed lock by redis. It's also named as [redlock](https://redis.com/glossary/redlock/). See [Distributed Locks with Redis](https://redis.io/docs/manual/patterns/distributed-locks/).

Don't forget to delete a task explicitly from storage after handling. See `Committer::commit`.

It's experimental implementation. In the future, it will be implemented more comfortable way.

## Recomendation

I recomend to use `autodelete=True`, if it fits to you. This way is simple to undestanding and it do not require extra configurations.
But you need to know that your tasks will not be handling again if your application has an error.

## Example

```rust,no_run,noplayground
{{#include ../../examples/redis_json_autodelete.rs}}
```
