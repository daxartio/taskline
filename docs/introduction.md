# Introduction

The library allows to create scheduled tasks via Redis for Rust.

## How does it work?

Taskline revolves around the concept of a task. A task is a unit of work that is requested by a producer to be completed by a consumer / worker.
