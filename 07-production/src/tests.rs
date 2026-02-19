//! Example: call the Router as a Service (integration-style test).

use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

use crate::router_builder;

#[tokio::test]
async fn health_returns_200() {
    let app = router_builder::app();
    let req = Request::builder().uri("/health").body(Body::empty()).unwrap();
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}
