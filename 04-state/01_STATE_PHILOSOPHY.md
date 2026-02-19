# 1 — Application State Philosophy

## Stateless handler design

Handlers are pure in the sense that they don’t rely on global mutable state. All inputs are explicit: the request (path, query, body, headers) and **injected state** (State\<T\>). So the same handler code runs for every request; only the request and the shared state differ. That makes testing and reasoning easier: you can pass a mock state in tests.

## Why dependency injection exists

Real backends need shared resources: config, DB pool, HTTP client, caches. Axum doesn’t use globals. You attach one value to the Router with `.with_state(state)` and every handler that needs it takes `State<S>`. So “shared state” is typed and visible in the handler signature. The compiler enforces that the Router has the right state type when you use `State<S>` in a handler.
