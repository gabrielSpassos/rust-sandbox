use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::json;
use tower::ServiceExt;

use axum_http_server_poc::{create_app, AppState};
use std::sync::{Arc, Mutex};

#[tokio::test]
async fn test_get_empty_items() {
    let state = AppState {
        db: Arc::new(Mutex::new(vec![])),
    };

    let app = create_app(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/items")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json, json!({ "items": [] }));
}

#[tokio::test]
async fn test_post_and_get_items() {
    let state = AppState {
        db: Arc::new(Mutex::new(vec![])),
    };

    let app = create_app(state.clone());

    // POST
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/items")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"value":"gabriel"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // GET
    let response = app
        .oneshot(
            Request::builder()
                .uri("/items")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json, json!({ "items": ["gabriel"] }));
}
