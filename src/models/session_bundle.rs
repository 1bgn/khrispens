use chrono::Utc;
use serde::Serialize;

use super::session_file::SessionFile;

#[derive(Clone, Serialize)]
pub struct SessionBundle {
    pub session_number: String,
    pub id: i64,
    pub create_at: String,
    pub update_at: String,
    pub files: Vec<SessionFile>,
}
