# 1 — IntoResponse Deep Dive

## Trait architecture

`IntoResponse` is the trait that turns a handler’s return value into an HTTP response. One method: `fn into_response(self) -> Response`. Axum calls it on whatever your handler returns (or on the `Ok`/`Err` variant when you return `Result<T, E>` and both implement `IntoResponse`).

## Automatic implementations

Many types implement it out of the box: `&'static str`, `String`, `StatusCode`, `(StatusCode, T)` where `T: IntoResponse`, `Json<T>`, `Response`, `()`, etc. So you can return plain strings, status codes, or tuples (status, body) without implementing the trait yourself.

## Tuple responses

A common pattern is `(StatusCode, Json<Body>)` or `(StatusCode, &str)`. The tuple’s first element sets the status; the second is converted to the response body. Headers can be added with a three-element tuple or a custom type.

## StatusCode integration

`StatusCode` by itself is a valid response (body empty). Combined with a body in a tuple, it sets the status. So “success” vs “error” is often expressed by returning different status codes (and optionally different body types).
