# 5️⃣ Async Execution Model (Axum Context Only)

## 5.1 Why Axum Is Fully Async

- **No blocking model**  
  Handlers are async; I/O (TCP, file, DB, HTTP client) is done via async APIs (Tokio, hyper, etc.). The runtime can run many concurrent tasks on a small number of threads. Blocking the thread (e.g. with a sync mutex or blocking I/O) would hurt throughput, so Axum doesn’t encourage or assume a blocking model.

- **Scalability reason**  
  Many connections can be handled with few threads because each task yields when it hits `.await`. So you get high concurrency without a thread-per-request model. This is the same reason Node.js or async Python (asyncio) scale well for I/O-bound servers.

---

## 5.2 Tokio Runtime Integration

- **#[tokio::main]**  
  Expands to a `main` that builds a Tokio runtime and runs your `async fn main()` on it. So when you run the binary, Tokio is the async runtime; `axum::serve` and all handlers run on that runtime.

- **Multi-thread runtime default**  
  With `tokio::main` and default features, Tokio uses a multi-threaded work-stealing scheduler. So many tasks can run concurrently across threads; when one task awaits, others can run on the same thread.

- **Cooperative scheduling concept**  
  A task runs until it hits `.await`; then it yields. The runtime can run another task on that thread. So you must not do long CPU work or blocking calls without `.await` in between, or you stall the thread. Axum doesn’t add its own scheduler; it relies entirely on Tokio.

---

## 5.3 Handler Execution Flow

- **Future returned**  
  Your handler is an async function, so it returns `impl Future<Output = R>` where `R: IntoResponse`. The handler service doesn’t run your function synchronously; it gets a future and awaits it.

- **Runtime polling**  
  When the handler service awaits that future, Tokio may poll it. If the future is pending (e.g. waiting on I/O), the task is parked and the thread can run other tasks. When the I/O is ready, the task is woken and polled again.

- **Completion path**  
  When the future completes, the handler service gets the `R` value, calls `into_response()`, and returns the response. That response is then written back by Hyper/Tokio. So the flow is: request → extractors → handler future → await → into_response → response.

No crate code is “async internals” specific; the whole app in `src/` runs on this model (main with `#[tokio::main]`, handlers async, `axum::serve` async).
