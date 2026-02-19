# 2 — Built-in Extractors

## 2.1 Path\<T\>

- **Dynamic segment parsing:** The route path has segments like `{id}` or `{id}/logs/{log_id}`. Axum parses those segments and deserializes them into `T`. For a single segment you use `Path<u32>` or `Path<String>`; for multiple, `Path<(u32, String)>` or a struct that implements `Deserialize`.
- **Deserialization rules:** Same as serde: numeric segments must be valid numbers for `u32`/`i64` etc.; otherwise extraction fails and the request is rejected (e.g. 404 or 400 depending on implementation).
- **Multiple params:** Use a tuple `Path<(A, B)>` or a struct `Path<Params>` with `#[serde(rename = "...")]` if segment names differ from field names.

## 2.2 Query\<T\>

- **Query string mapping:** The `?key=value&...` part of the URL is parsed and deserialized into `T`. Typically a struct with `Option` fields for optional params.
- **Optional params:** Use `Option<U>` for optional query parameters. Required fields in `T` must appear in the query string or extraction fails.
- **Validation strategy:** For simple validation, use types (e.g. `u32` for a numeric limit). For more complex rules, extract into a struct and validate in the handler (or in a custom extractor that returns a rejection on failure).

## 2.3 Json\<T\>

- **Body extraction:** The request body is read and deserialized as JSON into `T`. Requires `T: DeserializeOwned` (or equivalent).
- **Content-Type enforcement:** Axum’s `Json` extractor typically expects `Content-Type: application/json`. Mismatch or invalid JSON leads to rejection.
- **Rejection behavior:** Invalid JSON or type mismatch produces a rejection (often 422 or 400). The handler is not called; Axum turns the rejection into an HTTP response.

## 2.4 State\<T\>

- **Overview:** Injects the application state you gave to the Router via `.with_state(state)`. The handler receives a clone of the state (or a reference, depending on how Axum implements it; in 0.8 it’s typically a clone). So you get shared, read-only (or interior-mutable) state without globals.
- **Clone requirement:** State must be `Clone` (and `Send + Sync` for the async runtime). Usually you wrap your state in `Arc<S>` so cloning is cheap and the same data is shared across handlers.

## 2.5 HeaderMap / TypedHeader

- **HeaderMap:** Gives you the raw headers. Useful when you need ad-hoc header access.
- **TypedHeader:** Extract a single header as a typed value (e.g. `TypedHeader<ContentType>`) when the header type is in the tower-http or axum ecosystem. Requires the right feature and dependency. Cleaner than parsing from `HeaderMap` by hand when you only need one header.
