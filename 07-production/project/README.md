# Day 7 real-world project (topics: Day 1–7)

**Scope:** Full stack: foundation, extractors, errors, state, middleware, handler/service/repo, testing, tracing.

## Project: Production-style API

The parent crate: minimal app with health route, tracing, and an integration test that calls the Router with oneshot. This is the “final” shape: one place that ties all concepts together.

- **GET /health** — health check; traced.
- **cargo test** — health_returns_200.

## What you practiced

- Testing via ServiceExt::oneshot; tracing; structure ready for graceful shutdown and scaling.

## Extend (optional)

- Add graceful shutdown with tokio::signal::ctrl_c.
- Add a request ID middleware and log it in handlers.
- Add one more route and an integration test for it.
