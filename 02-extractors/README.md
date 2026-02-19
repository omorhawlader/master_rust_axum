# 02 — Extractors (Day 2)

Type-driven request parsing: Path, Query, Json, State, RequestParts vs Request, FromRequest, rejections.

## Layout

- **src/** — Crate: Path, Query, Json handlers; run, router
- **practice/** — Practice questions
- **project/** — Real-world project (Day 1 + Day 2)

## Docs (order)

1. 01_EXTRACTOR_ARCHITECTURE.md  
2. 02_BUILTIN_EXTRACTORS.md  
3. 03_ORDER_AND_FROM_REQUEST.md  
4. 05_REJECTION_AND_PRODUCTION.md  

## Run

```bash
cargo run
```

Try: `GET /items/42`, `GET /search?q=foo&limit=10`, `POST /items` with JSON body `{"name":"x","count":5}`.
