# Day 2 — Practice

1. Why must a body extractor (e.g. `Json<T>`) be the last argument in a handler? What is RequestParts vs Request?
2. Add a handler that takes `Query<HashMap<String, String>>` and returns the map as JSON. Which route path do you use?
3. What happens at runtime if you `POST /items` with an invalid JSON body (e.g. `{invalid}`)? Who produces the response — your handler or Axum?
4. If you use `Path<(u32, u32)>`, how many dynamic segments must the route have? What if you use a struct with two fields?
5. Implement a custom extractor that reads a header `X-Request-Id` and returns `String`, or rejects with 400 if missing. (Use FromRequestParts.)
6. What is the difference between “extractor rejection” and “application error” (e.g. “user not found”)? Where does each come from?
