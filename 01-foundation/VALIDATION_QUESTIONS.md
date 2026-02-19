# 8️⃣ Practical Reinforcement — Validation Questions

Use these to check that you’ve internalized Day 1. Answer in your own words (no code required unless you want to try in the crate).

---

## Architecture

1. In one sentence: what is Hyper’s job vs Axum’s job?
2. Why does Axum use Tower’s `Service` instead of defining its own “handler” type?
3. List the steps of the request lifecycle from “TCP connection accepted” to “response bytes written,” in order.

---

## Router

4. What does `nest("/api", router)` do to the paths defined in `router`?
5. What is the difference between `merge(router_a).merge(router_b)` and nesting one under a path?
6. Why does `Router::new().route(...).route(...)` take `self` by value and return a new `Router`?

---

## Handlers

7. What must a function return to be used as an Axum handler? (trait name)
8. How does Axum turn your async handler function into something that can handle a request? (one sentence: it becomes a …)
9. If you add `State<AppState>` to a handler but forget `.with_state(...)` on the Router, what happens at compile time?

---

## Async

10. Why does Axum use async handlers instead of blocking ones?
11. What does `#[tokio::main]` do to your `main()`?
12. When your handler awaits something (e.g. a DB call), what happens to the thread it was running on?

---

## Production mental model

13. Where does “shared state” (e.g. a DB pool) live in an Axum app, and how do handlers get it?
14. How is “what can this handler access?” made visible in Axum (compared to Express or FastAPI)?

---

## Completion checklist (Day 1 requirement)

You should be able to:

- [ ] Explain how Axum processes a request end-to-end.
- [ ] Explain how Router relates to Tower Service.
- [ ] Explain how a handler’s return value becomes an HTTP response.
- [ ] Explain how the async runtime (Tokio) is used when a request is handled.

Run the crate from `day_1_plan`: `cargo run`, then try `GET /`, `GET /api/health`, `GET /api/users/42`, `GET /api/custom`, `GET /api/status`, and a non-existent path to see the fallback.
