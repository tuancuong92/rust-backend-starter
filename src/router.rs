use crate::app_state;
use app_state::AppState;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use axum::response::IntoResponse;
use tracing::trace;

pub async fn handler_not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Nothing to see here")
}

pub async fn root_handler() -> String {
    "Hello, World".to_string()
}

pub fn foo_router() -> Router<AppState> {
    Router::new()
        .route("/:id", get(get_foo))
        .route("/", post(post_foo))
}

async fn get_foo(
    Path(foo_id): Path<u32>,
    Query(params): Query<HashMap<String, String>>,
) -> (StatusCode, String) {
    trace!("Params: {:?}", params);
    let message = format!("OK {}", foo_id);
    (StatusCode::OK, message)
}

async fn post_foo(Json(payload): Json<CreateUser>) -> (StatusCode, Json<Value>) {
    let user = User {
        id: 1234,
        username: payload.username,
    };

    (
        StatusCode::CREATED,
        Json(json!({"status": true, "data": user})),
    )
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
