# 5 — Production Error Strategy

- **Error format standardization:** Use one JSON shape for errors (e.g. `{ "error": "message" }` or `{ "code": "...", "message": "..." }`). Map both rejections and app errors to that shape so clients can parse consistently.
- **Logging vs client message:** Log full details (and error chain) server-side; return a short, safe message to the client. Don’t leak internals in the response.
- **JSON error structure:** Often a single field for a human-readable message plus an optional code or type field for clients to branch on. Keep it minimal and stable.
