# 1 — Tower Architecture Foundation

## Service trait

Tower’s `Service` trait is: given a request, return a future that resolves to a response (or error). So every “layer” of the stack (Router, handler-backed service, middleware) is a `Service`. The server calls `service.call(request).await` and gets a response. That’s the only interface.

## Request → Future → Response model

A Service doesn’t do work synchronously. It returns a Future. So the runtime can run many requests concurrently: each call returns a future that is polled until the response is ready. Middleware wraps the inner service and can run code before calling the inner service (e.g. log request) and after the future completes (e.g. log response, set headers).
