use std::sync::Arc;

use axum::extract::State;
use tokio::sync::Mutex;

use crate::app_state::{self, AppState};

fn add_file_to_session(State(app_state): State<Arc<Mutex<AppState>>>) {}
