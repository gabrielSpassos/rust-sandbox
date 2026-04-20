use axum::{
    extract::State,
    routing::{get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Vec<String>>>,
}

#[derive(Deserialize)]
struct CreateItem {
    value: String,
}

#[derive(Serialize)]
struct ItemsResponse {
    items: Vec<String>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        db: Arc::new(Mutex::new(vec![])),
    };

    let app = Router::new()
        .route("/items", get(get_items).post(create_item))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn get_items(State(state): State<AppState>) -> Json<ItemsResponse> {
    let db = state.db.lock().unwrap();

    Json(ItemsResponse {
        items: db.clone(),
    })
}

async fn create_item(
    State(state): State<AppState>,
    Json(payload): Json<CreateItem>,
) -> Json<ItemsResponse> {
    let mut db = state.db.lock().unwrap();

    db.push(payload.value);

    Json(ItemsResponse {
        items: db.clone(),
    })
}