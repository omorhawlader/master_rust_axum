# 7️⃣ Production Mental Model

## 7.1 Stateless Handler Philosophy

- Handlers are pure in the sense that they don’t rely on global mutable state. All inputs come from the request (path, query, headers, body) and from explicit injectors (State, extensions). So each request is self-contained: same handler, same code path; only the request and injected state differ. That makes reasoning and testing easier.

## 7.2 Why State Injection Exists (Preview)

- Real backends need shared data: DB pool, config, HTTP client, etc. Axum doesn’t use globals; you inject state with `.with_state(state)` and handlers take `State<S>`. So “shared state” is explicit and typed. We’ll go deeper in later days; for Day 1 it’s enough to know that state is optional and injected at the Router level.

## 7.3 How Axum Avoids Hidden Magic

- No global router or config; you build a `Router` value and pass it to `serve`. No thread-local or global “current request” that middleware and handlers implicitly read; you use extractors and state. So “where does this value come from?” is always answerable: from the request (extractor) or from the app state (State/Extension). Types enforce what’s available.

## 7.4 Comparison with Express/FastAPI

- **Express**: Middleware and route handlers receive `(req, res)`; you mutate `res` and call `next()`. State is often attached to `req` or globals. Axum: no `res` object; you return a value that becomes the response. State is explicit (State/Extension). Middleware are Tower layers; they wrap the next service and call it with the request.
- **FastAPI**: Handlers are functions; dependency injection and path/query/body are declared by type. Axum is similar: extractors (Path, Query, Json, State) play the role of “dependencies” from the request or app state. Axum doesn’t have a separate “dependency” system; extractors are the mechanism. Return type is “response model” in FastAPI; in Axum it’s any `IntoResponse`.

So: explicit state, no hidden request/response object, composition via Router and Tower, and everything is a Service under the hood.
