# Day 5 real-world project (topics: Day 1–5)

**Scope:** Foundation, extractors, responses/errors, state, Tower layers (trace, timeout).

## Project: API with observability and timeout

The parent crate: routes plus TraceLayer (request/response logging) and TimeoutLayer (1s). /slow intentionally exceeds the timeout so you see timeout behavior.

- **GET /** — hello  
- **GET /slow** — sleeps 2s → timeout (504 or similar)

## What you practiced

- Layering: TraceLayer + TimeoutLayer; order and behavior.

## Extend (optional)

- Add CORS for a specific origin.
- Add a custom middleware that sets a request ID and logs it in the handler (via extension + extractor).
