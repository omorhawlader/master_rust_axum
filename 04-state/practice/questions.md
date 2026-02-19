# Day 4 — Practice

1. Why is state usually wrapped in `Arc`? What would happen if you used `Rc` instead?
2. What bounds must the state type satisfy (Clone, Send, Sync)? Why each?
3. Add a `Mutex<Vec<String>>` to AppState and a handler that appends a string from the request and returns the current list. Use `State<Arc<AppState>>`.
4. If you use `State<AppState>` but the Router has `.with_state(Arc::new(AppState::default()))`, does it compile? (Check: State’s type in Axum 0.8.)
5. Why is a DB “pool” a good fit for shared state? What shouldn’t you store in state?
