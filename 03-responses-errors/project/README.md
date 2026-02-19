# Day 3 real-world project (topics: Day 1 + 2 + 3)

**Scope:** Foundation, extractors, IntoResponse, Result in handlers, AppError, thiserror.

## Project: Resources API with errors

The parent crate is the project: one route that returns either success JSON or a structured error (validation).

- **GET /resources/{id}** — returns resource or AppError (e.g. validation for id=0).

## What you practiced

- Centralized error enum with IntoResponse; Result in handler; consistent error JSON.

## Extend (optional)

- Add a “not found” case (e.g. id > 100) and return AppError::NotFound with 404.
- Add a handler that returns different status codes (200, 201, 204) with different body types.
