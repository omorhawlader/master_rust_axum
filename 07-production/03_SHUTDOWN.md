# 3 — Error monitoring & 4 — Graceful shutdown

Log errors and context server-side. For graceful shutdown, use a shutdown signal (e.g. ctrl_c) and pass it to axum::serve so the server stops accepting new connections and drains in-flight requests.
