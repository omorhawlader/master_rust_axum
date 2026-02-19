use middleware::run;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env().add_directive("middleware=info".parse().unwrap()))
        .init();
    let addr: SocketAddr = "127.0.0.1:3004".parse().expect("invalid address");
    println!("Middleware server on http://{}", addr);
    run::serve(addr).await;
}
