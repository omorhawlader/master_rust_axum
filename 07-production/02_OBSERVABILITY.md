# 2 — Observability

## tracing crate

Use `tracing` for logs and spans. In handlers and middleware: `info!(...)", `error!(...)`, and span to group related work. Structured fields (e.g. request_id, user_id) make logs queryable.

## tracing-subscriber

`tracing_subscriber::fmt().init()` installs a subscriber that prints logs. Use `EnvFilter` (e.g. `RUST_LOG=info,myapp=debug`) to control level. In production you might use a layer that exports to a collector (e.g. OpenTelemetry).

## Request ID pattern

Generate a request ID (e.g. UUID) in a middleware, set it as an extension, and log it in every message for that request. So you can trace one request across logs. Extractors can read the request ID from extensions if handlers need it.
