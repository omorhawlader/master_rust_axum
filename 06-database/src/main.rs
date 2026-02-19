use database::run;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:3005".parse().expect("invalid address");
    println!("Database-structure server on http://{}", addr);
    run::serve(addr).await;
}
