# Day 5 — Practice

1. What is the difference between a Tower Service and a Layer? How do they combine?
2. If you add `.layer(TimeoutLayer::new(1s)).layer(TraceLayer::new_for_http())`, which runs first on the request path?
3. How would you add CORS so that only `https://example.com` can call your API?
4. Write a middleware (using `axum::middleware::from_fn`) that sets a request extension `X-Request-Id: uuid` and calls `next.run(req).await`. Then write an extractor that reads that extension (or use Extension\<Uuid\>).
5. Where should “auth” live: middleware or extractor? When would you use both?
