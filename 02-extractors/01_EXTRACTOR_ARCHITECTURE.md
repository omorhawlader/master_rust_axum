# 1 — Extractor System Architecture

## What is an extractor?

An **extractor** is a type that takes data from the HTTP request and turns it into a Rust value your handler can use. In the handler signature you write the type (e.g. `Path<u32>`, `Query<SearchParams>`, `Json<Body>`); Axum calls the corresponding trait implementation to build that value from the request. If extraction fails (e.g. invalid JSON, missing path segment), the request is rejected and the handler is never called.

## Type-driven request parsing

You don’t manually read query strings or parse JSON. You declare what you need by type: `Query<T>`, `Json<T>`, `Path<T>`. The compiler and the trait system ensure that (1) `T` can be deserialized from that part of the request, and (2) the handler only runs when extraction succeeds. So “request parsing” is driven by the types in your handler.

## Compile-time request validation

Some validation is at compile time: e.g. if you use `State<S>`, the Router must have `.with_state(...)` with a compatible type. Path segment count and extractor type (e.g. `Path<(u32, u32)>` for two segments) are checked by the routing and `Handler` bounds. Deserialization (e.g. JSON shape, query string format) happens at runtime; failure produces a **rejection** (e.g. 400 Bad Request or 422 Unprocessable Entity) instead of calling the handler.

## RequestParts vs Request

- **Request** = full HTTP request, including the body. Used by extractors that need the body (e.g. `Json<T>`). The body is a stream that can only be consumed once.
- **RequestParts** = request without the body (method, URI, headers). Used by extractors that don’t need the body (e.g. `Path`, `Query`, `State`, headers).

Axum runs “parts” extractors first (they get `RequestParts`), then at most one “body” extractor (it gets the full `Request` and consumes the body). That’s why **body extractors (e.g. Json) must be last** in the handler: the body is taken once and is no longer available for other extractors.
