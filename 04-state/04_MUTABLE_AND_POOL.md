# 4 — Mutable Shared State & 5 — Database Pool Concept

## Mutex / RwLock

For shared mutable data, wrap it in `std::sync::Mutex` or `RwLock` (or `tokio::sync::Mutex` if you need to hold the lock across an `.await`). The state type might be `Arc<AppState>` where `AppState { cache: Mutex<HashMap<...>> }`. Handlers clone the Arc and lock when they need to read or write. Prefer RwLock when many readers and few writers.

## Tokio sync primitives

`tokio::sync::Mutex` is async-aware: you `.lock().await` and can hold the guard across an await. Use it when you need to hold a lock while doing I/O. `std::sync::Mutex` is for short, non-async critical sections.

## Pool as shared state

A DB connection pool (e.g. deadpool, bb8, sqlx’s pool) is typically `Clone` and `Send + Sync`. You put it in state: `Arc<Pool<DB>>` or just `Pool<DB>` if the pool type is already cheap to clone. Handlers take `State<Arc<Pool<...>>>` and call `pool.get().await` to get a connection for the request. The pool manages lifetime and concurrency; you don’t put raw connections in state.

## Lifetime safety

The state lives for the whole server run. Handles (e.g. pool, client) inside it must be valid for `'static` or the runtime’s lifetime. Pools and clients are usually designed that way. Don’t store request-scoped data in state; use extractors or pass it through the handler.
