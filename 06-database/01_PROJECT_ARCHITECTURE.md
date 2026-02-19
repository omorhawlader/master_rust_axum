# 1 — Project Folder Architecture

## main.rs

Entry point: build the app (router + state), bind listener, run `axum::serve`. Keep it short. No business logic.

## Router module

Composes routes and attaches state (and optionally layers). Returns the top-level `Router`. All route definitions live here (or in nested routers from other modules).

## Handler layer

Handlers are async functions that: extract (Path, Query, Json, State), call into the **service layer**, and return a response (or Result). Handlers do not access the database or repo directly. They translate HTTP into service calls and service results into HTTP.

## Service layer

Business logic: “create item,” “get user by id,” “place order.” Services take DTOs or ids and call the **repository**. They can orchestrate multiple repo calls, validate, and return domain results or errors. Handlers call services; services don’t know about HTTP.

## Repository layer

Data access: run queries, return entities or DTOs. In production the repo holds the pool (or gets it from state) and executes SQL. The service layer depends on the repo (trait or concrete type); the repo does not depend on the service. So: handler → service → repo → DB (or in-memory for structure demo).
