# 1 — Testing Axum Applications

## Unit testing handlers

Handlers are async functions. You can unit-test the logic by calling them with mock extractors: build the types (e.g. Path(42), State(mock_state)) and call the handler. So you test “given this input, does the handler return the right output?” without HTTP.

## Integration testing via ServiceExt

The Router implements Tower’s Service. Use `tower::ServiceExt::oneshot`: build a Request, call `app.oneshot(request).await`, then assert on the Response (status, body). So you test the full stack (routing + handlers) without binding a real port. You can use `Router::with_state(mock_state)` so the app has the state you need for the test.

## Mocking state

For handlers that take State, build a Router with `.with_state(your_mock)`. The mock can be a minimal struct or a fake repo that returns fixed data. No real DB or network.
