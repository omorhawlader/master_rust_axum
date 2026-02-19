# 05 — Middleware (Day 5)

Tower Service, Layer, TraceLayer, TimeoutLayer, order of layers.

## Layout

- **src/** — Crate: TraceLayer (logging), TimeoutLayer (1s); /slow exceeds it
- **practice/** — Practice questions
- **project/** — Real-world project (Day 1–5)

## Docs (order)

1. 01_TOWER_ARCHITECTURE.md  
2. 02_LAYER_SYSTEM.md  
3. 03_BUILTIN_AND_CUSTOM.md  

## Run

```bash
RUST_LOG=middleware=info cargo run
```

Try: `GET /` (OK), `GET /slow` (times out after 1s).
