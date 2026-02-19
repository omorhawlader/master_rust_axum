# 4 — DTO vs Domain & 5 — Validation

## DTO vs domain

DTO = shape at the API boundary (request/response). Domain = internal representation. Separate them so the API stays stable and you don’t expose internals. Repo may return domain types; service maps to DTO for the response.

## Validation

Extractor: “does the request shape match?” (rejection if not). Business rules: “is this allowed?” in the service layer; return app error (e.g. Validation, NotFound).
