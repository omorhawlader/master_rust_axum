# Day 3 — Practice

1. What trait must a handler’s return type implement? What about the `E` in `Result<T, E>`?
2. Implement `IntoResponse` for a struct `Message { code: u16, text: String }` that sets status from `code` and body as plain text.
3. Why use a centralized `AppError` enum instead of returning different error types from each handler?
4. Add a `NotFound` variant to `AppError` and return it when a resource id doesn’t exist. Map it to 404 and a JSON body.
5. What is the difference between “rejection” and “application error”? Who produces each?
6. How would you make extractor rejections (e.g. invalid JSON) return the same JSON shape as your AppError?
