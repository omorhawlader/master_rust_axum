//! Server entry: Tokio runtime, bind, axum::serve.

use axum::Router;
use std::net::SocketAddr;

use crate::router_builder;

pub async fn serve(addr: SocketAddr) {
    let app: Router = router_builder::app();
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind");
    axum::serve(listener, app).await.expect("serve");
}
