# 2 — Pool & 3 — Transactions

## Pool

An async connection pool is `Clone` and `Send + Sync`. Create it at startup and put it in state (e.g. `Arc<Pool<Postgres>>`). The repository gets the pool (via State or in its constructor). Per request: acquire a connection, run queries, release. So the pool is shared; connections are short-lived.

## Injecting pool

Put the pool in state or put the repo (which holds the pool) in state. Common: `State<Arc<ItemRepo>>` with `ItemRepo { pool }`. Handler has State(repo), calls repo.get_item(id).await; repo uses self.pool.acquire() internally.

## Transactions

Start a transaction from the pool (e.g. pool.begin().await), pass it (or a repo using it) through the service, commit at the end or rollback on error. Transaction is request-scoped; don’t store it in state. On Err, drop or rollback so the DB rolls back.
