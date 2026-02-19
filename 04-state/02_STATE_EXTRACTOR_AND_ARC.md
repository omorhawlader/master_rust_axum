# 2 — State\<T\> Extractor & 3 — Arc (Backend Context)

## Router::with_state

You call `Router::new().route(...).with_state(state)`. The state type becomes part of the Router’s type. Every handler that uses `State<S>` must use the same `S` (or a type that matches what you passed). If you forget `.with_state(...)`, you get a compile error when you try to use `State` in a handler.

## Clone bounds

The state type must be `Clone + Send + Sync`. Axum may clone the state (or a reference to it) when dispatching to a handler. So typically you wrap your state in `Arc<S>`: `Arc` is cheap to clone (it’s a pointer), and the inner `S` can be shared across many requests and threads. The state type you give to `with_state` is then `Arc<AppState>` (or similar).

## Type-safe injection

Only handlers on that Router can see that state. The type is fixed at compile time. So “where does this value come from?” is always clear: from `.with_state(...)`.

## Why state must be Send + Sync

Handlers run on the async runtime (Tokio), which may move tasks between threads. So any value that crosses an `.await` or is shared across tasks must be `Send`. If the state is shared across concurrent requests (which it is), it must also be `Sync` (safe to share references across threads). `Arc<T>` is Send + Sync when T is Send + Sync. So your `AppState` (or whatever is inside the Arc) must be Send + Sync: avoid Rc, non-Send locks, or raw pointers that aren’t thread-safe.

## Immutable vs mutable state

Often state is “immutable” (e.g. config, pool). If you need mutability, use interior mutability: `Mutex`, `RwLock`, or atomic types. Then the state type is e.g. `Arc<AppState>` where `AppState` contains a `Mutex<HashMap<...>>` or `AtomicU64`. That way the State extractor still gives you a clone of the Arc (cheap), and you lock or atomically update inside the handler.
