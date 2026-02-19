# 2️⃣ Modern Project Setup (Current Axum Way)

## 2.1 Cargo Configuration

- **edition = "2021"**  
  Use Rust 2021. No need for older editions.

- **Minimal dependency setup**  
  For Day 1 you only need:
  - `axum` (with features you use: e.g. `json`, `form`, `query`, `macros`, `matched-path`, `original-uri`).
  - `tokio` with `full` or at least `macros` + `rt-multi-thread` + `net` for `#[tokio::main]` and `TcpListener`.
  - `tower` for tests and for understanding the Service abstraction (optional in Cargo for running the app, but part of Axum’s design).
  - `serde` / `serde_json` only if you use `Json<T>` or serialize/deserialize in handlers.

- **Feature flags strategy**  
  Axum uses feature flags to avoid pulling in unused code. Enable only what you need (e.g. `json`, `form`, `query`, `macros`). The crate in this folder enables a small set so you can see route + handler + JSON without extra deps.

---

## 2.2 Binary Crate Structure for Backend

- **main.rs**  
  Single entry: `#[tokio::main] async fn main()` that parses address (or uses default), builds the `Router` (via a dedicated function or module), and runs `axum::serve(listener, app)`. Keep it short: “wire and run.”

- **Separation of router builder**  
  The `Router` is built in a dedicated module (e.g. `router_builder::app()`). That way main stays minimal and you can test or reuse the same app in tests/tools.

- **Module separation**  
  - `handlers`: async functions that are used as handlers (return `impl IntoResponse` or concrete types that implement it).
  - `router_builder`: composition of routes (route, nest, merge, fallback) and any route-level config.
  - `run`: bind address, create listener, call `axum::serve`.  
  This keeps “what handles a request,” “how routes are composed,” and “how the server is started” clearly separated.

---

## 2.3 Compile-Time Guarantees

- **Why types catch route mistakes**  
  Routes are registered with a specific path and method. Handlers are typed: extractors (e.g. `Path<T>`) must match what the route can provide. If you use `Path<(u32, u32)>` you need two segments; the type system doesn’t enforce path string shape at compile time, but the extractor’s type and the number of segments must align or you get runtime rejection. State (e.g. `State<S>`) is enforced: if the Router has no `.with_state(...)`, a handler that takes `State<S>` does not compile.

- **Why handler signature matters**  
  A “handler” in Axum is an async function whose arguments implement the right extractors and whose return type implements `IntoResponse`. The compiler checks that the function can be used as a `Handler` for the given route. Wrong extractors (e.g. `Json` when body is not JSON) or missing state show up as type errors when you pass the function to `get(handler)` / `post(handler)`.

- **Why wrong return type fails compile**  
  The handler must return something that implements `IntoResponse`. If you return a type that doesn’t (and don’t wrap it), you get a compile error. So “forgot to convert to response” is caught at compile time.

The crate in this directory is structured so you can see these guarantees: `src/main.rs`, `src/run.rs`, `src/router_builder.rs`, `src/handlers.rs`.
