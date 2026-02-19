# Day 2 real-world project (topics: Day 1 + Day 2)

**Scope:** Router, handlers, Path, Query, Json. No State, no custom error types yet.

## Project: Items API

The parent crate (`../`) is the project: a minimal CRUD-style API using only foundation + extractors.

- **GET /items/{id}** — path param
- **GET /search?q=...&limit=...** — query params (optional)
- **POST /items** — JSON body (name, optional count)

## What you practiced

- Path, Query, Json; extractor order (body last); rejection when JSON or path is invalid.

## Extend (optional)

- Add `GET /items` that returns a list (hardcoded or from in-memory state in a later day).
- Add a query struct with validation: e.g. `limit` must be 1..=100 or reject.
