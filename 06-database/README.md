# 06 — Database & Structure (Day 6)

Project layout (handler → service → repo), pool concept, DTO, validation.

## Layout

- **src/** — main, run, router_builder, handlers, service, repo, dto
- **practice/** — Practice questions
- **project/** — Real-world project (Day 1–6)

## Docs (order)

1. 01_PROJECT_ARCHITECTURE.md  
2. 02_POOL_AND_TRANSACTIONS.md  
3. 04_DTO_AND_VALIDATION.md  

## Run

```bash
cargo run
```

Try: `POST /items` with `{"name":"foo"}`, then `GET /items/1`.
