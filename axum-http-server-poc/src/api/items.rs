use axum::{
    extract::State,
    routing::{get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Vec<String>>>,
}

#[derive(Deserialize)]
pub struct CreateItem {
    pub value: String,
}

#[derive(Serialize)]
pub struct ItemsResponse {
    pub items: Vec<String>,
}

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/items", get(get_items).post(create_item))
        .with_state(state)
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
