use axum::{extract::State, http::StatusCode, Json};

use crate::{app_state::AppState, domain::models::session_bundle::SessionBundle};

// use tokio::sync::Mutex;


pub async fn create_session_route(
    State(mut app_state): State<AppState>,
) -> (StatusCode, Json<SessionBundle>) {
    // let mut state_clone = Arc::clone(&mut app_state.sessions);
    //         tokio::time::sleep(Duration::from_secs(10)).await;

    let len = app_state.sessions.len();

    let session_bundle = SessionBundle::new(len);
    let mut map = &app_state.sessions;
        map.insert(len,session_bundle.clone()) ;


    (StatusCode::OK, Json(session_bundle))
}

