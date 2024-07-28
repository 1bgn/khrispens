use serde::Serialize;

use crate::models::session_bundle::SessionBundle;

#[derive(Clone, Serialize)]
pub struct AppState {
    pub sessions: Vec<SessionBundle>,
}
