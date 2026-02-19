# 3 — Extractor Order & 4 — FromRequest Deep Dive

## Extractor execution order

- Axum evaluates extractors in the order they appear in the handler signature. **RequestParts** extractors run first (Path, Query, State, headers): they receive the request without the body. The **body** is consumed by at most one extractor (e.g. `Json<T>`). So any extractor that needs the body must be last, because after it runs the body is gone. Putting two body extractors in one handler is a type error or would fail at compile time.

## Rejection propagation

- If any extractor fails, it returns a **Rejection** (a type that encapsulates “why extraction failed”). Axum doesn’t call the handler; it converts the rejection into an HTTP response (e.g. 400, 404, 422). So “validation” at the extractor level is “reject or succeed”; the handler only sees successful extractions.

## FromRequest trait

- **Purpose:** A type that can be extracted from a full `Request` implements `FromRequest`. It has an associated type for the rejection and an async `from_request` that takes the request and returns `Result<Self, Rejection>`.
- **FromRequestParts:** Same idea but for types that don’t need the body. They receive `RequestParts` and return `Result<Self, Rejection>`. Path, Query, State, and header-based extractors use this so they can run before the body is consumed.

## Implementing a custom extractor

- Implement `FromRequest` (or `FromRequestParts`) for your type. In `from_request` you read what you need from the request (or parts), validate, and return `Ok(value)` or `Err(your_rejection)`. Your rejection type must be convertible to a response (Axum provides ways to map rejections to status codes and bodies). Then use your type as a handler parameter like any other extractor.
