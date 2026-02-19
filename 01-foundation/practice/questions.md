# Day 1 — Practice (no solutions; reinforce concepts)

Answer in your own words or by writing minimal code in the crate.

---

## Router & lifecycle

1. Draw (or list) the request lifecycle from TCP accept to response write. Where does Tower appear?
2. Why can’t you call `.route(...)` twice on the same `Router` value without reassigning? (e.g. `let r = Router::new().route(...); r.route(...)` fails — why?)
3. When would you use `nest` vs `merge`? Give one example each.

---

## Handlers & responses

4. What trait must your handler’s return type implement? Name one type that implements it and one that doesn’t.
5. Add a handler that returns only `StatusCode::CREATED`. Register it at `POST /api/items`. What do you see when you call it?
6. Why is the handler function always `async` in Axum?

---

## Concepts

7. In one sentence: what is Hyper’s job vs Axum’s job?
8. What does `#[tokio::main]` do to your `main()`?
9. If you add `State<AppState>` to a handler but don’t call `.with_state(...)` on the Router, what happens?

---

Use **VALIDATION_QUESTIONS.md** for the formal completion checklist.
