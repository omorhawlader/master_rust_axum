# Day 6 real-world project (topics: Day 1–6)

**Scope:** Foundation through database structure: handler → service → repo, DTOs, in-memory “store.”

## Project: Items API with layered structure

The parent crate: POST create item, GET item by id. Handler extracts and calls service; service uses repo; repo is in-memory (no real DB). State holds the service (which holds the repo).

- **POST /items** — JSON body → service.create_item → repo.create  
- **GET /items/{id}** — path → service.get_item → repo.get  

## What you practiced

- Clean layers: handler → service → repo; DTOs for request/response; state = Arc<Service>.

## Extend (optional)

- Replace in-memory repo with a real pool (e.g. sqlx) and one table; keep the same handler/service/repo interface.
- Add validation in the service (e.g. name length) and return a structured error.
