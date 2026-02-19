# Day 4 real-world project (topics: Day 1–4)

**Scope:** Foundation, extractors, responses/errors, State, Arc, shared mutable (atomic).

## Project: Config API with shared state

The parent crate: one route that reads shared state (app name + request counter) and returns JSON.

- **GET /config** — returns app name and total request count (state is shared and updated per request).

## What you practiced

- with_state(Arc\<AppState\>), State\<Arc\<AppState\>>, interior mutability (AtomicU64).

## Extend (optional)

- Add in-memory cache (e.g. HashMap in a Mutex) and a route to get/set by key.
- Simulate a “pool”: a struct that holds a fixed number of “slots” and a method to acquire/release (no real DB yet).
