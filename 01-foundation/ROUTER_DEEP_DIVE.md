# 3️⃣ Router Deep Dive

## 3.1 Router as Service

- **How Router implements Tower Service**  
  `Router` implements `Service<Request<Body>, Response = Response>`. When you call `axum::serve(listener, app)`, Hyper (or the integration layer) calls `router.call(request).await` for each request. The Router’s job is to match path and method and delegate to an inner service (another Router, or a handler-backed service).

- **Why everything is a Service**  
  So the “app” is a single Service. Nested routers are inner services; a route’s handler is turned into a Service (MethodRouter wraps handlers per HTTP method). Middleware are Tower `Layer`s that wrap a Service. So one abstraction (Service + Layer) covers app, routes, and middleware—no separate “middleware type” in Axum.

---

## 3.2 Route Registration System

- **route(path, method_router)**  
  Registers a path and a `MethodRouter` (get/post/put/patch/delete or a chain of them). The path can be static (`"/health"`) or have segments (`"/users/{id}"`). One route = one path; different methods are handled by the same `MethodRouter` on that path.

- **merge(other_router)**  
  Combines two routers at the “same level”: both trees are tried. Useful to combine independently built routers (e.g. from different crates or modules) without a common prefix.

- **nest(prefix, sub_router)**  
  Mounts `sub_router` under `prefix`. Every route in `sub_router` is then served under that prefix (e.g. `nest("/api", api_routes)` makes `/api/health` etc.). The inner router doesn’t know about the prefix; Axum strips the prefix when matching inside the nested router.

- **fallback(handler)**  
  Registers a handler that runs when no route matches. Typically returns 404 or a catch-all response. Without fallback, Axum returns a default 404.

In this crate, `router_builder::app()` uses `route`, `nest`, and `fallback`; `app_with_merge()` and `method_router_demo()` show merge and MethodRouter chaining.

---

## 3.3 MethodRouter

- **get(handler)**, **post(handler)**, **put(handler)**, **patch(handler)**, **delete(handler)**  
  Each creates a `MethodRouter` that handles that HTTP method. You can chain: `.route("/r", get(h1).post(h2).delete(h3))`. Only one handler per method per path; for the same path different methods can have different handlers.

- **Chaining behavior**  
  Chaining is “add another method to the same path.” The result is still one `MethodRouter` that dispatches by method. If a request matches the path but not the method, you get 405 Method Not Allowed (unless you add a fallback for that path).

---

## 3.4 Immutability & Builder Pattern

- **Why Router consumes self**  
  Methods like `route`, `nest`, `merge`, `layer` take `self` by value and return a new `Router`. So you chain: `Router::new().route(...).nest(...)`. This is a functional builder: each step produces a new value; you don’t mutate a single router. That makes it easy to share and compose: you can pass a router to a function that adds more routes and returns a new router.

- **Functional builder pattern**  
  You build one final Router value and pass it to `axum::serve`. No global mutable router; the “app” is just that value.

---

## 3.5 Route Resolution Order

- **Static vs dynamic**  
  More specific paths should be registered before more general ones if they overlap (e.g. `/users/me` before `/users/:id`). Axum matches in a defined order (typically more specific first when using a radix or similar structure). Dynamic segments (`{id}`) match one path segment.

- **Overlapping route behavior**  
  If two routes could match the same path, the one that is matched first (by the router’s internal order) wins. So design your paths and nesting so that the intended route is unambiguous (e.g. don’t have both `/users/{id}` and `/users/me` in an order where the wrong one wins). Fallback only runs when no route matches at all.

Code: see `src/router_builder.rs` for `app()`, `app_with_merge()`, and `method_router_demo()`.
