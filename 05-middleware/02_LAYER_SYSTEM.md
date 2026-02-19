# 2 — Layer System

## What is a Layer?

A **Layer** is a type that wraps a `Service` and produces another `Service`. So you get a stack: `Layer A` wraps `Layer B` wraps `Layer C` wraps your Router. When a request comes in, it goes through A → B → C → Router → handler; the response goes back C → B → A. Typical use: logging, timeout, compression, CORS, auth.

## Middleware stacking behavior

Layers are applied in order. `.layer(A).layer(B)` means: the outer layer is A, then B, then the Router. So the request hits A first, then B, then the router. Response flows back through B then A. Order matters: e.g. you usually want logging (see full request/response) outside timeout (so timeouts are logged).

## Order importance

If you add timeout inside logging, the timeout only applies to the inner service; logging still sees the full latency. If you add timeout outside logging, the logger might see a timeout error. Think about what you want to observe and what you want to enforce (e.g. auth before handler, timeout around handler, CORS around everything).
