use axum::{
    extract::{Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::domain::segment_tree::SegmentTree;

#[derive(Clone)]
pub struct AppState {
    pub tree: Arc<Mutex<Option<SegmentTree>>>,
}

// ---------- DTOs ----------

#[derive(Deserialize)]
pub struct CreateRequest {
    pub input: Vec<i32>,
}

#[derive(Deserialize)]
pub struct UpdateRequest {
    pub idx: usize,
    pub value: i32,
}

#[derive(Deserialize)]
pub struct QueryParams {
    pub left: usize,
    pub right: usize,
}

#[derive(Serialize)]
pub struct TreeResponse {
    pub array: Vec<i32>,
    pub tree: Vec<i32>,
}

#[derive(Serialize)]
pub struct QueryResponse {
    pub result: i32,
}

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/segment-tree", post(create_tree).get(get_tree).put(update_tree))
        .route("/segment-tree/query", get(query_tree))
        .with_state(state)
}

async fn create_tree(
    State(state): State<AppState>,
    Json(payload): Json<CreateRequest>,
) -> Json<TreeResponse> {
    let mut guard = state.tree.lock().unwrap();

    let st = SegmentTree::new(payload.input);
    let response = build_tree_response(&st);

    *guard = Some(st);

    Json(response)
}

async fn get_tree(State(state): State<AppState>) -> Json<TreeResponse> {
    let guard = state.tree.lock().unwrap();

    let st = guard.as_ref().expect("tree not initialized");

    Json(build_tree_response(st))
}

async fn update_tree(
    State(state): State<AppState>,
    Json(payload): Json<UpdateRequest>,
) -> Json<TreeResponse> {
    let mut guard = state.tree.lock().unwrap();

    let st = guard.as_mut().expect("tree not initialized");

    st.update(payload.idx, payload.value);

    Json(build_tree_response(st))
}

async fn query_tree(
    State(state): State<AppState>,
    Query(params): Query<QueryParams>,
) -> Json<QueryResponse> {
    let guard = state.tree.lock().unwrap();

    let st = guard.as_ref().expect("tree not initialized");

    let result = st.query(params.left, params.right);

    Json(QueryResponse { result })
}

fn build_tree_response(st: &SegmentTree) -> TreeResponse {
    let raw_tree = st.get_tree();
    let logical_size = 2 * st.get_array().len() - 1;

    let tree: Vec<i32> = raw_tree
        .iter()
        .take(logical_size)
        .cloned()
        .collect();

    TreeResponse {
        array: st.get_array().clone(),
        tree,
    }
}