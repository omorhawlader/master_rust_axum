# 5 — Rejection System & 7 — Production Scenarios

## Rejection system

- **Default rejections:** Built-in extractors return specific rejection types (e.g. `JsonRejection`, `PathRejection`, `QueryRejection`). Axum maps these to HTTP responses (status code and sometimes a body).
- **Custom rejection types:** For custom extractors you define a rejection enum that implements the traits Axum expects so it can turn it into a response. You can use `#[derive(AxumRejection)]` or implement the mapping by hand depending on the Axum version.
- **Mapping rejections to responses:** You can add a fallback or layer that converts certain rejections into a standard JSON error shape so clients always get a consistent format.

## Production scenarios

- **Validating input:** Use extractors for “shape” (e.g. JSON deserializes to `CreateUser`). For business rules (e.g. “email must be unique”), validate in the handler or in a service layer and return an application error (Result or custom type that implements IntoResponse).
- **Auth extractor preview:** A common pattern is a custom extractor that reads a header or cookie, validates the token, and returns the current user (or a rejection if unauthenticated). That’s a FromRequestParts extractor that runs before body extractors.
- **DTO validation strategy:** DTOs (data transfer objects) for request bodies often use serde for deserialization and a validation crate (e.g. validator) or manual checks in the handler. Failed validation can be returned as a 400/422 with a structured error body.
