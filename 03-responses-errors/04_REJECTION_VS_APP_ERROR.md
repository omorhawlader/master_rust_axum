# 4 — Rejection vs Application Error

## Difference

- **Rejection:** Comes from an **extractor** (e.g. invalid JSON, missing path segment, wrong Content-Type). The handler is never called. Axum turns the rejection into a response (often 400, 404, 422). So “request shape wrong” = rejection.
- **Application error:** Comes from **your handler or service** (e.g. “user not found”, “email already taken”). You return `Err(app_error)`. The error type implements `IntoResponse`, so Axum turns it into a response. So “business rule failed” = application error.

## Mapping layer pattern

You can add a layer or fallback that converts **rejections** (e.g. JsonRejection, PathRejection) into the same JSON shape as your application errors, so clients always get `{ "error": "..." }` (or similar) regardless of whether the failure was at extraction or in the handler.
