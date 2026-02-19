# 6️⃣ Minimal Rust Concepts Required Inside Axum

(Only what you need to read Axum code and docs.)

## 6.1 Traits (IntoResponse, Service)

- **Trait** = a contract: types that implement it provide certain methods. Axum relies on:
  - **IntoResponse**: one method `into_response(self) -> Response`. Handlers return any `T: IntoResponse`; the framework calls this to get the HTTP response.
  - **Service** (from Tower): `poll_ready` and `call(request) -> Future<Output = Result<Response, E>>`. Router and handler-backed services implement this so they can be called with a request and produce a response. You don’t implement these yourself for normal handlers; you implement `IntoResponse` only when you want a custom response type.

## 6.2 Generics in Handler Signatures

- Handlers are generic over their argument and return types. When you write `async fn get_user(Path(id): Path<u32>) -> impl IntoResponse`, the compiler checks that `Path<u32>` can be extracted and that the return type implements `IntoResponse`. The router and the handler are type-checked together (e.g. state type must match). So “generics” here just means: the handler’s types are inferred and checked at compile time.

## 6.3 Lifetimes When Returning References

- If you return `&'static str`, that’s a reference that lives for the whole program, so no lifetime issue. If you ever return a reference that depends on the request or handler state (e.g. `&str` from a buffer), you’d need to tie its lifetime to the request or the future. For Day 1, returning owned types (`String`, `Json<T>`, etc.) or `&'static str` is enough and avoids lifetime complexity.

## 6.4 Send + Sync Requirement Overview

- Futures that the runtime moves between threads (e.g. in a multi-threaded Tokio runtime) must be `Send`. Data shared across await points in that future must be `Send` (and often `Sync` if shared with other tasks). Axum and Tower require handler futures to be `Send` so they can run on a multi-threaded runtime. In practice: don’t hold non-`Send` types across an `.await` in a handler; use `Arc` for shared state so it’s `Send + Sync`.
