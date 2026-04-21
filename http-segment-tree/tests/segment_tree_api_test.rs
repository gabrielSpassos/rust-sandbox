use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

use http_segment_tree::{create_routes, AppState};

use std::sync::{Arc, Mutex};

fn setup_app() -> axum::Router {
    let state = AppState {
        tree: Arc::new(Mutex::new(None)),
    };

    create_routes(state)
}

#[tokio::test]
async fn should_create_and_get_segment_tree() {
    let app = setup_app();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/segment-tree")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"input":[5,8,7,2,10,2,2]}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/segment-tree")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["array"], json!([5, 8, 7, 2, 10, 2, 2]));
    assert!(json["tree"].as_array().unwrap().len() > 0);
    assert_eq!(json["tree"], json!([36, 22, 14, 13, 9, 12, 2, 5, 8, 7, 2, 10, 2]));
}

#[tokio::test]
async fn should_update_segment_tree() {
    let app = setup_app();

    app.clone()
        .oneshot(
            Request::builder()
                .uri("/segment-tree")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"input":[5,8,7,2,10,2,2]}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/segment-tree")
                .method("PUT")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"idx":3,"value":6}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["array"], json!([5, 8, 7, 6, 10, 2, 2]));
    assert!(json["tree"].as_array().unwrap().len() > 0);
    assert_eq!(json["tree"], json!([40, 26, 14, 13, 13, 12, 2, 5, 8, 7, 6, 10, 2]));
}

#[tokio::test]
async fn should_query_segment_tree() {
    let app = setup_app();

    app.clone()
        .oneshot(
            Request::builder()
                .uri("/segment-tree")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"input":[5,8,7,2,10,2,2]}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/segment-tree/query?left=0&right=6")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json, json!({ "result": 36 }));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/segment-tree/query?left=0&right=3")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json, json!({ "result": 22 }));
}