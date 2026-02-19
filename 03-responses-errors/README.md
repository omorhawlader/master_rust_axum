# 03 — Responses & Errors (Day 3)

IntoResponse, custom responses, Result in handlers, centralized error enum, thiserror, rejection vs app error.

## Layout

- **src/** — Crate: AppError + IntoResponse, handler returning Result
- **practice/** — Practice questions
- **project/** — Real-world project (Day 1–3)

## Docs (order)

1. 01_INTO_RESPONSE.md  
2. 02_CUSTOM_RESPONSE_AND_ERRORS.md  
3. 04_REJECTION_VS_APP_ERROR.md  
4. 05_PRODUCTION_ERROR_STRATEGY.md  

## Run

```bash
cargo run
```

Try: `GET /resources/1` (OK), `GET /resources/0` (Validation error 422).
