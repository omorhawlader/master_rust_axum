# Axum Learn Plans (Days 1–7)

Structured learning for **Axum** (Rust): one folder per day, with crate code, deep docs, practice questions, and a cumulative real-world project.

## Folder layout (beauty names)

| Folder | Day | Focus |
|--------|-----|--------|
| **01-foundation** | 1 | Ecosystem, Router, handlers, async |
| **02-extractors** | 2 | Path, Query, Json, FromRequest, rejections |
| **03-responses-errors** | 3 | IntoResponse, Result, AppError, thiserror |
| **04-state** | 4 | State\<T\>, Arc, shared state, pool concept |
| **05-middleware** | 5 | Tower Service/Layer, trace, timeout |
| **06-database** | 6 | Handler → service → repo, DTO, validation |
| **07-production** | 7 | Testing, tracing, shutdown, scaling |

Each folder has:

- **src/** — Crate (minimal code for that day)
- **practice/** — Questions (no solutions)
- **project/** — Real-world project using only topics up to that day
- **README.md** + numbered docs (e.g. 01_*.md)

## How to use

1. Work through **01-foundation** first (docs + run the crate + practice).
2. Do the **project** in that folder (concepts for that day only).
3. Repeat for 02–07. Each project uses all previous days’ topics.

## Other files

- **day_1.md** — Day 1 learning plan (source for 01-foundation).
- **day_2-7.md** — Day 2–7 learning plans (source for 02–07).
- **project_base_learning.md** — Base project reference.

All teaching targets **current stable Axum (0.8.x)** and modern patterns.
