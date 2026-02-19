# 3 — Built-in Middleware & 4 — Custom Middleware

## Built-in (tower-http)

- **Logging / Trace:** `TraceLayer` logs request and response (and duration). Use with the `tracing` crate.
- **CORS:** `CorsLayer` sets CORS headers. Configure origins, methods, headers as needed.
- **Timeout:** `TimeoutLayer` fails the request if the inner service doesn’t respond in time (e.g. 504 or a timeout error response).
- **Compression:** `CompressionLayer` compresses response bodies (gzip, etc.) when the client supports it.

## Writing custom middleware

Implement Tower’s `Layer` and `Service`. The Layer’s job is to take the inner `S: Service` and return a new Service that wraps `S`. In the wrapper’s `call`, you can: inspect or modify the request, call `inner.call(req).await`, then inspect or modify the response. You can also short-circuit (e.g. return 401 without calling the inner service). Axum’s `middleware::from_fn` lets you write an async function that takes `Request` and `next: Next` and returns a `Response`, which is a convenient way to build a simple middleware without implementing Service by hand.

## Middleware vs extractor

- **Middleware:** Runs for every request (or every request under that route). Good for logging, timeout, CORS, auth that rejects before the handler. It can modify the request (e.g. add an extension) or reject with a response.
- **Extractor:** Runs only when a handler that uses that extractor is hit. Good for “give me the current user” or “give me the body.” So: auth that **checks** and **injects** the user is often “middleware that sets an extension” + “extractor that reads the extension.” The middleware does the work; the extractor is the typed way to get the result in the handler.
