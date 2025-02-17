# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.10.0 (2025-02-13)

### Refactor

- simplify code (#2)

## 0.9.5 (2025-01-22)

## 0.9.4 (2024-10-23)

## 0.9.3 (2024-07-11)

## 0.9.2 (2024-04-29)

### Refactor

- add with_capacity

## 0.9.1 (2023-11-04)

### Fix

- fix interval between tasks

## 0.9.0 (2023-09-22)

### Feat

- add new functions - now, poll_tasks

### Fix

- return a value for poll_tasks

## 0.8.1 (2023-09-21)

### Fix

- make the MemoryBackend type as a generic type

## 0.8.0 (2023-09-20)

### Feat

- add the memory backend

## 0.7.0 (2023-08-19)

### Feat

- use & for args

### Refactor

- don't use async trait methods

## 0.6.0 (2023-07-21)

### Feat

- add is_redis_version_ok method

## 0.5.1 (2023-07-14)

### Refactor

- add committer instead of the delete method

## 0.5.0 (2023-07-12)

### Feat

- add autodelete arg

## 0.4.2 (2023-07-03)

### Fix

- update docs

## 0.4.1 (2023-06-25)

### Fix

- **docs**: update code's documentation

## 0.4.0 (2023-06-25)

### Feat

- add JsonRedisBackend

## 0.3.0 (2023-06-18)

### Feat

- return error

## 0.2.0 (2023-06-15)

### Feat

- simpify api

## 0.1.1 (2023-06-13)

### Fix

- allow to write of queue_key as String or &str

## 0.1.0 (2023-06-12)

### Feat

- add generics
- add loop instead of tasks
- use async await
- add scheduled tasks
- init

### Fix

- add delay between requests
