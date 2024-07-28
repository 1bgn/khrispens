use std::sync::{Arc, Mutex};

use axum::{
    routing::{get, post},
    Router,
};
use filetravel_backend::{app_state::AppState, routes::create_session::create_session};
// use tokio::sync::Mutex;
use tower_http::services::ServeDir;

pub fn create_route(app_state: AppState) -> Router {
    Router::new()
        .nest_service("/", ServeDir::new("../filetravel_frontend/build/web"))
        .route("/session-bundle", post(create_session))
        .with_state(Arc::new(Mutex::new(app_state)))
}
