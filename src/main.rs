mod router;
mod user;
use std::sync::{Arc, Mutex, RwLock};

use filetravel_backend::app_state::AppState;
use router::create_route;
use tower_http::services::ServeDir;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use user::User;

#[tokio::main]
async fn main() {
    let state = AppState { sessions: vec![] };
    let app = create_route(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn hello_world() -> &'static str {
    "hello world"
}
async fn create_user() -> (StatusCode, Json<User>) {
    let user = User {
        username: "Semion Khrispens".to_string(),
    };
    (StatusCode::OK, Json(user))
}
