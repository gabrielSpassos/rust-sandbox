use http_segment_tree::{create_routes, AppState};
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let state = AppState {
        tree: Arc::new(Mutex::new(None)),
    };

    let app: axum::Router = create_routes(state)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}