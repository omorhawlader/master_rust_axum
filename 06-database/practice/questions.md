# Day 6 — Practice

1. Why separate handler, service, and repository? What does each layer depend on?
2. Where would you put “user must be 18+” validation: extractor, handler, or service?
3. If the repo holds `Arc<Pool<Postgres>>`, who creates the pool and who creates the repo? When?
4. What is the difference between a DTO and a domain model? Why use DTOs at the API boundary?
5. How would you run two repo operations in a single DB transaction? Who starts and commits the transaction?
