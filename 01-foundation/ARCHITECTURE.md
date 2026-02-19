# 1️⃣ Axum Ecosystem Architecture (Master-Level)

## 1.1 Axum Position in Rust Web Stack

### Axum vs Hyper

- **Hyper** is a low-level HTTP library: it parses HTTP, manages connections, and gives you `Request`/`Response` and async body types. It does **not** do routing, extractors, or “handler” abstraction.
- **Axum** sits **on top of Hyper**. It uses Hyper for the transport and HTTP protocol; it adds routing, extractors, `IntoResponse`, and the idea of “handlers” as async functions. So: Hyper = protocol + transport; Axum = application layer (routes + handlers + Tower integration).

### Axum vs Actix

- **Actix** is actor-based and has its own runtime and abstractions. It’s very fast and has its own middleware and extractor system.
- **Axum** is **not** actor-based. It’s built on **Tower** and **Hyper**, so:
  - Everything composes as **Tower `Service`s** (Router, middleware, handlers).
  - You get the same mental model as the rest of the Tokio/Hyper/Tower ecosystem.
  - Middleware and utilities from Tower/tower-http work directly (timeouts, tracing, compression, etc.).

### Why Axum Is Built on Tower

- **Tower** defines the `Service` trait: `fn call(&mut self, req: Request) -> Future<Output = Result<Response, E>>`. That’s the core abstraction for “something that takes a request and produces a response.”
- Axum’s **Router** implements `Service`. So the “app” is literally a Tower service. Handlers are turned into services; middleware are Tower layers. That’s why:
  - You can wrap the app in any Tower middleware.
  - You can test by calling the service with a request.
  - There’s no second “middleware system”—Tower is the middleware system.

### Composition-Based Design

- You don’t configure “the framework”; you **compose** values: `Router::new().route(...).nest(...).layer(...)`. Each step is a value; the final value is the service that gets passed to `axum::serve`. No global state or hidden configuration.

---

## 1.2 Internal Dependency Tree

```
your app (Router + handlers)
    ↓
axum (routing, extractors, IntoResponse, serve)
    ↓
tower (Service, Layer)
    ↓
hyper (HTTP/1, HTTP/2, Request/Response, body)
    ↓
tokio (async runtime: TCP, timers, task scheduling)
```

- **Tokio**: Provides the async runtime. `axum::serve(listener, app)` runs on Tokio; each accepted connection is handled by async tasks. No blocking I/O in the hot path.
- **Hyper**: Implements the HTTP layer (parsing, framing, body streaming). It produces `http::Request`/`http::Response` and works with Tower’s `Service` interface.
- **Tower**: Defines `Service` and `Layer`. Axum’s Router and handler-backed services implement `Service`; middleware are added via `Layer`. So the “stack” is: Layer(s) around a Service (your Router).

Request flow at a high level: **TCP (Tokio) → Hyper (parse request) → Tower service call (Router) → route matching → handler execution → IntoResponse → Hyper (write response).**

---

## 1.3 Request Lifecycle (Very Important)

1. **Incoming TCP**  
   The OS delivers a new TCP connection to the listening socket.

2. **Tokio accept**  
   `TcpListener::accept()` yields a `TcpStream`. Tokio spawns (or assigns) a task to handle that connection.

3. **Hyper parse**  
   Hyper reads bytes from the stream, parses HTTP (request line, headers, body), and builds an `http::Request<Body>`.

4. **Tower service call**  
   That request is passed to your top-level `Service` (the Router): `service.call(request).await`. So the “entry point” of your app is literally `Service::call`.

5. **Router**  
   The Router matches method + path. It may delegate to a nested Router (e.g. after `nest("/api", ...)`) or to a `MethodRouter` (get/post/…). The selected handler is itself a Tower service (a “handler service” that Axum built from your async function).

6. **Handler execution**  
   The handler service’s `call` runs: it may run extractors (FromRequest/FromRequestParts), then call your async function. Your function returns something that implements `IntoResponse`.

7. **IntoResponse conversion**  
   Axum calls `into_response()` on the return value and gets an `http::Response`. That may set status, headers, and body (e.g. JSON, plain text).

8. **Response write-back**  
   Hyper takes that `Response`, serializes it back to bytes, and writes to the `TcpStream`. Tokio handles the async write and connection lifecycle.

So end-to-end: **TCP → Tokio → Hyper (parse) → Tower (Router → handler service) → your handler → IntoResponse → Hyper (write) → TCP.**

---

## Mental Model

- **Axum** = routing + extractors + response building on top of **Hyper + Tower**.
- **Router** = Tower `Service` that dispatches by path/method to inner services (handlers or nested routers).
- **Handler** = async function turned into a Tower service; its return type must be `IntoResponse`.
- **Request lifecycle** = one linear chain: accept → parse → service call → handler → into_response → write.

All crate code that demonstrates this lives in `src/`: `router_builder::app()` builds the Router; `run::serve` binds the listener and calls `axum::serve(listener, app)`.
