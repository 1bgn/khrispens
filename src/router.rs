use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use filetravel_backend::{
    app_state::AppState,
    routes::{
        add_file_to_session::add_file_to_session, create_session::create_session,
        get_session_bundle_by_number::get_session_bundle_by_number,
        upload_file_by_id::upload_file_by_id,
    },
};
use tokio::sync::Mutex;
// use tokio::sync::Mutex;
use tower_http::services::ServeDir;

pub fn create_route(app_state: AppState) -> Router {
    Router::new()
        .nest_service("/", ServeDir::new("../filetravel_frontend/build/web"))
        .route("/session-bundle", get(get_session_bundle_by_number))
        .route("/session-bundle", post(create_session))
        .route("/add-file-to-session", post(add_file_to_session))
        .route("/upload-file-by-id", post(upload_file_by_id))
        .with_state(Arc::new(Mutex::new(app_state)))
}
