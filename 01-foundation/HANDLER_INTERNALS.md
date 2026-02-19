# 4️⃣ Handler System Internals

## 4.1 What Makes a Function a Handler?

- **Async function requirement**  
  A handler must be an `async fn` (or a function that returns `impl Future`). Axum runs it on the Tokio runtime; the framework awaits the future. Sync functions that block would block the task and hurt scalability, so Axum is async-only for handlers.

- **Trait bounds behind handler**  
  Axum uses a `Handler` trait (or equivalent internal bounds) that is implemented for async functions whose:
  - Arguments implement the right extractors (e.g. `FromRequest` or `FromRequestParts`).
  - Return type implements `IntoResponse`.  
  So “handler” = async function that fits these constraints. The compiler enforces this when you pass the function to `get(handler)` / `post(handler)`.

---

## 4.2 Handler Trait

- **How Axum converts function into Service**  
  For each handler, Axum generates a small Tower `Service` that: (1) runs extractors (FromRequest/FromRequestParts) to build the handler’s arguments, (2) calls the async function with those arguments, (3) takes the return value and calls `into_response()`, then returns that response. So the “handler” you write is wrapped in a service that does extraction + call + response conversion.

- **Generic constraints overview**  
  The handler’s argument types must be “extractable” from the request (or from request parts); the return type must be `IntoResponse`. Those are the main constraints. State is special: if you use `State<S>`, the Router must be built with `.with_state(s)` and the type must match.

---

## 4.3 Return Type System

- **IntoResponse trait**  
  Anything that implements `IntoResponse` can be returned from a handler. Axum calls `.into_response()` to get an `http::Response`. Built-in impls include: `&'static str`, `String`, `StatusCode`, `(StatusCode, T)` where `T: IntoResponse`, `Json<T>` (with serde), `Response`, and others.

- **Automatic conversion**  
  You don’t call `.into_response()` yourself; the handler service does it. So you can return `"hello"`, `StatusCode::OK`, `(StatusCode::OK, Json(data))`, or a custom type that implements `IntoResponse`.

- **Why flexible return types work**  
  Because the only requirement is `IntoResponse`, you can return different types from different handlers (plain text, JSON, custom struct, etc.) and the compiler checks that each return type implements the trait. In this crate, `handlers::root` returns `&'static str`, `handlers::health` returns `(StatusCode, Json<HealthBody>)`, and `handlers::custom_response` returns a custom `PlainText` type that implements `IntoResponse`.

---

## 4.4 Compile-Time Validation

- **Wrong extractor fails compile**  
  If you use an extractor that requires something the request doesn’t have (e.g. wrong body type), or you use `State<S>` but the router has no state, the handler’s type doesn’t satisfy the `Handler` bounds and you get a compile error. So many mistakes are caught at compile time.

- **Missing state fails compile**  
  `State<S>` is part of the handler’s type. The Router is typed with the same state (or no state). If you add `State<AppState>` to a handler but don’t call `.with_state(...)` on the Router, the types don’t match and the code doesn’t compile.

See `src/handlers.rs` for examples of different return types and the custom `PlainText` + `IntoResponse` impl.
