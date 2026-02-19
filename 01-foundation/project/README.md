# Day 1 real-world project (topics: foundation only)

**Scope:** Router, handlers, `IntoResponse`, fallback, `axum::serve`. No extractors beyond `Path` (used once), no state, no error types.

## Project: Minimal public API

The crate in the parent folder (`../`) **is** this project: a small API that could serve as the base of a product API.

- **GET /** — welcome
- **GET /api/health** — health check (JSON)
- **GET /api/users/{id}** — path parameter
- **GET /api/custom** — custom `IntoResponse`
- **GET /api/status** — status-only response
- **Any other path** — fallback (e.g. “Not found”)

## What you practiced

- One Router with `route`, `nest`, `fallback`
- Handlers with different return types (`&str`, `Json`, `StatusCode`, custom type)
- Request flow: Tokio → Hyper → Tower (Router) → handler → `IntoResponse` → response

## Extend it (optional)

- Add `GET /api/version` returning a JSON object with a version string.
- Add a route that returns `StatusCode::NO_CONTENT` for `DELETE /api/cache` (no body).
- Keep only concepts from Day 1 (no State, no Query/Json body extractors yet).
