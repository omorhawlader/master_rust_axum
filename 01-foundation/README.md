# 01 — Foundation (Day 1)

Axum ecosystem, router, handlers, async model. Crate under `src/`; docs in this folder.

## Layout

- **src/** — Crate (main, run, router_builder, handlers, lib)
- **practice/** — Practice questions (no solutions; reinforce concepts)
- **project/** — Real-world project for Day 1 only (this crate = minimal API)

## Learning docs (order)

1. ARCHITECTURE.md  
2. SETUP.md  
3. ROUTER_DEEP_DIVE.md  
4. HANDLER_INTERNALS.md  
5. ASYNC_MODEL.md  
6. RUST_CONCEPTS.md  
7. PRODUCTION_MODEL.md  
8. VALIDATION_QUESTIONS.md  

## Run

```bash
cargo run
```

Try: `GET /`, `GET /api/health`, `GET /api/users/42`, `GET /api/custom`, `GET /api/status`, then a non-existent path (fallback).
