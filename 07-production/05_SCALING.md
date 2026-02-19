# 5 — Scaling & 6 — Performance

Stateless design allows horizontal scaling: run many instances behind a load balancer. Use a reverse proxy for TLS and distribution. Avoid blocking the runtime; minimize cloning; use async libraries.
