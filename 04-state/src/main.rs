use state::run;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:3003".parse().expect("invalid address");
    println!("State server on http://{}", addr);
    run::serve(addr).await;
}
