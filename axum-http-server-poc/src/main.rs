use axum_http_server_poc::{create_app, AppState};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let state = AppState {
        db: Arc::new(Mutex::new(vec![])),
    };

    let app: axum::Router = create_app(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}