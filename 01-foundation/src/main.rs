//! Day 1 — run the app. See ARCHITECTURE.md, ROUTER_DEEP_DIVE.md, etc.

use foundation::run;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:3000".parse().expect("invalid address");
    println!("Foundation server on http://{}", addr);
    run::serve(addr).await;
}
