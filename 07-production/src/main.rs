use production::run;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("production=info").init();
    let addr: SocketAddr = "127.0.0.1:3006".parse().expect("invalid address");
    println!("Production server on http://{}", addr);
    run::serve(addr).await;
}
