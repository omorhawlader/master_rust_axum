# 2 — Custom Response Types & 3 — Error Handling Architecture

## Implementing IntoResponse manually

When you need full control (headers, specific body format), define a struct and implement `IntoResponse`: in `into_response(self)` build an `http::Response` (or use helpers like `(StatusCode::OK, Json(...)).into_response()` and then add headers). Custom types are useful for a consistent API shape (e.g. always `{ "data": ... }` or `{ "error": ... }`).

## Result in handler

Handlers can return `Result<impl IntoResponse, E>` where `E: IntoResponse`. On `Ok`, the inner value is turned into a response; on `Err`, the error is. So “application errors” (e.g. not found, validation failure) are values that implement `IntoResponse` (often with a 4xx status and a JSON body). The handler uses `?` to propagate errors; the last `?` or the final `Err` becomes the response.

## Centralized error enum

One enum (e.g. `AppError`) with variants for NotFound, Validation, Unauthorized, etc. Each variant can carry context (e.g. `NotFound(String)`). Implement `IntoResponse` for the enum once: map each variant to a status code and body. Then handlers return `Result<T, AppError>` and errors are consistent across the API.

## thiserror integration

`thiserror` lets you derive `Error` and `Display` for your error enum. You get a clear message and source chain without boilerplate. The enum still needs an explicit `IntoResponse` impl so Axum can turn it into an HTTP response.
