use std::sync::Arc;
use std::time::Duration;
use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use tokio::sync::Mutex;
use uuid::{uuid, Uuid};

use crate::{app_state::AppState, domain::models::session_bundle::SessionBundle};

// use tokio::sync::Mutex;


pub async fn create_session(
    State( app_state): State<AppState>,
) -> (StatusCode, Json<SessionBundle>) {
    // let mut state_clone = Arc::clone(&mut app_state.sessions);
    //         tokio::time::sleep(Duration::from_secs(10)).await;

    let mut guard =app_state.sessions;
    let len = guard.len();
    let session_bundle = SessionBundle::new(len);
    {
        guard.insert(len,session_bundle.clone());
    }


    (StatusCode::OK, Json(session_bundle))
}

