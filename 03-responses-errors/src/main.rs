use responses_errors::run;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:3002".parse().expect("invalid address");
    println!("Responses & errors server on http://{}", addr);
    run::serve(addr).await;
}
